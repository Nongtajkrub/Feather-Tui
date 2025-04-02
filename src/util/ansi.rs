// foreground text color
pub(crate) const ESC_BLACK_F: &str = "\x1b[30m";
pub(crate) const ESC_RED_F: &str = "\x1b[31m";
pub(crate) const ESC_GREEN_F: &str = "\x1b[32m";
pub(crate) const ESC_YELLOW_F: &str = "\x1b[33m";
pub(crate) const ESC_BLUE_F: &str = "\x1b[34m";
pub(crate) const ESC_MAGENTA_F: &str = "\x1b[35m";
pub(crate) const ESC_CYAN_F: &str = "\x1b[36m";
pub(crate) const ESC_WHITE_F: &str = "\x1b[37m";

// background text color
pub(crate) const ESC_BLACK_B: &str = "\x1b[40m";
pub(crate) const ESC_RED_B: &str = "\x1b[41m";
pub(crate) const ESC_GREEN_B: &str = "\x1b[42m";
pub(crate) const ESC_YELLOW_B: &str = "\x1b[43m";
pub(crate) const ESC_BLUE_B: &str = "\x1b[44m";
pub(crate) const ESC_MAGENTA_B: &str = "\x1b[45m";
pub(crate) const ESC_CYAN_B: &str = "\x1b[46m";
pub(crate) const ESC_WHITE_B: &str = "\x1b[47m";
pub(crate) const ESC_COLOR_RESET: &str = "\x1b[0m";

// text styles
pub(crate) const _ESC_BOLD: &str = "\x1b[1m";
pub(crate) const _ESC_DIM: &str = "\x1b[2m";
pub(crate) const _ESC_ITALIC: &str = "\x1b[3m";
pub(crate) const _ESC_UNDERLINE: &str = "\x1b[4m";
pub(crate) const _ESC_BLINK: &str = "\x1b[5m";
pub(crate) const _ESC_REVERSED: &str = "\x1b[7m";
pub(crate) const _ESC_HIDDEN: &str = "\x1b[8m";
pub(crate) const _ESC_STRIKETHROUGH: &str = "\x1b[9m";
pub(crate) const _ESC_DOUBLE_UNDERLINE: &str = "\x1b[21m";
pub(crate) const _ESC_OVERLINE: &str = "\x1b[53m";
pub(crate) const _ESC_STYLE_RESET: &str = "\033[0m";

// cursors
pub(crate) const ESC_CURSOR_HOME: &str = "\x1b[H";
pub(crate) const ESC_CURSOR_HIDE: &str = "\x1b[?25l";
pub(crate) const ESC_CURSOR_SHOW: &str = "\x1b[?25h";

// terminal
pub(crate) const ESC_CLEAR_TERM: &str = "\x1b[2J";
