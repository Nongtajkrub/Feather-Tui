/// This macro generates a function that takes a reference to a `Box<dyn Any>`
/// as an argument and returns a `bool`. The function body (`$body`) determines
/// whether the condition is met.
///
/// # Usage
///
/// Use for defining functions required to create a `Trigger` object.
/// 
/// # Example
/// ```
/// // Define a trigger function that print the argument than evaluate whether the argument is 5
/// tui_trg_new_trigger_func!(function_name, argument_name, {
///     let number = *argument_name.downcast_ref::<u32>().unwrap();
///     println!("{}", number);
///     return number == 5;
/// });
/// ```
#[macro_export]
macro_rules! tui_trg_new_trigger_func {
    ($func_name:ident, $arg_name:ident, $body:block) => {
        fn $func_name($arg_name: &Box<dyn std::any::Any>) -> bool $body
    };
}

/// A generic trigger handler for evaluating conditions based on stored arguments.
/// 
/// `Trigger` allows you to define a condition as a function, associate it with an argument, and check whether the condition is met.
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
    func: fn(arg: &Box<dyn std::any::Any>) -> bool,
    arg: Box<dyn std::any::Any>,
}

impl Trigger {
    pub fn new<T: 'static>(
        func: fn(arg: &Box<dyn std::any::Any>) -> bool, arg: T) -> Self {
        Trigger {
            func,
            arg: Box::new(arg),
        }
    }

    pub fn check(&self) -> bool {
        (self.func)(&self.arg)
    }

    pub fn update_arg<T: 'static>(&mut self, arg: T) {
        self.arg = Box::new(arg);
    }
}
