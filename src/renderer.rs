use std::io::Write;

use crate::components as cpn;
use crate::error::FtuiError;
use crate::error::FtuiResult;
use crate::util::ansi;
use crate::util::Dimension;
use crate::util::RenderableMut;

const WHITESPACE_CHAR: char = ' ';

/// A helper class for `Renderer`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Line {
    ansi: Vec<&'static str>,
    width: usize,
    data: Vec<char>,
}

impl Line {
    pub fn new(width: u16) -> Line {
        let width = width as usize;

        Line {
            ansi: Vec::new(),
            width: width,
            data: std::iter::repeat(WHITESPACE_CHAR).take(width).collect(),
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
            self.data.push(WHITESPACE_CHAR);
        }
    }

    pub fn edit(&mut self, data: &str, begin: u16) {
        let begin = begin as usize;

        for (i, c) in data.chars().enumerate() {
            self.data[begin + i] = c;
        }
    }

    pub fn edit_iter<I>(&mut self, data_iter: I, begin: u16) 
    where
        I: Iterator<Item = char>
    {
        let begin = begin as usize;

        for (i, c) in data_iter.enumerate() {
            self.data[begin + i] = c;
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.fill(WHITESPACE_CHAR);
        self.ansi.clear();
    }

    #[inline]
    pub fn as_string(&self) -> String {
        self.data.iter().collect()
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
    pub fn new(dimension: Dimension) -> Renderer {
        Renderer {
            width: dimension.width(),
            height: dimension.height(),
            lines: Self::make_lines(dimension.width(), dimension.height()),
        }
    }

    fn make_lines(width: u16, height: u16) -> Vec<Line> {
        (0..height).map(|_| Line::new(width)).collect()
    }

    // A static method because it often cause borrow checker problem.
    /// Caculate the position of a middle-aligned component.
    #[inline] 
    pub(crate) fn calc_middle_align_pos(width: u16, len: usize) -> u16 {
        ((width as f32 - len as f32) / 2.0).round() as u16 
    }

    // A static method because it often cause borrow checker problem.
    /// Caculate the position of a bottom-aligned component.
    #[inline]
    fn calc_bottom_align_pos(height: u16) -> u16 {
        height - 1
    }

    #[inline]
    pub(crate) fn ensure_label_inbound(&self, len: usize) -> FtuiResult<()> {
        if len > self.width as usize {
            Err(FtuiError::RendererContainerTooBig)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub(crate) fn lines_mut(&mut self) -> &mut[Line] {
        &mut self.lines
    }

    #[inline]
    pub(crate) fn line_mut(&mut self, n: usize) -> &mut Line {
        &mut self.lines[n]
    }

    #[inline]
    pub(crate) fn get_dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub(crate) fn render_text_as_footer(
        &mut self, footer: &mut cpn::Text
    ) -> FtuiResult<()> {
        self.ensure_label_inbound(footer.len())?;
        footer.resolve_pos(self.width);
        footer.set_line(Self::calc_bottom_align_pos(self.height));

        let line = &mut self.lines[footer.line() as usize];

        line.edit(footer.label(), footer.pos());
        line.add_ansi_many(footer.styles());

        Ok(())
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.lines.iter_mut().for_each(|line| line.clear());
    }

    fn to_string(&self) -> String {
        let mut buf = String::with_capacity(((self.height * self.width) + 40) as usize);
        let reset_suffix = format!("{}{}", ansi::ESC_COLOR_RESET, ansi::ESC_STYLE_RESET);

        buf.push_str(ansi::_ESC_CLEAR_TERM);

        for (i, line) in self.lines.iter().enumerate() {
            let have_ansi = !line.ansi.is_empty();
            let line_data = line.as_string();

            buf.push_str(&line.ansi.concat());

            // Exclude lines containing only whitesapce unless it have ANSIs.
            if !line_data.trim().is_empty() || have_ansi {
                buf.push_str(if have_ansi { &line_data } else { &line_data.trim_end() });
            }

            // Only include the ANSI reset suffix if the line have ANSIs.
            if have_ansi {
                buf.push_str(&reset_suffix);
            }

            if i != (self.height - 1) as usize {
                buf.push_str("\r\n");
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
    /// > The `renderable` parameter must be provided as either a mutable reference
    /// > or an owned value.
    /// > 
    /// > If you encounter an error mentioning `&mut &mut ...` or something similar,
    /// > you’ll need to **reborrow** the inner value (e.g., `&mut *value`).
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
    pub fn draw<C>(&mut self, renderable: &mut C) -> FtuiResult<()>
    where 
        C: RenderableMut<Renderer>
    {
        renderable.render(self)?;

        let mut stdout = std::io::stdout().lock();
        stdout.write_all(self.to_string().as_bytes())?;
        stdout.flush()?;

        Ok(())
    }
}
