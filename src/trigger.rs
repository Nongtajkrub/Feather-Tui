use crate::error::{FtuiResult, FtuiError};
use std::any::Any;

/// This macro generates a function that takes a reference to a `Box<dyn Any>`
/// as an argument and returns a `bool`. The function body (`$body`) determines
/// whether the condition is met.
///
/// # Usage
/// Use for defining functions required to create a `Trigger` object.
///
/// # Parameters
/// - `func_name`: An identifier (`ident`) representing the generated function name.
/// - `arg_name`: An identifier (`ident`) representing the function argument name.
/// - `body`: A block (`block`) containing the function implementation.
/// 
/// # Example
/// ```
/// // Define a trigger function that print the argument than evaluate whether
/// // the argument is 5
/// trg_new_trigger_func!(func_name, arg, {
///     let number = trg::cast_arg::<u32>(arg)?;
///     println!("{}", number);
///     *number == 5
/// });
/// ```
#[macro_export]
macro_rules! trg_new_trigger_func {
    ($func_name:ident, $arg_name:ident, $body:block) => {
        fn $func_name(
            $arg_name: &Option<Box<dyn std::any::Any>>
        ) -> feather_tui::error::FtuiResult<bool> $body
    };
}

/// Casts the argument of a trigger function to the specified type.
///
/// # Parameters
/// - `arg`: The argument of the trigger function.
///
/// # Returns
/// - `Ok(&T)`: The casted argument..
/// - `Err(FtuiError)`: Returns an error.
///
/// # Notes
/// - This function should only be use in a trigger function. 
///
/// # Example
/// ```rust
/// // A trigger function that take in a u32 an evaluate whether it is five.
/// trg_new_trigger_func!(is_five, arg, {
///     Ok(*trg::cast_arg::<u32>(arg)? == 5)
/// });
///
/// assert_eq!(Trigger::new(is_five, 5u32).check()?, true); // Evaluate to true
/// assert_eq!(Trigger::new(is_five, 6u32).check()?, false); // Evaluate to false
///                                           
/// Trigger::new(is_five, "String").check()?; // Error (Wrong type) 
/// Trigger::no_arg(is_five).check()?;        // Error (No argument)
/// ```
pub fn cast_arg<T>(arg: &Option<Box<dyn Any>>) -> FtuiResult<&T> 
where
    T: 'static,
{
    arg.as_ref()
        .ok_or(FtuiError::TriggerCastArgNoArgument)?
        .downcast_ref::<T>()
        .ok_or(FtuiError::TriggerCastArgWrongType)
}

/// A generic trigger handler for evaluating conditions. `Trigger` allows you
/// to define a condition as a function, associate it with an optional argument,
/// and check whether the condition is met.
///
/// # Usage
/// Trigger is use for creating a `Selector` object.
pub struct Trigger {
    func: fn(&Option<Box<dyn Any>>) -> FtuiResult<bool>,
    arg: Option<Box<dyn Any>>,
}

impl Trigger {
    /// Constructs a new `Trigger` with an associated argument.
    ///
    /// # Parameters
    /// - `func`: A trigger function created using the `trg_new_trigger_func!` macro.  
    /// - `arg`: The argument value to associate with the `Trigger` (`T: 'static`).
    ///
    /// # Example
    /// ```rust
    /// // Define a trigger function using the macro.
    /// trg_new_trigger_func!(trigger_function, arg, {
    ///     ...
    /// });
    ///
    /// // Create a `Trigger` with an associated `u32` value.
    /// let _ = Trigger::new(trigger_function, 5u32);
    /// ```
    pub fn new<T>(
        func: fn(&Option<Box<dyn Any>>) -> FtuiResult<bool>, arg: T
    ) -> Self
    where
        T: 'static,
    {
        Trigger {
            func,
            arg: Some(Box::new(arg)),
        }
    }

    /// Constructs a new `Trigger` without an associated argument.
    ///
    /// # Parameters
    /// - `func`: A trigger function created using the `trg_new_trigger_func!` macro.  
    ///
    /// # Example
    /// ```rust
    /// // Define a trigger function using the macro.
    /// trg_new_trigger_func!(trigger_function, arg, {
    ///     ...
    /// });
    ///
    /// // Create a `Trigger` without an associated argument.
    /// let _ = Trigger::no_arg(trigger_function);
    /// ```
    pub fn no_arg(func: fn(&Option<Box<dyn Any>>) -> FtuiResult<bool>) -> Self {
        Trigger {
            func,
            arg: None,
        }
    }

    /// Check whether the `Trigger` evaluate to `true` or `false`. Typically used
    /// for testing purposes.
    ///
    /// # Returns
    /// - `Ok(bool)`: Whether the trigger condition was met.
    /// - `Err(FtuiError)`: Returns an error.  
    ///
    /// # Example
    /// ```rust
    /// // A trigger function that take in a u32 an evaluate whether it is five.
    /// trg_new_trigger_func!(is_five, arg, {
    ///     Ok(*trg::cast_arg::<u32>(arg)? == 5)
    /// });
    ///
    /// assert_eq!(Trigger::new(is_five, 5u32).check()?, true);  // Evaluates to true
    /// assert_eq!(Trigger::new(is_five, 6u32).check()?, false); // Evaluates to false
    /// ```
    pub fn check(&self) -> FtuiResult<bool> {
        (self.func)(&self.arg)
    }

    /// Updates the argument associated with the `Trigger`.  
    ///
    /// # Parameters
    /// - `arg`: The new argument value to associate with the `Trigger` (`T' static`).
    ///
    /// # Example
    /// ```rust
    /// // Define a trigger function that checks if the argument is 5.
    /// trg_new_trigger_func!(is_five, arg, {
    ///     Ok(*trg::cast_arg::<u32>(arg)? == 5)
    /// });
    ///
    /// // Create a `Trigger` with an initial argument value of 5.
    /// let mut trigger = Trigger::new(is_five, 5u32);
    ///
    /// assert_eq!(trigger.check()?, true); // Evaluates to true.
    ///
    /// // Update the argument to a different value.
    /// trigger.update_arg(6u32);
    ///
    /// assert_eq!(trigger.check()?, false); // Now evaluates to false.
    /// ```
    pub fn update_arg<T>(&mut self, arg: T) 
    where
        T: 'static
    {
        self.arg = Some(Box::new(arg));
    }

    /// Remove the argument associated with the `Trigger`.
    pub fn remove_arg(&mut self) {
        self.arg = None;
    }
}
