#[macro_export]
macro_rules! tui_cbk_new_callback_func {
    ($func_name:ident, $arg_name:ident, $body:block) => {
        fn $func_name($arg_name: &Box<dyn std::any::Any>) $body
    };
}

pub struct Callback {
    func: fn(arg: &Box<dyn std::any::Any>) -> (),
    arg: Box<dyn std::any::Any>,
}

impl Callback {
    pub fn new<T: 'static>(
        func: fn(arg: &Box<dyn std::any::Any>), arg: T) -> Callback {
        Callback {
            func,
            arg: Box::new(arg),
        }
    }

    pub fn call(&self) {
        (self.func)(&self.arg);
    }
}
