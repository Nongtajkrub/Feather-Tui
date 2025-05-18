use crate::{
    container as con, components as cpn, error::{FtuiError, FtuiResult}, util::ansi
};
use std::io::{self, Write};
use crossterm as ct;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Line {
    ansi: Vec<&'static str>,
    width: usize,
    data: String,
}

impl Line {
    pub fn new(width: u16) -> Line {
        Line {
            ansi: vec![],
            width: width as usize,
            data: " ".repeat(width as usize),
        }
    }

    #[inline]
    pub fn add_ansi(&mut self, value: &'static str) {
        self.ansi.push(value);
    }

    pub fn add_ansi_many(&mut self, value: &[&'static str]) {
        self.ansi.reserve(value.len());
        self.ansi.extend(value.iter().copied());
    }

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
    pub fn edit(&mut self, data: &String, begin: u16) {
        self.data.replace_range(begin as usize..data.len() + begin as usize, data);
    }

    pub fn clear(&mut self) {
        self.fill(' ');
        self.ansi.clear();
    }
}

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
/// use feather_tui as tui;
///
/// tui::ren::ready();
///
/// loop {
///     // Main loop
/// }
///
/// tui::ren::unready();
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
/// use feather_tui as tui;
///
/// tui::ren::ready();
///
/// loop {
///     // Main loop
/// }
///
/// tui::ren::unready();
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
/// use feather_tui as tui;
///
/// // This clear the terminal.
/// tui::ren::clear();
/// ```
pub fn clear() -> FtuiResult<()> {
    print!("{}", ansi::ESC_CLEAR_TERM);

    io::stdout().flush()?;

    Ok(())
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
/// use feather_tui as tui;
///
/// // Create a Renderer with a width of 40 and a height of 20
/// let mut renderer = tui::ren::Renderer::new(40, 20);
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
    /// use feather_tui as tui;
    ///
    /// // Create a Renderer with a width of 40 and a height of 20 characters.
    /// let renderer = tui::ren::Renderer::new(40, 20);
    /// ```
    pub fn new(width: u16, height: u16) -> Renderer {
        Renderer {
            width,
            height,
            lines: Self::make_lines(width, height), 
        }
    }

    pub fn fullscreen() -> FtuiResult<Renderer> {
        let (width, height) = ct::terminal::size()?;

        Ok(Self::new(width, height))
    }

    fn make_lines(width: u16, height: u16) -> Vec<Line> {
        (0..height).map(|_| Line::new(width)).collect()
    }

    /// Caculate the position of a middle-aligned component.
    #[inline] 
    fn calc_middle_align_pos(width: u16, len: usize) -> u16 {
        ((width as f32 - len as f32) / 2.0).round() as u16 
    }

    /// Caculate the position of a left-aligned component.
    #[inline]
    fn calc_right_align_pos(width: u16, len: usize) -> u16 {
        (width as usize - len) as u16
    }

    /// Caculate the position of a left-aligned component.
    #[inline]
    fn calc_left_align_pos() -> u16 {
        0
    }

    #[inline]
    fn calc_bottom_align_pos(height: u16) -> u16 {
        height - 1
    }

    fn ensure_label_inbound(&self, len: usize) -> FtuiResult<()> {
        if len > self.width as usize {
            Err(FtuiError::RendererContainerTooBig)
        } else {
            Ok(())
        }
    }

    fn render_header(&mut self, header: &cpn::Header) -> FtuiResult<()> {
        self.ensure_label_inbound(header.len())?;

        self.lines[0].edit(
            header.label(),
            Self::calc_middle_align_pos(self.width, header.len()));
        self.lines[0].add_ansi(ansi::ESC_GREEN_B);

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
    
    fn render_separator(&mut self, separators: &[cpn::Separator]) {
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

    fn resolve_text_pos(&self, text: &mut cpn::Text) {
        // x pos
        if text.flags().contains(cpn::TextFlags::ALIGN_MIDDLE) {
            text.set_pos(Self::calc_middle_align_pos(self.width, text.len()));
        } else if text.flags().contains(cpn::TextFlags::ALIGN_RIGHT) {
            text.set_pos(Self::calc_right_align_pos(self.width, text.len()));
        } else {
            // default to left alignment
            text.set_pos(Self::calc_left_align_pos());
        } 

        // y pos
        if text.flags().contains(cpn::TextFlags::ALIGN_BOTTOM) {
            text.set_line(Self::calc_bottom_align_pos(self.height));
        }
    }

    fn render_text(&mut self, texts: &mut [cpn::Text]) -> FtuiResult<()> {
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
    /// use feather_tui as tui;
    ///
    /// // Create a `Renderer` with a width of 40 and a height of 20 characters.
    /// let mut renderer = tui::ren::Renderer::new(40, 20);
    ///
    /// // Render the container into the renderer buffer
    /// // (assuming `container` is created elsewhere)
    /// renderer.render(&mut container)?;
    /// ```
    pub fn render(&mut self, container: &mut con::Container) -> FtuiResult<()> {
        if container.component_count() > self.height {
            return Err(FtuiError::RendererContainerTooBig);
        }

        if let Some(header) = container.header().as_ref() {
            self.render_header(header)?;
        }
        self.render_options(container.options())?;
        self.render_text(container.texts_mut())?;
        self.render_separator(container.separators());

        Ok(())
    }
    
    /// Draws the `Renderer` buffer to the terminal.
    ///
    /// # Note
    /// The `render` method must be called at least once before `draw`, as `draw` only
    /// displays the content stored in the `Renderer` buffer.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// // Create a `Renderer` with a width of 40 and a height of 20 characters.
    /// let mut renderer = tui::ren::Renderer::new(40, 20);
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
    pub fn draw(&mut self) -> FtuiResult<()> {
        for (i, line) in self.lines.iter().enumerate() {
            let output = format!(
                "{}{}{}{}",
                line.ansi.concat(),
                line.data, ansi::ESC_COLOR_RESET, ansi::ESC_STYLE_RESET);

            if i == (self.height - 1) as usize {
                print!("{}", output);
            } else {
                println!("{}", output);
            }
        }

        print!("{}", ansi::ESC_CURSOR_HOME);
        io::stdout().flush()?;

        Ok(())
    }

    /// Clears the `Renderer` buffer. This method should be called before rendering.
    ///
    /// # Note
    /// Calling this method before rendering prevents visual artifacts.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// // Create a `Renderer` with a width of 40 and a height of 20 characters.
    /// let mut renderer = tui::ren::Renderer::new(40, 20);
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
    pub fn clear(&mut self) {
        self.lines.iter_mut().for_each(|line| line.clear());
    }

    /// Executes a full rendering cycle in a single method call. This method 
    /// automatically calls `clear`, `render`, and `draw` in sequence.
    ///
    /// # Parameters
    /// - `container`: A mutable reference to the `Container` to be drawn.
    ///
    /// # Returns
    /// - `Ok(())`: Returns nothing.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// // Create a `Renderer` with a width of 40 and a height of 20 characters.
    /// let mut renderer = tui::ren::Renderer::new(40, 20);
    ///
    /// // Standard rendering loop
    /// loop {
    ///     renderer.clear();
    ///     // Render content (assuming `container` is created elsewhere)
    ///     renderer.render(&mut container)?;
    ///     renderer.draw();
    /// }
    ///
    /// // Simplified rendering loop using `simple_draw`
    /// loop {
    ///     // Render and draw in a single step
    ///     renderer.simple_draw(&mut container)?;
    /// }
    /// ```
    pub fn simple_draw(&mut self, container: &mut con::Container) -> FtuiResult<()> {
        self.clear();
        self.render(container)?;
        self.draw()?;

        Ok(())
    }
}
