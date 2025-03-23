use crate::util::ansi;

bitflags::bitflags! {
    #[derive(Clone)]
    pub struct TextFlags: u16 {
        const NONE          = 0;

        // alignment
        const ALIGN_CENTER  = 1 << 0;
        const ALIGN_LEFT    = 1 << 1;
        const ALIGN_RIGHT   = 1 << 2;
        const ALIGN_BOTTOM  = 1 << 3;

        // color settings
        const COLOR_BACK    = 1 << 4;
        const COLOR_FORE    = 1 << 5;

        // colors
        const COLOR_BLACK   = 1 << 6;
        const COLOR_RED     = 1 << 7;
        const COLOR_GREEN   = 1 << 8;
        const COLOR_YELLOW  = 1 << 9;
        const COLOR_BLUE    = 1 << 10;
        const COLOR_MAGENTA = 1 << 11;
        const COLOR_CYAN    = 1 << 12;
        const COLOR_WHITE   = 1 << 13;

        // no COLOR_..._FORE because if COLOR_BACK flag is not set text will
        // default to foreground color
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

/// A UI component representing a text element in a `Container`. `Text` components
/// are displayed in the order they are added to the `Container`. They can be
/// customized using `TextFlags` to adjust alignment, color, and other styling options.
///
/// # Usage
///
/// The `Text` component is used within a `Container` to display static text elements.  
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

pub struct Text {
    label: String,
    line: u16,
    flags: TextFlags,
    pos_resolve: bool,
    pos: u16,
    color: String,
}

impl Text {
    pub fn new(label: &str, flags: TextFlags) -> Self {
        Text {
            label: label.to_string(),
            line: 0,
            flags: flags.clone(),
            pos_resolve: false,
            pos: 0,
            color: Self::resolve_color(flags),
        }
    }

    #[inline]
    fn color_f_or_b(flags: TextFlags, b: &str, f: &str) -> String {
        // if COLOR_BACK flag is not set text will default to foreground color
        if flags.contains(TextFlags::COLOR_BACK) { 
            String::from(b) 
        } else {
            String::from(f)
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
