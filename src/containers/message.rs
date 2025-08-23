use crate::util::ansi;

pub(crate) const MSG_INFO_ANSI: [&'static str; 2] = [ansi::ESC_WHITE_B, ansi::ESC_BLACK_F];
pub(crate) const MSG_WARN_ANSI: [&'static str; 1] = [ansi::ESC_YELLOW_B];
pub(crate) const MSG_ERRO_ANSI: [&'static str; 2] = [ansi::ESC_RED_B, ansi::ESC_BOLD];

/// Represents the visual style of a `Message`, typically used to convey different
/// levels of importance or severity.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStyle {
    /// Informational message: white background with black foreground.
    Info,
    /// Warning message: yellow background with white foreground.
    Warning,
    /// Error message: red background with white foreground and bold text.
    Error,
}

impl MessageStyle {
    pub(crate) fn to_ansi(self) -> &'static[&'static str] {
        match self {
            MessageStyle::Info => &MSG_INFO_ANSI,
            MessageStyle::Warning => &MSG_WARN_ANSI,
            MessageStyle::Error => &MSG_ERRO_ANSI,
        }
    }
}

/// A specialized variant of `Container` used to display a centered message on a
/// `Renderer`. The appearance of the message is defined by the `MessageStyle` enum.
/// 
/// # Usage
/// Use this to present informational messages, warnings, or errors to the user in
/// a visually distinct way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    message: String,
    style: MessageStyle,
}

impl Message {
    /// Creates a new `Message` with the given content and style.
    ///
    /// # Parameters
    /// - `message`: A type that impl `ToString`, representing the message content.
    /// - `style`: A `MessageStyle` indicating how the message should be displayed.
    ///
    /// # Returns
    /// A new `Message` instance with the specified content and style.
    ///
    /// # Example
    /// ```rust
    /// let _ = Message::new("Information!", MessageStyle::Info);
    /// ```
    pub fn new(message: impl ToString, style: MessageStyle) -> Self {
        Self {
            message: message.to_string(),
            style: style,
        }
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn len(&self) -> usize {
        self.message.len()
    }

    pub(crate) fn style(&self) -> MessageStyle {
        self.style
    }
}
