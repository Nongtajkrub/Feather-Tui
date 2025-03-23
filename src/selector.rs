use crate::trg;
use crate::cpn;

pub struct Selector {
    up_trig: trg::Trigger,
    down_trig: trg::Trigger,
    selc_trig: trg::Trigger,
    on: u16,
}

impl Selector {
    pub fn new(
        up_trig: trg::Trigger, down_trig: trg::Trigger, selc_trig: trg::Trigger
    ) -> Selector {
        Selector {
            up_trig,
            down_trig,
            selc_trig,
            on: 0,
        }
    }

    fn move_up(&mut self, options: &mut Vec<cpn::opt::Option>) {
        if self.on == 0 {
            return;
        }

        // move the selector up
        options[self.on as usize].set_selc_on(false);
        self.on -= 1;
        options[self.on as usize].set_selc_on(true);
    }

    fn move_down(&mut self, options: &mut Vec<cpn::opt::Option>) {
        if self.on as usize == options.len() - 1 {
            return;
        }

        // move selector down
        options[self.on as usize].set_selc_on(false);
        self.on += 1;
        options[self.on as usize].set_selc_on(true);
    }

    fn selc(&mut self, options: &mut Vec<cpn::opt::Option>) {
        options[self.on as usize].callback().call();
    }

    // return whether a move occure
    pub fn looper(&mut self, options: &mut Vec<cpn::opt::Option>) -> bool {
        if self.up_trig.check() {
            self.move_up(options);
            return true;
        }
        if self.down_trig.check() {
            self.move_down(options);
            return true;
        }
        if self.selc_trig.check() {
            self.selc(options);
            return true;
        }

        return false;
    }

    pub fn update_trig_arg<T: 'static>(
        &mut self, up_arg: T, down_arg: T, selc_arg: T) {
        self.up_trig.update_arg(up_arg);
        self.down_trig.update_arg(down_arg);
        self.selc_trig.update_arg(selc_arg);
    }
}
