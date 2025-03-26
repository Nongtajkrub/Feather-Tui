use crate::{error::FtuiError, util::ansi};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TextFlags: u16 {
        // NONE can't be 0
        const NONE          = 1 << 0;

        // alignment
        const ALIGN_RIGHT   = 1 << 1;
        const ALIGN_MIDDLE  = 1 << 2;
        const ALIGN_BOTTOM  = 1 << 3;

        // color settings
        const COLOR_BACK    = 1 << 4;

        // colors
        const COLOR_BLACK   = 1 << 6;
        const COLOR_RED     = 1 << 7;
        const COLOR_GREEN   = 1 << 8;
        const COLOR_YELLOW  = 1 << 9;
        const COLOR_BLUE    = 1 << 10;
        const COLOR_MAGENTA = 1 << 11;
        const COLOR_CYAN    = 1 << 12;
        const COLOR_WHITE   = 1 << 13;

        const COLOR_BLACK_BACK = 
            Self::COLOR_BLACK.bits() | Self::COLOR_BACK.bits();
        const COLOR_RED_BACK = 
            Self::COLOR_RED.bits() | Self::COLOR_BACK.bits();
        const COLOR_GREEN_BACK = 
            Self::COLOR_GREEN.bits() | Self::COLOR_BACK.bits();
        const COLOR_YELLOW_BACK = 
            Self::COLOR_YELLOW.bits() | Self::COLOR_BACK.bits();
        const COLOR_BLUE_BACK =
            Self::COLOR_BLUE.bits() | Self::COLOR_BACK.bits();
        const COLOR_MAGENTA_BACK =
            Self::COLOR_MAGENTA.bits() | Self::COLOR_BACK.bits();
        const COLOR_CYAN_BACK =
            Self::COLOR_CYAN.bits() | Self::COLOR_BACK.bits();
        const COLOR_WHITE_BACK =
            Self::COLOR_WHITE.bits() | Self::COLOR_BACK.bits();
    }
}

impl Default for TextFlags {
    fn default() -> Self {
        Self::NONE
    }
}

/// A UI component representing a text element in a `Container`. `Text` components
/// are displayed in the order they are added to the `Container`. They can be
/// customized using `TextFlags` to adjust alignment, color, and other styling options.
///
/// # Usage
///
/// The `Text` component is used within a `Container` to display static text elements.  
///
/// # Derives
/// `Debug`, `Clone`, `PartialEq`
///
/// # PartialEq Implementation
///
/// Only the `label` and `flags` are considered when comparing `Text` instances.
///
/// # Example
///
/// ```rust
/// use feather_tui as tui;
///
/// // Create a text component with custom styling
/// let text = 
///     tui::cpn::txt::Text::new(
///         "Text!",
///         tui::cpn::txt::TextFlags::COLOR_CYAN_BACK |
///         tui::cpn::txt::TextFlags::ALIGN_RIGHT);
///
/// // Add the text component to a container
/// let mut container = tui::con::Container::new();
/// container.add_text(text);
/// ```
#[derive(Debug, Clone)]
pub struct Text {
    label: String,
    line: u16,
    flags: TextFlags,
    pos_resolve: bool,
    pos: u16,
    color: String,
}

/// Implementation of the `PartialEq` trait for the `Text` struct. This implementation
/// defines equality based on two fields: `label` and `flags`.
///
/// # Notes
/// - Only the `label` and `flags` are considered when comparing `Text` instances.
///
/// # Example
/// ```rust
/// use feather_tui as tui; 
///
/// // Create two `Text` instances with identical `label` and `flags`
/// let text1 = tui::cpn::txt::Text::new("Text", tui::cpn::txt::TextFlags::NONE);
/// let text2 = tui::cpn::txt::Text::new("Text", tui::cpn::txt::TextFlags::NONE);
///
/// // Assert that both `text1` and `text2` are equal, since they have the same `label` and `flags`
/// assert_eq!(text1, text2); // This will evaluate to true
/// 
/// // Create a new `Text` instance with a different flag (ALIGN_RIGHT)
/// let text3 = tui::cpn::txt::Text::new("Text", tui::cpn::txt::TextFlags::ALIGN_RIGHT);
/// 
/// // Assert that `text1` and `text3` are not equal, since their `flags` are different
/// assert_ne!(text1, text3); // This will evaluate to true (text1 != text3)
/// 
/// // Create a new `Text` instance with a different `label`
/// let text4 = tui::cpn::txt::Text::new("Hello", tui::cpn::txt::TextFlags::ALIGN_RIGHT);
/// 
/// // Assert that `text3` and `text4` are not equal, since their `label` is different
/// assert_ne!(text3, text4); // This will evaluate to true (text3 != text4)
/// ```
impl PartialEq for Text {
    fn eq(&self, others: &Self) -> bool {
        self.label == others.label &&
        self.flags == others.flags
    }
}

