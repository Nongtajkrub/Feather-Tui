use crate::{cpn, err::{FtuiResult, FtuiError}, trg::Trigger};

/// A `Selector` is used within a `Container` to navigate and select `Option` 
/// components. It allows movement up and down between options and selection of
/// an option.
///
/// # Usage
/// A `Selector` is required for `Option` components in a `Container` to work.
///
/// # Notes
/// - Without a `Selector`, `Option` components in a `Container` cannot be 
/// selected or navigated.
pub struct Selector {
    up_trig: Option<Trigger>,
    down_trig: Option<Trigger>,
    selc_trig: Option<Trigger>,
    on: usize,
}

impl Selector {
    /// Constructs a new `Selector` component with triggers functions for 
    /// navigating up, down, and selecting. Each action is associated with a `Trigger`,
    /// which determines whether the corresponding input (up, down, or select) is activated.
    ///
    /// # Example
    /// ```rust
    /// // A `Trigger` that always activates.
    /// tui::trg_new_trigger_func!(always_true_trigger, _arg, {
    ///     Ok(true)
    /// });
    ///
    /// // A `Trigger` that never activates.
    /// tui::trg_new_trigger_func!(always_false_trigger, _arg, {
    ///     Ok(false)
    /// });
    ///
    /// // Create a `Selector` that always moves down when evaluated.
    /// let selector = Selector::new(
    ///     Trigger::no_arg(always_false_trigger), // up
    ///     Trigger::no_arg(always_true_trigger),  // down
    ///     Trigger::no_arg(always_false_trigger), // select
    /// );
    /// ```
    pub fn new(up_trig: Trigger, down_trig: Trigger, selc_trig: Trigger) -> Self {
        Selector {
            up_trig: Some(up_trig),
            down_trig: Some(down_trig),
            selc_trig: Some(selc_trig),
            on: 0,
        }
    }

    /// Creates a new `Selector` component without any trigger functions.
    /// When using this variant, you must manually control its behavior by calling
    /// the `selector_up`, `selector_down`, and `selector_select` methods on the
    /// `Container` that contains it.
    ///
    /// # Example
    /// ```rust
    /// // Create a new `Selector` with no triggers.
    /// let _ = Selector::no_triggers();
    ///
    /// // Controlling the `Selector` manually.
    ///
    /// // Build a container that contains a manual `Selector`.
    /// let mut container = ContainerBuilder::new()
    ///     .selector_no_triggers()
    ///     .build();
    ///
    /// // Move the selector up.
    /// container.selector_up()?;
    ///
    /// // Move the selector down.
    /// container.selector_down()?;
    ///
    /// // Select the currently highlighted option.
    /// container.selector_select()?;
    /// ```
    pub fn no_triggers() -> Self {
        Selector {
            up_trig: None,
            down_trig: None,
            selc_trig: None,
            on: 0,
        }
    }

    pub(crate) fn up(&mut self, options: &mut Vec<cpn::Option>) -> bool {
        if self.on == 0 {
            return false;
        }

        // move the selector up
        options[self.on].set_selc_on(false);
        self.on -= 1;
        options[self.on].set_selc_on(true);

        true
    }

    pub(crate) fn down(&mut self, options: &mut Vec<cpn::Option>) -> bool {
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
    pub(crate) fn select(&mut self, options: &mut Vec<cpn::Option>) -> FtuiResult<bool> {
        if let Some(callback) = options[self.on].callback() {
            callback.call()?;
        }
        options[self.on].set_is_selc(true);
        Ok(true)
    }

    #[inline]
    fn have_trigs(&self) -> bool {
        matches!(
            (&self.up_trig, &self.down_trig, &self.selc_trig),
            (Some(_), Some(_), Some(_)))
    }

    /// Return whether an update occured.
    pub(crate) fn looper(&mut self, options: &mut Vec<cpn::Option>) -> FtuiResult<bool> {
        // Hard to avoid using `unwrap`.
        if !self.have_trigs() {
            return Err(FtuiError::SelectorNoTriggers);
        }

        if self.up_trig.as_ref().unwrap().check()? && self.up(options) {
            Ok(true)
        } else if self.down_trig.as_ref().unwrap().check()? && self.down(options) {
            Ok(true)
        } else if self.selc_trig.as_ref().unwrap().check()? {
            self.select(options)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    pub fn up_trig_mut(&mut self) -> FtuiResult<&mut Trigger> {
        self.up_trig.as_mut().ok_or(FtuiError::SelectorNoTriggers)
    }

    #[inline]
    pub fn down_trig_mut(&mut self) -> FtuiResult<&mut Trigger> {
        self.down_trig.as_mut().ok_or(FtuiError::SelectorNoTriggers)
    }

    #[inline]
    pub fn select_trig_mut(&mut self) -> FtuiResult<&mut Trigger> {
        self.selc_trig.as_mut().ok_or(FtuiError::SelectorNoTriggers)
    }

    /// Update all of the `Selector` triggers argument. 
    pub fn update_trig_arg<T, U, V>(
        &mut self, up_arg: T, down_arg: U, selc_arg: V
    ) -> FtuiResult<()>
    where
        T: 'static,
        U: 'static,
        V: 'static,
    {
        match (&mut self.up_trig, &mut self.down_trig, &mut self.selc_trig) {
            (Some(u), Some(d), Some(s)) => {
                u.update_arg(up_arg);
                d.update_arg(down_arg);
                s.update_arg(selc_arg);
                Ok(())
            }
            _ => Err(FtuiError::SelectorNoTriggers),
        }
    }
}
