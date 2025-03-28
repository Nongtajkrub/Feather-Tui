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
///     let number = trg::cast_arg::<u32>(arg);
///     println!("{}", number);
///     *number == 5
/// });
/// ```
#[macro_export]
macro_rules! trg_new_trigger_func {
    ($func_name:ident, $arg_name:ident, $body:block) => {
        // Do not use Any use std::any::Any only
        fn $func_name($arg_name: &Option<Box<dyn std::any::Any>>) -> bool $body
    };
}

/// Casts the argument of a trigger function to the specified type. For the time
/// being this function will panic if the argument is not set or if the cast type
/// is wrong.
///
/// # Parameters
/// - `arg`: The argument of the trigger function.
///
/// # Notes
/// - This function should only be use in a trigger function. 
///
/// # Usage
/// use this function within a trigger function to cast the argument to the
/// expected type.
///
/// # Example
/// ```rust
/// // A trigger function that take in a u32 an evaluate whether it is five.
/// trg_new_trigger_func!(is_five, arg, {
///     *trg::cast_arg::<u32>(arg) == 5
/// });
///
/// trg::Trigger::new(is_five, 5u32).check(); // Evaluate to true
/// trg::Trigger::new(is_five, 6u32).check(); // Evaluate to false
///                                           
/// trg::Trigger::new(is_five, "String").check(); // Panic
/// trg::Trigger::no_arg(is_five).check();        // Panic
/// ```
#[inline]
pub fn cast_arg<T>(arg: &Option<Box<dyn Any>>) -> &T 
where
    T: 'static,
{
    arg.as_ref().unwrap().downcast_ref::<T>().unwrap()
}

/// A generic trigger handler for evaluating conditions based on stored arguments.
/// `Trigger` allows you to define a condition as a function, associate it with
/// an argument, and check whether the condition is met.
///
/// # Usage
/// Trigger is use for creating a `Selector` object.
///
/// # Example
///   
/// ```
/// use feather_tui as tui;
/// 
/// tui::tui_trg_new_trigger_func!(trigger, arg, {
///     arg.downcast_ref::<u32>().unwrap() == 5
/// });
/// 
/// let mut trig = tui::trg::Trigger::new(trigger, 5u32);
///
/// trig.check(); // Condition is met return True
/// trig.update_arg(6);
/// trig.check(); // Condition no longer met return False
/// ```
pub struct Trigger {
    func: fn(&Option<Box<dyn Any>>) -> bool,
    arg: Option<Box<dyn Any>>,
}

impl Trigger {
    pub fn new<T>(func: fn(&Option<Box<dyn Any>>) -> bool, arg: T) -> Self
    where
        T: 'static,
    {
        Trigger {
            func,
            arg: Some(Box::new(arg)),
        }
    }

    pub fn no_arg(func: fn(&Option<Box<dyn Any>>) -> bool) -> Self {
        Trigger {
            func,
            arg: None,
        }
    }

    pub fn check(&mut self) -> bool {
        (self.func)(&self.arg)
    }

    pub fn update_arg<T>(&mut self, arg: impl Into<Option<T>>) 
    where
        T: 'static
    {
        self.arg = arg.into().map(|arg| Box::new(arg) as Box<dyn Any>);
    }
}
