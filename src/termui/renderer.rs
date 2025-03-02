use crate::termui::container::Container;
use crate::termui::components::{
    header::Header,
    option::Option,
    text::Text,
};

const BG_CHAR: &'static str = " ";
const NO_HEADER_ERRMSG: &'static str = "Renderer: Header is not set!";

struct Line {
    ascii: std::option::Option<String>,
    data: String,
}

impl Line {
    pub fn new(width: u16) -> Line {
        Line {
            ascii: None,
            data: BG_CHAR.repeat(width as usize)
        }
    }

    pub fn set_ascii(&mut self, ascii: String) {
        self.ascii = Some(ascii);
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

    fn render_header(&mut self, header: &Header) {
        let pos: u16 = 
            ((self.width as f32 - header.len() as f32) / 2.0).round() as u16; 

        self.lines[0].edit(header.label(), pos);
    }

    fn render_options(&mut self, options: &Vec<Option>) {
        for option in options.iter() {
            self.lines[option.line() as usize].edit(option.label(), 0);
        }
    }

    fn render_text(&mut self, texts: &Vec<Text>) {
        for text in texts.iter() {
            self.lines[text.line() as usize].edit(text.label(), 0);
        }
    }

    pub fn render(&mut self, container: &Container) {
        // TODO: implement rendering options and texts
        self.render_header(container.header().as_ref().expect(NO_HEADER_ERRMSG));
        self.render_options(container.options());
        self.render_text(container.texts());
    }

    pub fn draw(&mut self) {
        for line in self.lines.iter() {
            if let Some(ascii) = &line.ascii {
                println!("{}{}", ascii, line.data);
            } else {
                println!("{}", line.data)
            }
        }
    }
}
