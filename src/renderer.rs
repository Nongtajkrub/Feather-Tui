use crate::{
    components::{self as cpn}, container::{Container, List, Document},
    error::{FtuiError, FtuiResult}, util::{ansi, number as num}};
use std::io::{self, Write};
use crossterm as ct;

/// Prepares the terminal for rendering. This function is typically used in 
/// conjunction with `unready()`, similar to how `malloc` pairs with `free`.
/// It clears the terminal screen and moves the cursor to the home position,
/// then hide it. This ensure a clean state before rendering.
///
/// # Returns
/// - `Ok(())` if the operation completes successfully.
/// - `Err(FtuiError)` if an error occurs during the operation.
///
/// # Example
/// ```rust
/// ready();
///
/// loop {
///     // Main loop
/// }
///
/// unready();
/// ```
pub fn ready() -> FtuiResult<()> {
    print!(
        "{}{}{}",
        ansi::ESC_CLEAR_TERM, ansi::ESC_CURSOR_HOME, ansi::ESC_CURSOR_HIDE);

    io::stdout().flush()?;

    Ok(())
}

/// Restores the terminal state after rendering is done. This function is 
/// typically used in conjunction with `ready()`, similar to how `malloc` pairs
/// with `free`. It clears the terminal screen and moves the cursor to the home
/// position, then unhide it. This ensure a clean state before rendering.
///
/// # Returns
/// - `Ok(())` if the operation completes successfully.
/// - `Err(FtuiError)` if an error occurs during the operation.
/// 
/// # Example
/// ```rust
/// ready();
///
/// loop {
///     // Main loop
/// }
///
/// unready();
/// ```
pub fn unready() -> FtuiResult<()> {
    print!(
        "{}{}{}",
        ansi::ESC_CLEAR_TERM, ansi::ESC_CURSOR_HOME, ansi::ESC_CURSOR_SHOW);

    io::stdout().flush()?;

    Ok(())
}

/// Clears the terminal screen. This function clears the **terminal screen**, 
/// which is different from `Renderer::clear` that clears only the renderer
/// buffer.
///
/// # Returns
/// - `Ok(())` if the operation completes successfully.
/// - `Err(FtuiError)` if an error occurs during the operation.
///
/// # Example
/// ```rust
/// // This clear the terminal.
/// clear();
/// ```
pub fn clear() -> FtuiResult<()> {
    print!("{}", ansi::ESC_CLEAR_TERM);

    io::stdout().flush()?;

    Ok(())
}

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
    Container(&'a mut Container),
    List(&'a mut List),
    Document(&'a mut Document),
}

impl<'a> From<&'a mut Container> for Renderable<'a> {
    fn from(value: &'a mut Container) -> Self {
        Renderable::Container(value)
    }
} 

impl<'a> From<&'a mut List> for Renderable<'a> {
    fn from(value: &'a mut List) -> Self {
        Renderable::List(value)
    }
} 

impl<'a> From<&'a mut Document> for Renderable<'a> {
    fn from(value: &'a mut Document) -> Self {
        Renderable::Document(value)
    }
} 

/// A `Renderer` is responsible for rendering the UI to the terminal. It takes 
/// a `Container` and displays its components on the screen.
///
/// # Usage
///
/// A `Renderer` is used to render a `Container` to the terminal. It manages
/// drawing operations and handles the rendering process efficiently.
///
/// # Derives
///
/// `Clone`, `Debug`, `PartialEq`, `Eq`
///
/// # Example
/// ```rust
/// // Create a Renderer with a width of 40 and a height of 20
/// let mut renderer = Renderer::new(40, 20);
///
/// // Clear the buffer before rendering
/// renderer.clear();
///
/// // Render the container (assuming `container` is created elsewhere)
/// renderer.render(&container);
///
/// // Draw the final output to the terminal
/// renderer.draw();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)] 
pub struct Renderer {
    width: u16,
    height: u16,
    lines: Vec<Line>,
    bounds: bool,
}

