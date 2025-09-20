use std::io;
use std::io::Write;

use crossterm as ct;

use crate::components as cpn;
use crate::containers::Container;
use crate::containers::Document;
use crate::containers::List;
use crate::containers::Message;
use crate::error::FtuiError;
use crate::error::FtuiResult;
use crate::util::ansi;
use crate::util::mom::Mom;
use crate::util::number as num;

/// A helper class for `Renderer`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    ansi: Vec<&'static str>,
    width: usize,
    data: String,
}

impl Line {
    pub fn new(width: u16) -> Line {
        let width = width as usize;

        Line {
            ansi: Vec::new(),
            width: width,
            data: std::iter::repeat(' ').take(width).collect(),
        }
    }

    #[inline]
    pub fn add_ansi(&mut self, value: &'static str) {
        self.ansi.push(value);
    }

    #[inline]
    pub fn add_ansi_many(&mut self, value: &[&'static str]) {
        self.ansi.reserve(value.len());
        self.ansi.extend(value.iter().copied());
    }

    #[inline]
    pub fn fill(&mut self, c: char) {
        self.data.clear();
        self.data.extend(std::iter::repeat(c).take(self.width));
    }

    pub fn fill_dotted(&mut self, c: char) {
        let repeat_count = (self.width as f32 / 2.0).floor() as usize;

        self.data.clear();

        for _ in 0..repeat_count {
            self.data.push(c);
            self.data.push(' ');
        }
    }

    #[inline]
    pub fn edit(&mut self, data: &str, begin: u16) {
        let begin = begin as usize;
        self.data.replace_range(begin..data.len() + begin, data);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.fill(' ');
        self.ansi.clear();
    }
}

/// Renderable containers.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Renderable<'a> {
    Container(Mom<'a, Container>),
    List(Mom<'a, List>),
    Document(Mom<'a, Document>),
    Message(Mom<'a, Message>),
}

macro_rules! impl_renderable_from {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for Renderable<'a> {
            fn from(value: &'a mut $type) -> Self {
                Renderable::$variant(Mom::Ref(value))
            }
        }

        impl<'a> From<$type> for Renderable<'a> {
            fn from(value: $type) -> Self {
                Renderable::$variant(Mom::Owned(value))
            }
        }
    };
}

impl_renderable_from!(Container, Container);
impl_renderable_from!(List, List);
impl_renderable_from!(Document, Document);
impl_renderable_from!(Message, Message);

impl AsMut<Renderer> for Renderer {
    fn as_mut(&mut self) -> &mut Renderer {
        self
    }
}

/// A `Renderer` is responsible for rendering the UI to the terminal. It takes 
/// a `Container` and displays its components on the screen.
///
/// # Usage
/// A `Renderer` is used to render a `Container` to the terminal. It manages
/// drawing operations and handles the rendering process efficiently.
#[derive(Clone, Debug, PartialEq, Eq)] 
pub struct Renderer {
    width: u16,
    height: u16,
    lines: Vec<Line>,
}

impl Renderer {
    /// Create a Renderer without checking the terminal size.
    fn new_uncheck(width: u16, height: u16) -> Renderer {
        Renderer {
            width,
            height,
            lines: Self::make_lines(width, height),
        }
    }

    /// Constructs a new `Renderer` with the specified width and height.
    ///
    /// # Parameters
    /// - `width`: A `u16` representing the width in characters.
    /// - `height`: A `u16` representing the height in characters.
    ///
    /// # Returns
    /// `Ok(Renderer)`: A `Renderer` instance.
    /// `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a Renderer with a width of 40 and a height of 20 characters.
    /// let renderer = Renderer::new(40, 20)?;
    /// ```
    pub fn new(width: u16, height: u16) -> FtuiResult<Renderer> {
        let (term_width, term_height) = ct::terminal::size()?;

        if width > term_width || height > term_height {
            Err(FtuiError::RendererTerminalToSmall)
        } else {
            Ok(Self::new_uncheck(width, height))
        }
    }

    /// Constructs a new fullscreen `Renderer` (Does not resize).
    ///
    /// # Returns
    /// `Ok(Renderer)`: A `Renderer` instance.
    /// `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a fullscreen Renderer.
    /// let renderer = Renderer::fullscreen()?;
    /// ```
    pub fn fullscreen() -> FtuiResult<Renderer> {
        let (width, height) = ct::terminal::size()?;
        Ok(Self::new_uncheck(width, height))
    }

    /// Constructs a new `Renderer` with the specified height with a fullscreen width.
    ///
    /// # Parameters
    /// - `height`: A `u16` representing the height in characters.
    ///
    /// # Returns
    /// `Ok(Renderer)`: A `Renderer` instance.
    /// `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a Renderer with a fullscreen width and a height of 20 characters.
    /// let renderer = Renderer::fullwidth(20)?;
    /// ```
    pub fn fullwidth(height: u16) -> FtuiResult<Renderer> {
        let (width, term_height) = ct::terminal::size()?;
        
        if height > term_height {
            Err(FtuiError::RendererTerminalToSmall)
        } else {
            Ok(Self::new_uncheck(width, height))
        }
    }

    /// Constructs a new `Renderer` with the specified width with a fullscreen height.
    ///
    /// # Parameters
    /// - `width`: A `u16` representing the width in characters.
    ///
    /// # Returns
    /// `Ok(Renderer)`: A `Renderer` instance.
    /// `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a Renderer with a fullscreen height and a width of 20 characters.
    /// let renderer = Renderer::fullheight(40)?;
    /// ```
    pub fn fullheight(width: u16) -> FtuiResult<Renderer> {
        let (term_width, height) = ct::terminal::size()?;
        
        if width > term_width {
            Err(FtuiError::RendererTerminalToSmall)
        } else {
            Ok(Self::new_uncheck(width, height))
        }
    }

    fn make_lines(width: u16, height: u16) -> Vec<Line> {
        (0..height).map(|_| Line::new(width)).collect()
    }

    // A static method because it often cause borrow checker problem.
    /// Caculate the position of a middle-aligned component.
    #[inline] 
    fn calc_middle_align_pos(width: u16, len: usize) -> u16 {
        ((width as f32 - len as f32) / 2.0).round() as u16 
    }

    // A static method because it often cause borrow checker problem.
    /// Caculate the position of a bottom-aligned component.
    #[inline]
    fn calc_bottom_align_pos(height: u16) -> u16 {
        height - 1
    }

    #[inline]
    fn ensure_label_inbound(&self, len: usize) -> FtuiResult<()> {
        if len > self.width as usize {
            Err(FtuiError::RendererContainerTooBig)
        } else {
            Ok(())
        }
    }

    fn render_header(&mut self, header: &mut cpn::Text) -> FtuiResult<()> {
        self.ensure_label_inbound(header.len())?;
        header.resolve_pos(self.width);

        let line = &mut self.lines[header.line() as usize];

        line.edit(header.label(), header.pos());
        line.add_ansi_many(header.styles());

        Ok(())
    }

    fn render_footer(&mut self, footer: &mut cpn::Text) -> FtuiResult<()> {
        self.ensure_label_inbound(footer.len())?;
        footer.resolve_pos(self.width);
        footer.set_line(Self::calc_bottom_align_pos(self.height));

        let line = &mut self.lines[footer.line() as usize];

        line.edit(footer.label(), footer.pos());
        line.add_ansi_many(footer.styles());

        Ok(())
    }

    fn render_options(&mut self, options: &[cpn::Option]) -> FtuiResult<()> {
        for option in options {
            self.ensure_label_inbound(option.len())?;
            
            let line = &mut self.lines[option.line() as usize];

            line.edit(option.label(), 0);

            if option.selc_on() {
                line.add_ansi(ansi::ESC_BLUE_B);
            }
        }

        Ok(())
    }

    #[inline]
    fn apply_correct_separator(&mut self, separator: &cpn::Separator, c: char) {
        if separator.is_dotted() {
            self.lines[separator.line() as usize].fill_dotted(c);
        } else {
            self.lines[separator.line() as usize].fill(c); 
        }
    }
    
    fn render_separators(&mut self, separators: &[cpn::Separator]) {
        for separator in separators {
            match separator.style() {
                cpn::SeparatorStyle::Solid => 
                    self.apply_correct_separator(separator, '█'), 
                cpn::SeparatorStyle::Medium =>
                    self.apply_correct_separator(separator, '━'),
                cpn::SeparatorStyle::Thin =>
                    self.apply_correct_separator(separator, '─'),
                cpn::SeparatorStyle::Double => 
                    self.apply_correct_separator(separator, '═'),
                cpn::SeparatorStyle::Custom(c) =>
                    self.apply_correct_separator(separator, c),
            }
        }
    }

    fn render_texts(&mut self, texts: &mut [cpn::Text]) -> FtuiResult<()> {
        for text in texts.iter_mut() {
            self.ensure_label_inbound(text.len())?;
            text.resolve_pos(self.width);

            let line = &mut self.lines[text.line() as usize];

            line.edit(text.label(), text.pos());
            line.add_ansi_many(text.styles());
        }

        Ok(())
    }

    fn render_container(&mut self, container: &mut Container) -> FtuiResult<()> {
        if container.component_count() > self.height {
            return Err(FtuiError::RendererContainerTooBig);
        }

        self.clear();

        if let Some(header) = container.header_mut().as_mut() {
            self.render_header(header)?;
        }

        self.render_options(container.option_comps())?;
        self.render_texts(container.text_comps_mut())?;
        self.render_separators(container.separators());

        if let Some(footer) = container.footer_mut().as_mut() {
            self.render_footer(footer)?;
        }

        Ok(())
    }

    fn render_list(&mut self, list: &mut List) -> FtuiResult<()> {
        let offset = list.offset();
        let is_number = list.is_number();
        let skip_top = if list.header().is_some() { 1 } else { 0 };  
        let skip_bottom = if list.footer().is_some() { 1 } else { 0 };
        let max_elements = (self.height - 1) as usize - skip_bottom;
        let num_prefix = if is_number {
            (num::digits(list.len() as u64) + 2) as usize 
        } else { 0 };

        self.clear();

        if let Some(header) = list.header_mut() {
            self.render_header(header)?;
        }

        if let Some(footer) = list.footer_mut() {
            self.render_footer(footer)?;
        }
        
        for (i, elt) in list
            .elements_mut()
            .iter_mut()
            .skip(offset)
            .take(max_elements)
            .enumerate() 
        {
            self.ensure_label_inbound(elt.len())?;
            elt.resolve_pos_custom_len(self.width, elt.len() + num_prefix);

            let line = &mut self.lines[i + skip_top];

            if is_number {
                line.edit(&format!("{}. {}", i + 1 + offset, elt.label()), elt.pos());
            } else {
                line.edit(elt.label(), elt.pos());
            }

            line.add_ansi_many(elt.styles());
        }

        Ok(())
    }

    fn render_document(&mut self, document: &mut Document) -> FtuiResult<()> {
        let len = document.data().len();
        let wrap_n = (len as f64 / self.width as f64).ceil() as usize;
        let width = self.width as usize;
        let height = self.height as usize;
        let skip_top = if document.header().is_some() { 1 } else { 0 };
        let skip_bottom = if document.footer().is_some() { 1 } else { 0 };
        let max_lines = (height - 1) - skip_bottom;
        document.offset_ensure_in_bound(wrap_n - 1);
        let offset = document.offset();

        self.clear();

        if let Some(header) = document.header_mut().as_mut() {
            self.render_header(header)?;
        }

        for i in (0..wrap_n - offset).take(max_lines) {
            let line = &mut self.lines[i + skip_top];
            let begin = (i + offset) * width;
            let end = (begin + len.min(width)).min(len);

            line.edit(&document.data()[begin..end], 0);
            line.add_ansi_many(document.style());
        }

        if let Some(footer) = document.footer_mut().as_mut() {
            self.render_footer(footer)?;
        }

        Ok(())
    }

    fn render_message(&mut self, message: &Message) -> FtuiResult<()> {
        self.ensure_label_inbound(message.len())?;
        let line = (self.height as f32 / 2.0).round() as usize;
        let x_pos = Self::calc_middle_align_pos(self.width, message.len());
        let ansi = message.style().to_ansi();

        self.lines[line].edit(message.message(), x_pos);
        self.lines[line].add_ansi_many(ansi);

        self.lines.get_mut(line - 1).map(|line| {
            line.clear();
            line.add_ansi_many(ansi);
        });
        self.lines.get_mut(line + 1).map(|line| {
            line.clear();
            line.add_ansi_many(ansi);
        });

        Ok(())
    }

    fn render<'a>(&mut self, renderable: impl Into<Renderable<'a>>) -> FtuiResult<()> {
        match renderable.into() {
            Renderable::Container(ref mut container) =>
                self.render_container(container.as_mut())?,
            Renderable::List(ref mut list) =>
                self.render_list(list.as_mut())?,
            Renderable::Document(ref mut document) =>
                self.render_document(document.as_mut())?,
            Renderable::Message(ref mut message) =>
                self.render_message(message.as_mut())?,
        }

        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        self.lines.iter_mut().for_each(|line| line.clear());
    }

    fn to_string(&self) -> String {
        let mut buf = String::with_capacity(((self.height * self.width) + 40) as usize);
        let reset_suffix = format!("{}{}", ansi::ESC_COLOR_RESET, ansi::ESC_STYLE_RESET);

        buf.push_str(ansi::_ESC_CLEAR_TERM);

        for (i, line) in self.lines.iter().enumerate() {
            let have_ansi = !line.ansi.is_empty();

            buf.push_str(&line.ansi.concat());

            // Exclude lines containing only whitesapce unless it have ANSIs.
            if !line.data.trim().is_empty() || have_ansi {
                buf.push_str(if have_ansi { &line.data } else { line.data.trim() });
            }

            // Only include the ANSI reset suffix if the line have ANSIs.
            if have_ansi {
                buf.push_str(&reset_suffix);
            }

            if i != (self.height - 1) as usize {
                buf.push('\n');
            }
        }

        buf.push_str(ansi::ESC_CURSOR_HOME);
        buf
    }
    
    /// Draws the `Renderer` buffer to the terminal.
    ///
    /// # Note
    /// The `render` method must be called at least once before `draw`, as `draw` only
    /// displays the content stored in the `Renderer` buffer.
    ///
    /// # Example
    /// ```rust
    /// // Create a `Renderer` with a width of 40 and a height of 20 characters.
    /// let mut renderer = Renderer::new(40, 20);
    ///
    /// // Render the container into the renderer buffer
    /// // (assuming `container` is created elsewhere)
    /// renderer.render(&mut container)?;
    ///
    /// // Draw the rendered content to the terminal
    /// renderer.draw();
    ///
    /// // The draw method can be called again without re-rendering,
    /// // but changes won't be reflected unless `render` is called.
    /// renderer.draw();
    /// ```
    pub fn draw<'a>(&mut self, renderable: impl Into<Renderable<'a>>) -> FtuiResult<()> {
        self.render(renderable)?;

        let mut stdout = io::stdout().lock();
        stdout.write_all(self.to_string().as_bytes())?;
        stdout.flush()?;

        Ok(())
    }
}
