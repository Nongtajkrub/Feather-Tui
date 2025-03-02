use crate::termui::util::ansi::{self, ANSI_ESC_CURSOR_SHOW};
use crate::termui::container;
use crate::termui::components;

const BG_CHAR: &str = " ";
const NO_HEADER_ERRMSG: &str = "Renderer: Header is not set!";

struct Line {
    ansi: std::option::Option<String>,
    data: String,
}

impl Line {
    pub fn new(width: u16) -> Line {
        Line {
            ansi: None,
            data: BG_CHAR.repeat(width as usize)
        }
    }

    pub fn set_ansi(&mut self, ansi: String) {
        self.ansi = Some(ansi);
    }

    pub fn edit(&mut self, data: &String, begin: u16) {
        self.data.replace_range(begin as usize..data.len() + begin as usize, data);
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
        ansi::ANSI_ESC_CLEAR_TERM, 
        ansi::ANSI_ESC_CURSOR_HOME, ansi::ANSI_ESC_CURSOR_HOME);
}

pub fn unready() {
    print!(
        "{}{}{}",
        ansi::ANSI_ESC_CLEAR_TERM,
        ansi::ANSI_ESC_CURSOR_HOME, ANSI_ESC_CURSOR_SHOW);
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

    fn render_header(&mut self, header: &components::header::Header) {
        let pos: u16 = 
            ((self.width as f32 - header.len() as f32) / 2.0).round() as u16; 

        let line = &mut self.lines[0];

        line.edit(header.label(), pos);
        line.set_ansi(String::from(ansi::ANSI_ESC_GREEN_B));
    }

    fn render_options(&mut self, options: &Vec<components::option::Option>) {
        for option in options.iter() {
            let line = &mut self.lines[option.line() as usize];

            line.edit(option.label(), 0);

            if option.selc_on() {
                line.set_ansi(String::from(ansi::ANSI_ESC_BLUE_B));
            }
        }
    }

    fn render_text(&mut self, texts: &Vec<components::text::Text>) {
        for text in texts.iter() {
            self.lines[text.line() as usize].edit(text.label(), 0);
        }
    }

    pub fn render(&mut self, container: &container::Container) {
        self.render_header(container.header().as_ref().expect(NO_HEADER_ERRMSG));
        self.render_options(container.options());
        self.render_text(container.texts());
    }

    pub fn draw(&mut self) {
        for line in self.lines.iter() {
            if let Some(ansi) = &line.ansi {
                println!("{}{}{}", ansi, line.data, ansi::ANSI_ESC_RESET);
            } else {
                println!("{}", line.data)
            }
        }
    }
}