impl Renderer {
    /// Constructs a new `Renderer` with the specified width and height.
    ///
    /// # Parameters
    /// - `width`: A `u16` representing the width in characters.
    /// - `height`: A `u16` representing the height in characters.
    ///
    /// # Returns
    /// A `Renderer` instance.
    ///
    /// # Example
    /// ```rust
    /// // Create a Renderer with a width of 40 and a height of 20 characters.
    /// let renderer = Renderer::new(40, 20);
    /// ```
    pub fn new(width: u16, height: u16) -> Renderer {
        Renderer {
            width,
            height,
            lines: Self::make_lines(width, height),
            bounds: true,
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
        Ok(Self::new(width, height))
    }

    pub fn disble_bounds(&mut self) {
        self.bounds = false;
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
    /// Caculate the position of a left-aligned component.
    #[inline]
    fn calc_right_align_pos(width: u16, len: usize) -> u16 {
        (width as usize - len) as u16
    }

    // A static method because it often cause borrow checker problem.
    /// Caculate the position of a left-aligned component.
    #[inline]
    fn calc_left_align_pos() -> u16 {
        0
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

    /// Resolve a `Text` component position with a custom specified lenght.
    fn resolve_text_pos_with_len(&self, text: &mut cpn::Text, len: usize) {
        // x pos
        if text.flags().contains(cpn::TextFlags::ALIGN_MIDDLE) {
            text.set_pos(Self::calc_middle_align_pos(self.width, len));
        } else if text.flags().contains(cpn::TextFlags::ALIGN_RIGHT) {
            text.set_pos(Self::calc_right_align_pos(self.width, len));
        } else {
            // default to left alignment
            text.set_pos(Self::calc_left_align_pos());
        } 
    }

    #[inline]
    fn resolve_text_pos(&self, text: &mut cpn::Text) {
        self.resolve_text_pos_with_len(text, text.len());
    }

    fn render_header(&mut self, header: &mut cpn::Text) -> FtuiResult<()> {
        self.ensure_label_inbound(header.len())?;
        self.resolve_text_pos(header);

        let line = &mut self.lines[header.line() as usize];

        line.edit(header.label(), header.pos());
        line.add_ansi_many(header.styles());

        Ok(())
    }

    fn render_footer(&mut self, footer: &mut cpn::Text) -> FtuiResult<()> {
        self.ensure_label_inbound(footer.len())?;
        self.resolve_text_pos(footer);
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
            self.resolve_text_pos(text);

            let line = &mut self.lines[text.line() as usize];

            line.edit(text.label(), text.pos());
            line.add_ansi_many(text.styles());
        }

        Ok(())
    }

    /// Renders a `Container` into the `Renderer` buffer without drawing to the terminal.
    ///
    /// # Parameters
    /// - `container`: A mutable reference to the `Container` to be rendered.
    ///
    /// # Note
    ///  - This method only updates the internal buffer. 
    ///  - To display the rendered content, call the `draw` method.
    ///  - You should use the `clear` method to clear the buffer first.
    ///
    /// # Returns
    /// - `Ok(())`: Returns nothing.
    /// - `Err(FtuiError)`: Returns an error.
    /// 
    /// # Example
    /// ```rust
    /// // Create a `Renderer` with a width of 40 and a height of 20 characters.
    /// let mut renderer = Renderer::new(40, 20);
    ///
    /// // Render the container into the renderer buffer
    /// // (assuming `container` is created elsewhere)
    /// renderer.render(&mut container)?;
    /// ```
    pub(crate) fn render_container(&mut self, container: &mut Container) -> FtuiResult<()> {
        if container.component_count() > self.height {
            return Err(FtuiError::RendererContainerTooBig);
        }

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

    /// Renders a `List` into the `Renderer` buffer without drawing to the terminal.
    ///
    /// # Parameters
    /// - `list`: A mutable reference to the `List` to be rendered.
    ///
    /// # Note
    ///  - This method only updates the internal buffer. 
    ///  - To display the rendered content, call the `draw` method.
    ///  - You should use the `clear` method to clear the buffer first.
    ///
    /// # Returns
    /// - `Ok(())`: Returns nothing.
    /// - `Err(FtuiError)`: Returns an error.
    /// 
    /// # Example
    /// ```rust
    /// // Create a `Renderer` with a width of 40 and a height of 20 characters.
    /// let mut renderer = Renderer::new(40, 20);
    ///
    /// // Render a list into the renderer buffer
    /// // (assuming `list` is created elsewhere)
    /// renderer.render_list(&mut list)?;
    /// ```
    pub(crate) fn render_list(&mut self, list: &mut List) -> FtuiResult<()> {
        let offset = list.offset();
        let is_number = list.is_number();
        let skip_top = if list.header().is_some() { 1 } else { 0 };  
        let skip_bottom = if list.footer().is_some() { 1 } else { 0 };
        let max_elements = (self.height - 1) as usize - skip_bottom;
        let num_prefix = if is_number {
            (num::digits(list.len() as u64) + 2) as usize 
        } else { 0 };

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
            self.resolve_text_pos_with_len(elt, elt.len() + num_prefix);

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
        let width = self.width as usize;
        let height = self.height as usize;
        let len = document.data().len();
        let wrap_n = (len as f64 / self.width as f64).ceil() as usize;
        let skip_top = if document.header().is_some() { 1 } else { 0 };
        let skip_bottom = if document.footer().is_some() { 1 } else { 0 };
        let max_lines = (height - 1) - skip_bottom; 

        if let Some(header) = document.header_mut().as_mut() {
            self.render_header(header)?;
        }

        for i in (0..wrap_n).take(max_lines) {
            let line = &mut self.lines[i + skip_top];
            let begin = i * width;
            let end = (begin + len.min(width)).min(len);

            line.edit(&document.data()[begin..end], 0);
            line.add_ansi_many(document.style());
        }

        if let Some(footer) = document.footer_mut().as_mut() {
            self.render_footer(footer)?;
        }

        Ok(())
    }

    pub(crate) fn render<'a>(
        &mut self, renderable: impl Into<Renderable<'a>>
    ) -> FtuiResult<()> {
        match renderable.into() {
            Renderable::Container(container) => self.render_container(container)?,
            Renderable::List(list) => self.render_list(list)?,
            Renderable::Document(document) => self.render_document(document)?,
        }

        Ok(())
    }

    /// Clears the `Renderer` buffer. This method should be called before rendering.
    ///
    /// # Note
    /// Calling this method before rendering prevents visual artifacts.
    ///
    /// # Example
    /// ```rust
    /// // Create a `Renderer` with a width of 40 and a height of 20 characters.
    /// let mut renderer = Renderer::new(40, 20);
    ///
    /// // Rendering loop
    /// loop {
    ///     // Clear the `Renderer` buffer to remove previous frame content
    ///     renderer.clear();
    ///
    ///     // Render the container into the renderer buffer
    ///     // (assuming `container` is created elsewhere)
    ///     renderer.render(&mut container)?;
    ///
    ///     // Draw the rendered content to the terminal
    ///     renderer.draw();
    /// }
    /// ```
    #[inline]
    pub(crate) fn clear(&mut self) {
        self.lines.iter_mut().for_each(|line| line.clear());
    }

    fn to_string(&self) -> String {
        let mut buf = String::with_capacity(((self.height * self.width) + 40) as usize);
        let reset_suffix = format!("{}{}", ansi::ESC_COLOR_RESET, ansi::ESC_STYLE_RESET);

        for (i, line) in self.lines.iter().enumerate() {
            buf.push_str(&line.ansi.concat());
            buf.push_str(&line.data);
            buf.push_str(&reset_suffix);

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
        self.clear();
        self.render(renderable)?;

        let mut stdout = io::stdout().lock();
        stdout.write_all(self.to_string().as_bytes())?;
        stdout.flush()?;

        Ok(())
    }
}
