use crate::renderer::RenderableComponent;
use crate::renderer::Renderer;
use crate::error::FtuiResult;


/// An `enum` representing all possible styles for a `Separator` component.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorStyle {
    /// A solid block line.
    ///
    /// `███████████████████████████`
    Solid,

    /// A medium-thickness line.
    ///
    /// `━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`
    Medium,

    /// A thin line.  
    ///   
    /// `──────────────────────────────`
    Thin,

    /// A double line.
    ///
    /// `══════════════════════════════`
    Double,

    /// A custom character for the separator. Example using the `+` character:
    ///
    /// `++++++++++++++++++++++++++++++`
    Custom(char),
}

/// A UI component that acts as a separator typically a horizontal line.
/// `Separator` components are displayed in the order they are added to a
/// `Container`. Each `Separator` can have a different style, specified using
/// the `SeparatorStyle` enum. There are two types of separators available:
/// normal and dotted.
///
/// # Notes
/// - A normal separator looks like this: `-------`
/// - A dotted separator looks like this: `- - - -`
///
/// # Usage
/// The `Separator` component is useful for dividing sections in your terminal UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Separator {
    line: u16,
    dotted: bool,
    style: SeparatorStyle,
}

impl Separator {
    /// Creates a standard (non-dotted) `Separator` with the given style.
    ///
    /// # Returns
    /// `Separator`: A new `Separator` instance.
    pub(crate) fn normal(style: SeparatorStyle) -> Self {
        Separator {
            line: 0,
            dotted: false,
            style,
        }
    }

    /// Creates a dotted `Separator` with the given style.
    ///
    /// # Returns
    /// `Separator`: A new `Separator` instance.
    pub(crate) fn dotted(style: SeparatorStyle) -> Self {
        Separator {
            line: 0,
            dotted: true,
            style
        }
    }

    pub(crate) fn set_line(&mut self, line: u16) {
        self.line = line; 
    }

    pub(crate) fn line(&self) -> u16 {
        self.line
    }

    pub(crate) fn is_dotted(&self) -> bool {
        self.dotted
    }

    pub(crate) fn style(&self) -> SeparatorStyle {
        self.style
    }
}

#[inline]
fn apply_correct_separator(renderer: &mut Renderer, separator: &Separator, c: char) {
    if separator.is_dotted() {
        renderer.line_mut(separator.line() as usize).fill_dotted(c);
    } else {
        renderer.line_mut(separator.line() as usize).fill(c);
    }
}
    
impl RenderableComponent for Separator {
    fn render(&mut self, renderer: &mut Renderer) -> FtuiResult<()> {
        match self.style() {
            SeparatorStyle::Solid => 
                apply_correct_separator(renderer, self, '█'), 
            SeparatorStyle::Medium =>
                apply_correct_separator(renderer, self, '━'),
            SeparatorStyle::Thin =>
                apply_correct_separator(renderer, self, '─'),
            SeparatorStyle::Double => 
                apply_correct_separator(renderer, self, '═'),
            SeparatorStyle::Custom(c) =>
                apply_correct_separator(renderer, self, c),
        }

        Ok(())
    }
} 
