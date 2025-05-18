use crate::error::{FtuiResult, FtuiError};
use std::any::Any;

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
/// // A callback function that accept a u32 an print it out.
/// cbk_new_callback_func!(print_num, arg, {
///    println!("{}", cbk::cast_arg::<u32>(arg)?);
///    Ok(())
/// });
/// ```
#[macro_export]
macro_rules! cbk_new_callback_func {
    ($func_name:ident, $arg_name:ident, $body:block) => {
        fn $func_name(
            $arg_name: &Option<Box<dyn std::any::Any>>
        ) -> feather_tui::error::FtuiResult<()> $body
    };
}

/// Casts the argument of a callback function to the specified type.
///
/// # Parameters
/// - `arg`: The argument of the callback function.
///
/// # Returns
/// - `Ok(&T)`: The casted argument..
/// - `Err(FtuiError)`: Returns an error.
///
/// # Notes
/// - This function should only be use in a callback function. 
///
/// # Example
/// ```rust
/// // A callback function that accept a u32 an print it out.
/// cbk_new_callback_func!(print_num, arg, {
///    println!("{}", cbk::cast_arg::<u32>(arg)?);
///    Ok(())
/// });
/// 
/// Callback::new(print_num, 5u32).call()?; // print 5
/// Callback::new(print_num, 6u32).call()?; // print 6
///     
/// Callback::new(print_num, "String").call()?; // Error (Wrong type)
/// Callback::no_arg(print_num).call()?;        // Error (No argument)
/// ```
pub fn cast_arg<T>(arg: &Option<Box<dyn Any>>) -> FtuiResult<&T> 
where
    T: 'static,
{
    arg.as_ref()
        .ok_or(FtuiError::CallbackCastArgNoArgument)?
        .downcast_ref::<T>()
        .ok_or(FtuiError::CallbackCastArgWrongType)
}

/// A generic callback handler for executing functions with stored arguments. 
/// `Callback` allows you to associate a function with an optional argument and
/// invoke it later. 
///
/// # Usage
/// `Callback` is use for creating a `Option` component. The callback will be
/// trigger when the `Option` component is selected.
pub struct Callback {
    func: fn(&Option<Box<dyn Any>>) -> FtuiResult<()>,
    arg: Option<Box<dyn Any>>,
}

impl Callback {
    /// Constructs a new `Callback` with an associated argument.
    ///
    /// # Parameters
    /// - `func`: A callback function created using the `cbk_new_callback_func!` macro.  
    /// - `arg`: The argument value to associate with the `Callback` (`T: 'static`).
    ///
    /// # Example
    /// ```rust
    /// // Define a callback function using the macro.
    /// cbk_new_callback_func!(callback_function, arg, {
    ///     ...
    /// });
    ///
    /// // Create a `Callback` with an associated `u32` value.
    /// let _ = Callback::new(callback_function, 5u32);
    /// ```
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

    /// Constructs a new `Callback` without an associated argument.
    ///
    /// # Parameters
    /// - `func`: A callback function created using the `cbk_new_callback_func!` macro.  
    ///
    /// # Example
    /// ```rust
    /// // Define a callback function using the macro.
    /// cbk_new_callback_func!(callback_function, arg, {
    ///     ...
    /// });
    ///
    /// // Create a `Callback` without a associated argument.
    /// let _ = Callback::no_arg(callback_function);
    /// ```
    pub fn no_arg(func: fn(&Option<Box<dyn Any>>) -> FtuiResult<()>) -> Self {
        Callback {
            func,
            arg: None,
        }
    }

    /// Invoke the `Callback`. Typically used for testing purposes.
    ///
    /// # Returns
    /// - `Ok(())`: Returns nothing.
    /// - `Err(FtuiError)`: Returns an error.  
    ///
    /// # Example
    /// ```rust
    /// // Define a callback function that accepts a `u32` and prints it.
    /// cbk_new_callback_func!(print_num, arg, {
    ///     println!("{}", tui::cbk::cast_arg::<u32>(arg)?);
    ///     Ok(())
    /// });
    /// 
    /// // Create a `Callback` with an argument of 5 and invoke it.
    /// Callback::new(print_num, 5u32).call()?; // Prints: 5
    /// ```
    pub fn call(&self) -> FtuiResult<()> {
        (self.func)(&self.arg)?;
        Ok(())
    }

    /// Updates the argument associated with this `Callback`.
    ///
    /// # Parameters
    /// - `arg`: The new argument value to associate with the `Callback` (`T' static`).
    ///
    /// # Example
    /// ```rust
    /// // Define a callback function that accepts a `u32` and prints it.
    /// cbk_new_callback_func!(print_num, arg, {
    ///     println!("{}", tui::cbk::cast_arg::<u32>(arg)?);
    ///     Ok(())
    /// });
    /// 
    /// // Create a `Callback` with an initial argument.
    /// let mut callback = Callback::new(print_num, 5u32); 
    ///
    /// callback.call()?; // Prints: 5
    ///
    /// // Update the argument to a new value.
    /// callback.update_arg(6u32);
    ///
    /// callback.call()?; // Prints: 6
    /// ```
    pub fn update_arg<T>(&mut self, arg: T)
    where
        T: 'static
    {
        self.arg = Some(Box::new(arg));
    }

    /// Remove the argument associated with the `Callback`.
    pub fn remove_arg(&mut self) {
        self.arg = None;
    }
}
