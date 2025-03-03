#[macro_export]
macro_rules! tui_trg_new_trigger_func {
    ($func_name:ident, $arg_name:ident, $body:block) => {
        fn $func_name($arg_name: &Box<dyn std::any::Any>) -> bool $body
    };
}

pub struct Trigger {
    func: fn(arg: &Box<dyn std::any::Any>) -> bool,
    arg: Box<dyn std::any::Any>,
}

impl Trigger {
    pub fn new<T: 'static>(
        func: fn(arg: &Box<dyn std::any::Any>) -> bool, arg: T) -> Trigger {
        Trigger {
            func,
            arg: Box::new(arg),
        }
    }

    pub fn check(&self) -> bool {
        (self.func)(&self.arg)
    }
}
