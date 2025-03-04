use crate::tui::util::ansi;
use crate::tui::con;
use crate::tui::cpn; 
use crate::tui::emg;

const BG_CHAR: &str = " ";

struct Line {
    ansi: std::option::Option<String>,
    width: u16,
    data: String,
}

impl Line {
    pub fn new(width: u16) -> Line {
        Line {
            ansi: None,
            width,
            data: Self::make_empty_line(width),
        }
    }

    fn make_empty_line(width: u16) -> String {
        BG_CHAR.repeat(width as usize)
    }

    pub fn set_ansi(&mut self, ansi: String) {
        self.ansi = Some(ansi);
    }

    pub fn edit(&mut self, data: &String, begin: u16) {
        self.data.replace_range(begin as usize..data.len() + begin as usize, data);
    }

    pub fn clear(&mut self) {
        self.data = Self::make_empty_line(self.width);
        self.ansi = None;
    }
}

pub struct Renderer {
    width: u16,
    height: u16,
    lines: Vec<Line>,
}

pub fn ready() {
    print!(
        "{}{}{}",
        ansi::ESC_CLEAR_TERM, ansi::ESC_CURSOR_HOME, ansi::ESC_CURSOR_HOME);
}

pub fn unready() {
    print!(
        "{}{}{}",
        ansi::ESC_CLEAR_TERM, ansi::ESC_CURSOR_HOME, ansi::ESC_CURSOR_SHOW);
}


impl Renderer {
    pub fn new(width: u16, height: u16) -> Renderer {
        Renderer {
            width,
            height,
            lines: Self::make_lines(width, height), 
        }
    }

    fn make_lines(width: u16, height: u16) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::with_capacity(height as usize);

        for _ in 0..height {
            lines.push(Line::new(width));
        }

        return lines;
    }

    fn render_header(&mut self, header: &cpn::hed::Header) {
        let pos: u16 = 
            ((self.width as f32 - header.len() as f32) / 2.0).round() as u16; 

        let line = &mut self.lines[0];

        line.edit(header.label(), pos);
        line.set_ansi(String::from(ansi::ESC_GREEN_B));
    }

    fn render_options(&mut self, options: &Vec<cpn::opt::Option>) {
        for option in options.iter() {
            let line = &mut self.lines[option.line() as usize];

            line.edit(option.label(), 0);

            if option.selc_on() {
                line.set_ansi(String::from(ansi::ESC_BLUE_B));
            }
        }
    }

    fn render_text(&mut self, texts: &Vec<cpn::txt::Text>) {
        for text in texts.iter() {
            self.lines[text.line() as usize].edit(text.label(), 0);
        }
    }

    pub fn render(&mut self, container: &con::Container) {
        self.render_header(container.header().as_ref().expect(emg::NO_HEADER_ERRMSG));
        self.render_options(container.options());
        self.render_text(container.texts());
    }

    pub fn draw(&mut self) {
        for line in self.lines.iter() {
            if let Some(ansi) = &line.ansi {
                println!("{}{}{}", ansi, line.data, ansi::ESC_RESET);
            } else {
                println!("{}", line.data)
            }
        }

        print!("{}", ansi::ESC_CURSOR_HOME);
    }

    pub fn clear(&mut self) {
        for line in self.lines.iter_mut() {
            line.clear();
        }
    }
}
