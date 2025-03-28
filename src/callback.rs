use std::any::Any;

use crate::err::FtuiResult;

/// This macro generates a function that take a reference to a `Box<dyn Any>`
/// as an argument and return nothing. The function body (`$body`) is the code
/// that will be execute when the callback is trigger.
///
/// # Usage
/// Use for defining functions required to create a `Callback` object,  
/// 
/// # Parameters
/// - `func_name`: An identifier (`ident`) representing the generated function name.
/// - `arg_name`: An identifier (`ident`) representing the function argument name.
/// - `body`: A block (`block`) containing the function implementation.
/// 
/// # Example
/// ```rust
/// use feather_tui as tui;
/// 
/// // A callback function that accept a u32 an print it out.
/// tui::cbk_new_callback_func!(print_num, arg, {
///    println!("{}", tui::cbk::cast_arg::<u32>(arg));
/// });
/// ```
#[macro_export]
macro_rules! cbk_new_callback_func {
    ($func_name:ident, $arg_name:ident, $body:block) => {
        fn $func_name($arg_name: &Option<Box<dyn std::any::Any>>) -> err::FtuiResult<()> $body
    };
}

/// Casts the argument of a callback function to the specified type. For the time
/// being this function will panic if the argument is not set or if the cast type
/// is wrong.
///
/// # Parameters
/// - `arg`: The argument of the callback function.
///
/// # Notes
/// - This function should only be use in a callback function. 
///
/// # Usage
/// use this function within a callback function to cast the argument to the
/// expected type.
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // A callback function that accept a u32 an print it out.
/// tui::cbk_new_callback_func!(print_num, arg, {
///    println!("{}", tui::cbk::cast_arg::<u32>(arg));
/// });
/// 
/// tui::cbk::Callback::new(print_num, 5u32).call(); // print 5
/// tui::cbk::Callback::new(print_num, 6u32).call(); // print 6
///     
/// tui::cbk::Callback::new(print_num, "String").call(); // Panic
/// tui::cbk::Callback::no_arg(print_num).call();        // Panic
/// ```
#[inline]
pub fn cast_arg<T>(arg: &Option<Box<dyn Any>>) -> &T 
where
    T: 'static,
{
    arg.as_ref().unwrap().downcast_ref::<T>().unwrap()
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
    func: fn(&Option<Box<dyn Any>>) -> FtuiResult<()>,
    arg: Option<Box<dyn Any>>,
}

impl Callback {
    pub fn new<T>(
        func: fn(&Option<Box<dyn Any>>) -> FtuiResult<()>, arg: T
    ) -> Self 
    where
        T: 'static,
    {
        Callback {
            func,
            arg: Some(Box::new(arg)),
        }
    }

    pub fn no_arg(func: fn(&Option<Box<dyn Any>>) -> FtuiResult<()>) -> Self {
        Callback {
            func,
            arg: None,
        }
    }

    pub fn call(&self) -> FtuiResult<()> {
        (self.func)(&self.arg)?;
        Ok(())
    }
}