impl Text {
    /// Creates a new `Text` component with the specified label and flags.
    ///
    /// # Parameters
    /// * `label`: A `&str` representing the text content.
    /// * `flags`: A set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Notes
    ///
    /// * This is what bitwise OR operator look like -> `flag1 | flag2 | flag3 ...`
    ///
    /// # Returns
    /// * `Ok(Text)`: Returns a `Text` instance
    /// * `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// // Create a `Text` component labeled "Text".
    /// // The text is right-aligned.
    /// // The background color is red.
    /// // (Use `COLOR_FORE` to change the foreground color.)
    /// let text = 
    ///     tui::cpn::txt::Text::new(
    ///         "Text",
    ///         tui::cpn::txt::TextFlags::ALIGN_RIGHT | 
    ///         tui::cpn::txt::TextFlags::COLOR_BACK |
    ///         tui::cpn::txt::TextFlags::COLOR_RED);
    /// ```
    pub fn new(label: &str, flags: impl Into<Option<TextFlags>>) -> Result<Self, FtuiError> {
        if label.is_empty() {
            return Err(FtuiError::LabelEmpty);
        }

        let flags = flags.into().unwrap_or(TextFlags::NONE);

        Self::ensure_compatible_flags(&flags)?; 
        
        Ok(Text {
            label: label.to_string(),
            line: 0,
            flags,
            pos_resolve: false,
            pos: 0,
            color: Self::resolve_color(flags),
        })
    }

    fn ensure_compatible_flags(flags: &TextFlags) -> Result<(), FtuiError> {
        // NONE Flags alone is always compatible
        if *flags == TextFlags::NONE {
            return Ok(());
        }

        // NONE Flags should not be combined with any other flags
        if flags.contains(TextFlags::NONE) && *flags != TextFlags::NONE {
            return Err(FtuiError::TextFlagNoneWithOther);
        }

        // Only one color can be set
        if flags
            .intersection(
                TextFlags::COLOR_BACK |
                TextFlags::COLOR_RED |
                TextFlags::COLOR_GREEN |
                TextFlags::COLOR_YELLOW |
                TextFlags::COLOR_BLUE |
                TextFlags::COLOR_MAGENTA |
                TextFlags::COLOR_CYAN |
                TextFlags::COLOR_WHITE)
            .bits()
            .count_ones() > 1 
        {
            return Err(FtuiError::TextFlagMultipleColor);
        }

        Ok(())
    }

    #[inline]
    fn color_f_or_b(flags: TextFlags, b: &str, f: &str) -> String {
        // if COLOR_BACK flag is not set text will default to foreground color
        if flags.contains(TextFlags::COLOR_BACK) {
            b.to_string() 
        } else {
            f.to_string()
        }
    }

    fn resolve_color(flags: TextFlags) -> String {
        if flags.contains(TextFlags::COLOR_BLACK) {
            Self::color_f_or_b(flags, ansi::ESC_BLACK_B, ansi::ESC_BLACK_F)
        } else if flags.contains(TextFlags::COLOR_RED) {
            Self::color_f_or_b(flags, ansi::ESC_RED_B, ansi::ESC_RED_F)
        } else if flags.contains(TextFlags::COLOR_GREEN) {
            Self::color_f_or_b(flags, ansi::ESC_GREEN_B, ansi::ESC_GREEN_F)
        } else if flags.contains(TextFlags::COLOR_YELLOW) {
            Self::color_f_or_b(flags, ansi::ESC_YELLOW_B, ansi::ESC_YELLOW_F)
        } else if flags.contains(TextFlags::COLOR_BLUE) {
            Self::color_f_or_b(flags, ansi::ESC_BLUE_B, ansi::ESC_BLUE_F)
        } else if flags.contains(TextFlags::COLOR_MAGENTA) {
            Self::color_f_or_b(flags, ansi::ESC_MAGENTA_B, ansi::ESC_MAGENTA_F)
        } else if flags.contains(TextFlags::COLOR_CYAN) {
            Self::color_f_or_b(flags, ansi::ESC_CYAN_B, ansi::ESC_CYAN_F)
        } else if flags.contains(TextFlags::COLOR_WHITE) {
            Self::color_f_or_b(flags, ansi::ESC_WHITE_B, ansi::ESC_WHITE_F)
        } else {
            String::from("")
        }
    }

    pub fn set_line(&mut self, line: u16) {
        self.line = line;
    }

    pub fn line(&self) -> u16 {
        return self.line;
    }

    pub fn label(&self) -> &String {
        return &self.label;
    }

    pub fn len(&self) -> usize {
        return self.label.len();
    }

    pub fn set_pos(&mut self, pos: u16) {
        self.pos = pos;
    }

    pub fn pos(&self) -> u16 {
        return self.pos;
    }

    pub fn pos_resolve(&self) -> bool {
        return self.pos_resolve;
    }

    pub fn set_pos_resolve(&mut self, value: bool) {
        self.pos_resolve = value;
    }

    pub fn flags(&self) -> &TextFlags {
        return &self.flags;
    }

    pub fn color(&self) -> &String {
        return &self.color;
    }
}
