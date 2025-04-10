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
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // Create a normal separator with a solid style.
/// let separator = tui::cpn::Separator::normal(tui::cpn::SeparatorStyle::Solid);
///
/// // Create a dotted separator with a solid style.
/// let separator = tui::cpn::Separator::dotted(tui::cpn::SeparatorStyle::Solid);
///
/// // Add the separator to a container.
/// let mut container = tui::con::Container::new();
/// container.add_separator(separator);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Separator {
    line: u16,
    dotted: bool,
    style: SeparatorStyle,
}

impl Separator {
    /// Creates a standard (non-dotted) `Separator` with the given style.
    ///
    /// # Parameters
    /// - `style`: The visual style of the separator, specified as a `SeparatorStyle`.
    ///
    /// # Returns
    /// `Separator`: A new `Separator` instance.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// // Create a normal separator with a solid style.
    /// let separator =
    ///     tui::cpn::Separator::normal(tui::cpn::sep::SeparatorStyle::Solid);
    /// ```
    pub fn normal(style: SeparatorStyle) -> Self {
        Separator {
            line: 0,
            dotted: false,
            style,
        }
    }

    /// Creates a dotted `Separator` with the given style.
    ///
    /// # Parameters
    /// - `style`: The visual style of the separator, specified as a `SeparatorStyle`.
    ///
    /// # Returns
    /// `Separator`: A new `Separator` instance.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// // Create a dotted separator with a solid style.
    /// let separator =
    ///     tui::cpn::Separator::dotted(tui::cpn::sep::SeparatorStyle::Solid);
    /// ```
    pub fn dotted(style: SeparatorStyle) -> Self {
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
