use crate::util::ansi;

pub(crate) const MSG_INFO_ANSI: [&'static str; 2] = [ansi::ESC_WHITE_B, ansi::ESC_BLACK_F];
pub(crate) const MSG_WARN_ANSI: [&'static str; 1] = [ansi::ESC_YELLOW_B];
pub(crate) const MSG_ERRO_ANSI: [&'static str; 2] = [ansi::ESC_RED_B, ansi::ESC_BOLD];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStyle {
    Info,
    Warning,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    message: String,
    style: MessageStyle,
}

impl Message {
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
