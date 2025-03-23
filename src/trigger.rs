#[macro_export]
macro_rules! tui_trg_new_trigger_func {
    ($func_name:ident, $arg_name:ident, $body:block) => {
        fn $func_name($arg_name: &Box<dyn std::any::Any>) -> bool $body
    };
}

/// A generic trigger handler for evaluating conditions based on stored arguments.
/// 
/// `Trigger` allows you to define a condition as a function, associate it with an 
/// argument, and check whether the condition is met. This is useful for handling 
/// events like key presses in a terminal-based UI.
/// 
/// # Example
/// ```
/// use feather_tui as tui;
/// 
/// tui::tui_trg_new_trigger_func!(quit_trigger, key_char, {
///     match key_char.downcast_ref::<std::option::Option<char>>().unwrap() {
///         Some(c) => *c == 'q',
///         None => false,
///     }
/// });
/// 
/// let mut trig = tui::trg::Trigger::new(quit_trigger, Some('q'));
/// assert!(trig.check()); // The trigger condition is met
/// 
/// trig.update_arg(Some('w'));
/// assert!(!trig.check()); // Condition no longer met
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
