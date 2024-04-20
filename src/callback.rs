pub use gensym;
use linkme::distributed_slice;
pub use paste;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub struct AnyAction(pub Box<dyn std::any::Any>);

#[distributed_slice]
pub static CALLBACKS: [(&str, fn(&str, Box<dyn std::any::Any>) -> AnyAction)];

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Callback<T> {
    #[serde(skip, default = "default_fun_ptr")]
    fun_ptr: Option<fn(T) -> AnyAction>,
    pub fun_name: Cow<'static, str>,
}

fn default_fun_ptr<T>() -> Option<fn(T) -> AnyAction> {
    None
}

impl<T: 'static> Callback<T> {
    pub fn new(name: &'static str, fun_ptr: fn(T) -> AnyAction) -> Self {
        Self {
            fun_ptr: Some(fun_ptr),
            fun_name: Cow::Borrowed(name),
        }
    }

    pub fn call<Action>(&self, args: T) -> Action
    where
        Action: From<AnyAction>,
    {
        if let Some(fun) = self.fun_ptr {
            return fun(args).into();
        }

        // We reach this point only when the callback was deserialized
        for (name, fun) in CALLBACKS {
            if name == &self.fun_name {
                return fun(std::any::type_name::<T>(), Box::new(args)).into();
            }
        }

        panic!("callback function {} not found", self.fun_name)
    }
}

#[macro_export]
macro_rules! _callback {
    ($gensym:ident, $action_ty:ty, $arg:tt, $arg_type:ty, $body:expr) => {{
        use $crate::{AnyAction, Callback, CALLBACKS};
        use linkme::distributed_slice;

        redux::paste::paste! {
            #[allow(unused)] // $arg is marked as unused, but it's used in `$body`
            fn convert_impl($arg: $arg_type) -> AnyAction {
                let action: $action_ty = ($body).into();
                AnyAction(Box::new(action))
            }

            fn $gensym(call_type: &str, args: Box<dyn std::any::Any>) -> AnyAction {
                #[distributed_slice(CALLBACKS)]
                static CALLBACK_DESERIALIZE: (&str, fn(&str, Box<dyn std::any::Any>) -> AnyAction) = (
                    stringify!($gensym),
                    $gensym,
                );

                let $arg = *args.downcast::<$arg_type>()
                    .expect(&format!(
                        "Invalid argument type: {}, expected: {}",
                        call_type,
                        stringify!($arg_type)));

                convert_impl($arg)
            }
        }

        Callback::new(stringify!($gensym), convert_impl)
    }};
}

#[macro_export]
macro_rules! callback {
    (|($($var:ident : $typ:ty),+)| -> $action_ty:ty $body:block) => {
        $crate::gensym::gensym! { $crate::_callback!($action_ty, ($($var),+), ($($typ),+), $body) }
    };
    (|$var:ident : $typ:ty| -> $action_ty:ty $body:block) => {
        $crate::gensym::gensym! { $crate::_callback!($action_ty, $var, $typ, $body) }
    };
}
