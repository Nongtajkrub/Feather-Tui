use crate::{error::{FtuiError, FtuiResult}, util::ansi};
use bitflags::bitflags;
use unicode_segmentation::UnicodeSegmentation;

bitflags! {
    /// Flags used to style a `Text` component. Multiple flags can be combined 
    /// using the bitwise OR operator to apply multiple styles simultaneously.
    ///
    /// # Note
    /// The bitwise OR operator combines flags like this: `flag1 | flag2 | flag3`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TextFlags: u32 {
        // NONE can't be 0
        /// No flags.
        const NONE          = 1 << 0;

        /// Aligns text to the right of the renderer.
        const ALIGN_RIGHT   = 1 << 1;
        /// Centers text horizontally in the renderer.
        const ALIGN_MIDDLE  = 1 << 2;
        /// Aligns text to the bottom of the renderer.
        const ALIGN_BOTTOM  = 1 << 3;

        // Applies colors to the background of the text instead of foreground.
        const COLOR_BACK    = 1 << 4;

        /// Apply the black color to the text component.
        const COLOR_BLACK   = 1 << 6;
        /// Apply the red color to the text component.
        const COLOR_RED     = 1 << 7;
        /// Apply the green color to the text component.
        const COLOR_GREEN   = 1 << 8;
        /// Apply the yellow color to the text component.
        const COLOR_YELLOW  = 1 << 9;
        /// Apply the blue color to the text component.
        const COLOR_BLUE    = 1 << 10;
        /// Apply the magenta color to the text component.
        const COLOR_MAGENTA = 1 << 11;
        /// Apply the cyan color to the text component.
        const COLOR_CYAN    = 1 << 12;
        /// Apply the white color to the text component.
        const COLOR_WHITE   = 1 << 13;

        /// Apply the black color to the text component background.
        const COLOR_BLACK_BACK = 
            Self::COLOR_BLACK.bits() | Self::COLOR_BACK.bits();
        /// Apply the red color to the text component background.
        const COLOR_RED_BACK = 
            Self::COLOR_RED.bits() | Self::COLOR_BACK.bits();
        /// Apply the green color to the text component background.
        const COLOR_GREEN_BACK = 
            Self::COLOR_GREEN.bits() | Self::COLOR_BACK.bits();
        /// Apply the yellow color to the text component background.
        const COLOR_YELLOW_BACK = 
            Self::COLOR_YELLOW.bits() | Self::COLOR_BACK.bits();
        /// Apply the blue color to the text component background.
        const COLOR_BLUE_BACK =
            Self::COLOR_BLUE.bits() | Self::COLOR_BACK.bits();
        /// Apply the magneta color to the text component background.
        const COLOR_MAGENTA_BACK =
            Self::COLOR_MAGENTA.bits() | Self::COLOR_BACK.bits();
        /// Apply the cyan color to the text component background.
        const COLOR_CYAN_BACK =
            Self::COLOR_CYAN.bits() | Self::COLOR_BACK.bits();
        /// Apply the white color to the text component background.
        const COLOR_WHITE_BACK =
            Self::COLOR_WHITE.bits() | Self::COLOR_BACK.bits();

        /// Make the text component bold
        const STYLE_BOLD   = 1 << 14;
        /// Dim the text component
        const STYLE_DIM    = 1 << 15;
        /// Make the text component italic.
        const STYLE_ITALIC = 1 << 16;
        /// Underlined the text component.
        const STYLE_UNDER  = 1 << 17;
        /// Applies strike through to the text component.
        const STYLE_STRIKE = 1 << 18;
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
/// The `Text` component is used within a `Container` to display static text elements.  
///
/// # Derives
/// `Debug`, `Clone`, `PartialEq`
///
/// # PartialEq Implementation
///
/// Only the `label` and `flags` are considered when comparing `Text` instances.
#[derive(Debug, Clone)]
pub struct Text {
    label: String,
    len: usize,
    id: u16,
    line: u16,
    flags: TextFlags,
    pos: u16,
    style: Vec<&'static str>,
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
/// let text1 = tui::cpn::Text::new("Text", tui::cpn::TextFlags::NONE);
/// let text2 = tui::cpn::Text::new("Text", tui::cpn::TextFlags::NONE);
///
/// // Assert that both `text1` and `text2` are equal, since they have the same `label` and `flags`
/// assert_eq!(text1, text2); // This will evaluate to true
/// 
/// // Create a new `Text` instance with a different flag (ALIGN_RIGHT)
/// let text3 = tui::cpn::Text::new("Text", tui::cpn::TextFlags::ALIGN_RIGHT);
/// 
/// // Assert that `text1` and `text3` are not equal, since their `flags` are different
/// assert_ne!(text1, text3); // This will evaluate to true (text1 != text3)
/// 
/// // Create a new `Text` instance with a different `label`
/// let text4 = tui::cpn::Text::new("Hello", tui::cpn::TextFlags::ALIGN_RIGHT);
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
    /// - `label`: A `&str` representing the text content.
    /// - `flags`: A set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Notes
    /// The bitwise OR operator combines flags like this: `flag1 | flag2 | flag3`
    ///
    /// # Returns
    /// - `Ok(Text)`: Returns a `Text` instance
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a `Text` component labeled "Text".
    /// // The text is right-aligned.
    /// // The background color is red.
    /// let _ = Text::new(
    ///     "Text", TextFlags::ALIGN_RIGHT | TextFlags::COLOR_RED_BACK)?;
    /// ```
    pub fn new(label: &str, flags: impl Into<Option<TextFlags>>) -> FtuiResult<Self> {
        let flags = flags.into().unwrap_or(TextFlags::NONE);

        Self::ensure_compatible_flags(&flags)?; 
        
        Ok(Text {
            label: label.to_string(),
            len: label.graphemes(true).count(),
            id: 0,
            line: 0,
            flags,
            pos: 0,
            style: Self::resolve_style(flags),
        })
    }

    pub(crate) fn with_id(
        label: &str, flags: impl Into<Option<TextFlags>>, id: u16
    ) -> FtuiResult<Self> {
        let mut text = Text::new(label, flags)?;
        text.set_id(id);
        Ok(text)
    }

    pub(crate) fn ensure_compatible_flags(flags: &TextFlags) -> FtuiResult<()> {
        // NONE Flags alone is always compatible.
        if *flags == TextFlags::NONE {
            return Ok(());
        }

        // NONE Flags should not be combined with any other flags.
        if flags.contains(TextFlags::NONE) && *flags != TextFlags::NONE {
            return Err(FtuiError::TextFlagNoneWithOther);
        }

        // Only one color can be set.
        if flags
            .intersection(
                TextFlags::COLOR_BLACK |
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
    fn color_f_or_b(flags: TextFlags, b: &'static str, f: &'static str) -> &'static str {
        // if COLOR_BACK flag is not set text will default to foreground color
        if flags.contains(TextFlags::COLOR_BACK) { b } else { f }
    }

    fn resolve_color(flags: TextFlags) -> Option<&'static str> {
        if flags.contains(TextFlags::COLOR_BLACK) {
            Some(Self::color_f_or_b(flags, ansi::ESC_BLACK_B, ansi::ESC_BLACK_F))
        } else if flags.contains(TextFlags::COLOR_RED) {
            Some(Self::color_f_or_b(flags, ansi::ESC_RED_B, ansi::ESC_RED_F))
        } else if flags.contains(TextFlags::COLOR_GREEN) {
            Some(Self::color_f_or_b(flags, ansi::ESC_GREEN_B, ansi::ESC_GREEN_F))
        } else if flags.contains(TextFlags::COLOR_YELLOW) {
            Some(Self::color_f_or_b(flags, ansi::ESC_YELLOW_B, ansi::ESC_YELLOW_F))
        } else if flags.contains(TextFlags::COLOR_BLUE) {
            Some(Self::color_f_or_b(flags, ansi::ESC_BLUE_B, ansi::ESC_BLUE_F))
        } else if flags.contains(TextFlags::COLOR_MAGENTA) {
            Some(Self::color_f_or_b(flags, ansi::ESC_MAGENTA_B, ansi::ESC_MAGENTA_F))
        } else if flags.contains(TextFlags::COLOR_CYAN) {
            Some(Self::color_f_or_b(flags, ansi::ESC_CYAN_B, ansi::ESC_CYAN_F))
        } else if flags.contains(TextFlags::COLOR_WHITE) {
            Some(Self::color_f_or_b(flags, ansi::ESC_WHITE_B, ansi::ESC_WHITE_F))
        } else {
            None
        }
    }

    fn resolve_style(flags: TextFlags) -> Vec<&'static str> {
        let mut style: Vec<&'static str> = vec![];

        if let Some(color) = Self::resolve_color(flags) {
            style.push(color);
        }
        if flags.contains(TextFlags::STYLE_BOLD) {
            style.push(ansi::ESC_BOLD);
        }
        if flags.contains(TextFlags::STYLE_DIM) {
            style.push(ansi::ESC_DIM);
        }
        if flags.contains(TextFlags::STYLE_ITALIC) {
            style.push(ansi::ESC_ITALIC);
        }
        if flags.contains(TextFlags::STYLE_UNDER) {
            style.push(ansi::ESC_UNDERLINE);
        }
        if flags.contains(TextFlags::STYLE_STRIKE) {
            style.push(ansi::ESC_STRIKETHROUGH);
        }

        return style;
    }

    /// Updates the label of the `Text` component.
    ///
    /// # Parameters
    /// - `label`: The new label.
    ///
    /// # Example
    /// ```rust
    /// // Create a `Text` component with the label "Text" and no flags.
    /// let mut text = Text::new("Text", None)?;
    ///
    /// // Update the label to "New Label".
    /// text.set_label("New Label");
    /// ```
    pub fn set_label(&mut self, label: impl Into<String>) {
        let label = label.into();

        self.len = label.graphemes(true).count();
        self.label = label;
    }

    pub(crate) fn set_line(&mut self, line: u16) {
        self.line = line;
    }

    pub(crate) fn line(&self) -> u16 {
        return self.line;
    }

    pub(crate) fn label(&self) -> &String {
        return &self.label;
    }

    pub(crate) fn len(&self) -> usize {
        return self.len;
    }

    pub(crate) fn set_pos(&mut self, pos: u16) {
        self.pos = pos;
    }

    pub(crate) fn pos(&self) -> u16 {
        return self.pos;
    }

    pub(crate) fn flags(&self) -> &TextFlags {
        return &self.flags;
    }

    pub(crate) fn styles(&self) -> &[&'static str] {
        return &self.style;
    }

    pub(crate) fn id(&self) -> u16 {
        self.id
    }

    pub(crate) fn set_id(&mut self, value: u16) {
        self.id = value;
    }
}
