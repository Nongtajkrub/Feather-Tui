use crate::{cpn, err::{FtuiResult, FtuiError}, trg::Trigger};

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
    up_trig: Option<Trigger>,
    down_trig: Option<Trigger>,
    selc_trig: Option<Trigger>,
    on: usize,
}

impl Selector {
    pub fn new(up_trig: Trigger, down_trig: Trigger, selc_trig: Trigger) -> Self {
        Selector {
            up_trig: Some(up_trig),
            down_trig: Some(down_trig),
            selc_trig: Some(selc_trig),
            on: 0,
        }
    }

    pub fn no_triggers() -> Self {
        Selector {
            up_trig: None,
            down_trig: None,
            selc_trig: None,
            on: 0,
        }
    }

    pub(crate) fn move_up(&mut self, options: &mut Vec<cpn::Option>) -> bool {
        if self.on == 0 {
            return false;
        }

        // move the selector up
        options[self.on].set_selc_on(false);
        self.on -= 1;
        options[self.on].set_selc_on(true);

        true
    }

    pub(crate) fn move_down(&mut self, options: &mut Vec<cpn::Option>) -> bool {
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
    pub(crate) fn selc(&mut self, options: &mut Vec<cpn::Option>) -> FtuiResult<()> {
        if let Some(callback) = options[self.on].callback() {
            callback.call()?;
        }
        Ok(())
    }

    /// Return whether an update occured.
    pub fn looper(&mut self, options: &mut Vec<cpn::Option>) -> FtuiResult<bool> {
        if let Some(up_trig) = self.up_trig.as_ref() {
            if up_trig.check()? && self.move_up(options) {
                return Ok(true);
            }
        }

        if let Some(down_trig) = self.down_trig.as_ref() {
            if down_trig.check()? && self.move_down(options) {
                return Ok(true);
            }
        }

        if let Some(selc_trig) = self.selc_trig.as_ref() {
            if selc_trig.check()? {
                self.selc(options)?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn update_trig_arg<T>(
        &mut self, up_arg: T, down_arg: T, selc_arg: T
    ) -> FtuiResult<()>
    where
        T: 'static,
    {
        match (
            self.up_trig.as_mut(),
            self.down_trig.as_mut(),
            self.selc_trig.as_mut(),
        ) {
            (Some(up_trig), Some(down_trig), Some(selc_trig)) => {
                up_trig.update_arg(up_arg);
                down_trig.update_arg(down_arg);
                selc_trig.update_arg(selc_arg);

                Ok(())
            },
            _ => Err(FtuiError::SelectorNoTriggers),
        }
    }
}
