use std::any::Any;

/// This macro generates a function that take a reference to a `Box<dyn Any>`
/// as an argument and return nothing. The function body (`$body`) is the code
/// that will be execute when the callback is trigger.
///
/// # Usage
/// Use for defining functions required to create a `Callback` object,  
/// 
/// # Example
/// ```rust
/// use feather_tui as tui;
/// 
/// // Define a callback function that print out the argument that is was given
/// tui::tui_cbk_new_callback_func!(function_name, argument_name, {
///     println!(
///         "Callback received: {}",
///         argument_name.downcast_ref::<u32>().unwrap());
/// });
/// ```
#[macro_export]
macro_rules! cbk_new_callback_func {
    ($func_name:ident, $arg_name:ident, $body:block) => {
        fn $func_name($arg_name: &Option<Box<dyn std::any::Any>>) $body
    };
}

/// A generic callback handler for executing functions with stored arguments.
/// 
/// `Callback` allows you to associate a function with an argument and invoke 
/// it later. 
///
/// # Usage
/// `Callback` is use for creating a `Option` component. The callback will be
/// trigger when the `Option` component is selected.
/// 
/// # Example
/// ```rust
/// use feather_tui as tui;
/// 
/// // Define a callback function that print out the argument that is was given 
/// tui::tui_cbk_new_callback_func!(callback_func, argument, {
///     println!(
///         "Callback Argument: {}", 
///         argument.downcast_ref::<u32>().unwrap());
/// });
/// 
/// let cb = tui::cbk::Callback::new(callback_func, 42u32);
/// cb.call(); // Prints: Callback Argument: 42
/// ```
pub struct Callback {
    func: fn(&Option<Box<dyn Any>>) -> (),
    arg: Option<Box<dyn Any>>,
}

impl Callback {
    pub fn new<T>(func: fn(&Option<Box<dyn Any>>), arg: T) -> Self 
    where
        T: 'static,
    {
        Callback {
            func,
            arg: Some(Box::new(arg)),
        }
    }

    pub fn no_arg(func: fn(&Option<Box<dyn Any>>)) -> Self {
        Callback {
            func,
            arg: None,
        }
    }

    pub fn call(&self) {
        (self.func)(&self.arg);
    }
}
