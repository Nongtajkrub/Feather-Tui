use crate::{components as cpn, error::FtuiResult};

/// A UI component use for navigating and selecting `Option` components.
/// It allows movement up and down between options and selection of an option.
///
/// # Usage
/// A `Selector` is required for `Option` components in a `Container` to work.
///
/// # Notes
/// - Without a `Selector`, `Option` components in a `Container` cannot be 
/// selected or navigated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selector {
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
    pub fn new() -> Self {
        Selector {
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
        options[self.on].set_is_selc(true);
        Ok(true)
    }
}
