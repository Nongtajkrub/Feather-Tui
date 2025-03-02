// foreground text color
pub const ANSI_ESC_BLACK_F: &str = "\x1b[30m";
pub const ANSI_ESC_RED_F: &str = "\x1b[31m";
pub const ANSI_ESC_GREEN_F: &str = "\x1b[32m";
pub const ANSI_ESC_YELLOW_F: &str = "\x1b[33m";
pub const ANSI_ESC_BLUE_F: &str = "\x1b[34m";
pub const ANSI_ESC_MAGENTA_F: &str = "\x1b[35m";
pub const ANSI_ESC_CYAN_F: &str = "\x1b[36m";
pub const ANSI_ESC_WHITE_F: &str = "\x1b[37m";

// background text color
pub const ANSI_ESC_BLACK_B: &str = "\x1b[40m";
pub const ANSI_ESC_RED_B: &str = "\x1b[41m";
pub const ANSI_ESC_GREEN_B: &str = "\x1b[42m";
pub const ANSI_ESC_YELLOW_B: &str = "\x1b[43m";
pub const ANSI_ESC_BLUE_B: &str = "\x1b[44m";
pub const ANSI_ESC_MAGENTA_B: &str = "\x1b[45m";
pub const ANSI_ESC_CYAN_B: &str = "\x1b[46m";
pub const ANSI_ESC_WHITE_B: &str = "\x1b[47m";

// text styles
pub const ANSI_ESC_BOLD: &str = "\x1b[1m";
pub const ANSI_ESC_DIM: &str = "\x1b[2m";
pub const ANSI_ESC_ITALIC: &str = "\x1b[3m";
pub const ANSI_ESC_UNDERLINE: &str = "\x1b[4m";
pub const ANSI_ESC_BLINK: &str = "\x1b[5m";
pub const ANSI_ESC_REVERSED: &str = "\x1b[7m";
pub const ANSI_ESC_HIDDEN: &str = "\x1b[8m";
pub const ANSI_ESC_STRIKETHROUGH: &str = "\x1b[9m";
pub const ANSI_ESC_DOUBLE_UNDERLINE: &str = "\x1b[21m";
pub const ANSI_ESC_OVERLINE: &str = "\x1b[53m";

// reset Code
pub const ANSI_ESC_RESET: &str = "\x1b[0m";

// cursors
pub const ANSI_ESC_CURSOR_HOME: &str = "\x1b[H";
pub const ANSI_ESC_CURSOR_HIDE: &str = "\x1b[?25l";
pub const ANSI_ESC_CURSOR_SHOW: &str = "\x1b[?25h";

// terminal
pub const ANSI_ESC_CLEAR_TERM: &str = "\x1b[2J";
