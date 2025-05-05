use crate::{cpn, err::FtuiResult, trg::Trigger};

/// A `Selector` is used within a `Container` to navigate and select `Option` 
/// components. It allows movement up and down between options and selection of
/// an option using `Trigger`s.
///
/// # Usage
///
/// A `Selector` is required for `Option` components in a `Container` to work.
///
/// # Notes
/// - Without a `Selector`, `Option` components in a `Container` cannot be 
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
    up_trig: Trigger,
    down_trig: Trigger,
    selc_trig: Trigger,
    on: usize,
}

impl Selector {
    pub fn new(up_trig: Trigger, down_trig: Trigger, selc_trig: Trigger) -> Selector {
        Selector {
            up_trig,
            down_trig,
            selc_trig,
            on: 0,
        }
    }

    fn move_up(&mut self, options: &mut Vec<cpn::Option>) -> bool {
        if self.on == 0 {
            return false;
        }

        // move the selector up
        options[self.on].set_selc_on(false);
        self.on -= 1;
        options[self.on].set_selc_on(true);

        true
    }

    fn move_down(&mut self, options: &mut Vec<cpn::Option>) -> bool {
        if self.on == options.len() - 1 {
            return false;
        }

        // move selector down
        options[self.on].set_selc_on(false);
        self.on += 1;
        options[self.on].set_selc_on(true);

        true
    }

    /// Select action always trigger an update.
    fn selc(&mut self, options: &mut Vec<cpn::Option>) -> FtuiResult<()> {
        if let Some(callback) = options[self.on].callback() {
            callback.call()?;
        } else {
            options[self.on].set_is_selected(true);
        }
        Ok(())
    }

    /// Return whether an update occured.
    pub fn looper(&mut self, options: &mut Vec<cpn::Option>) -> FtuiResult<bool> {
        if self.up_trig.check()? && self.move_up(options) {
            Ok(true)
        } else if self.down_trig.check()? && self.move_down(options) {
            Ok(true)
        } else if self.selc_trig.check()? {
            self.selc(options)?;
            Ok(true)
        } else {
            Ok(false)
        }
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
