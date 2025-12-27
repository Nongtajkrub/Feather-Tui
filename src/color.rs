use crate::util::ansi;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colors {
    BlackFore,
    RedFore,
    GreenFore,
    YellowFore,
    BlueFore,
    MagentaFore,
    CyanFore,
    WhiteFore,
    BlackBack,
    RedBack,
    GreenBack,
    YellowBack,
    BlueBack,
    MagentaBack,
    CyanBack,
    WhiteBack,
}

impl Colors {
    pub(crate) fn to_ansi(&self) -> &'static str {
        use ansi::*;

        match self {
            Self::BlackFore => ESC_BLACK_F,
            Self::RedFore => ESC_RED_F,
            Self::GreenFore => ESC_GREEN_F,
            Self::YellowFore => ESC_YELLOW_F,
            Self::BlueFore => ESC_BLACK_F,
            Self::MagentaFore => ESC_MAGENTA_F,
            Self::CyanFore => ESC_CYAN_F,
            Self::WhiteFore => ESC_WHITE_F,
            Self::BlackBack => ESC_BLACK_B,
            Self::RedBack => ESC_RED_B,
            Self::GreenBack => ESC_GREEN_B,
            Self::YellowBack => ESC_YELLOW_B,
            Self::BlueBack => ESC_BLACK_B,
            Self::MagentaBack => ESC_MAGENTA_B,
            Self::CyanBack => ESC_CYAN_B,
            Self::WhiteBack => ESC_WHITE_B,
        }
    }
}
