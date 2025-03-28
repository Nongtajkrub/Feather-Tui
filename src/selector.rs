use crate::{cpn, err::FtuiResult, trg};

/// A `Selector` is used within a `Container` to navigate and select `Option` 
/// components. It allows movement up and down between options and selection of
/// an option using `Trigger`s.
///
/// # Usage
///
/// A `Selector` is required for `Option` components in a `Container` to work.
///
/// # Notes
/// * Without a `Selector`, `Option` components in a `Container` cannot be 
/// selected or navigated.
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // Create a selector with triggers for navigation
/// // (Assuming up_trig, down_trig, and select_trig are defined elsewhere)
/// let selector = tui::slc::Selector::new(up_trig, down_trig, select_trig);
///
/// // Create a container and assign the selector
/// let mut container = tui::con::Container::new();
/// container.set_selector(selector);
/// ```
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

    fn selc(&mut self, options: &mut Vec<cpn::opt::Option>) -> FtuiResult<()> {
        options[self.on as usize].callback().call()?;
        Ok(())
    }

    // return whether a move occure
    pub fn looper(&mut self, options: &mut Vec<cpn::opt::Option>) -> FtuiResult<bool> {
        if self.up_trig.check()? {
            self.move_up(options);
            return Ok(true);
        }
        if self.down_trig.check()? {
            self.move_down(options);
            return Ok(true);
        }
        if self.selc_trig.check()? {
            self.selc(options)?;
            return Ok(true);
        }

        return Ok(false);
    }

    pub fn update_trig_arg<T>(&mut self, up_arg: T, down_arg: T, selc_arg: T)
    where
        T: 'static,
    {
        self.up_trig.update_arg(up_arg);
        self.down_trig.update_arg(down_arg);
        self.selc_trig.update_arg(selc_arg);
    }
}
