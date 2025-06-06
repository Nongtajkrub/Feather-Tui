use crate::{
    components::{Header, Text, TextFlags}, error::{FtuiError, FtuiResult},
    util::id::IdGenerator,
};

/// Another variant of a `Container` designed to display data in a vertical 
/// list format. A `List` is scrollable, allowing it to handle a dynamic number
/// of elements. It can be created using the `ListBuilder`, and new elements can
/// be added using the `add` method.
/// 
/// # Usage
/// Use `List` to present information in a vertically ordered list.
/// 
/// `1. Item one`    
/// `2. Item two`   
/// `3. Item three`  
pub struct List {
    header: Option<Header>,
    elements: Vec<Text>,
    offset: usize,
    default_flags: Option<TextFlags>,
    number: bool,
    id_generator: IdGenerator<u16>,
}

impl List {
    /// Constructs a new `List`. 
    ///
    /// # Returns
    /// `List`: A new instance of `List`.
    ///
    /// # Example
    /// ```rust
    /// let _ = List::new();
    /// ```
    pub(crate) fn new() -> Self {
        List {
            header: None,
            elements: vec![],
            offset: 0,
            default_flags: None,
            number: false,
            id_generator: IdGenerator::new(),
        }
    }

    pub fn add(
        &mut self, label: &str, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<u16> {
        let flags: Option<TextFlags> = flags.into();

        let id = self.id_generator.get_id(); 

        match flags {
            Some(flags) if flags.contains(TextFlags::ALIGN_BOTTOM) =>
                return Err(FtuiError::TextFlagAlignBottomWithListElement),
            Some(flags) => self.elements.push(Text::with_id(label, flags, id)?),
            None => self.elements.push(Text::with_id(label, self.default_flags, id)?),
        }

        Ok(id)
    }

    pub fn scroll_down(&mut self) -> bool {
        if self.offset < self.elements.len() - 1 {
            self.offset += 1;
            true
        } else {
            false
        }
    }

    pub fn scroll_up(&mut self) -> bool {
        if self.offset != 0 {
            self.offset -= 1;
            true
        } else {
            false
        }
    }

    pub fn remove_index(&mut self, i: usize) -> FtuiResult<()> {
        if i < self.elements.len() {
            self.elements.remove(i);
            Ok(())
        } else {
            Err(FtuiError::ListRemoveIndexOutOfBound)
        }
    }

    pub fn remove_id(&mut self, id: u16) -> FtuiResult<()> {
        self.elements
            .iter()
            .position(|element| element.id() == id)
            .map(|index| { self.elements.remove(index); })
            .ok_or(FtuiError::ListRemoveNoElementById)
    }

    pub(crate) fn header(&self) -> &Option<Header> {
        &self.header
    }

    pub(crate) fn elements_mut(&mut self) -> &mut [Text] {
        &mut self.elements
    }

    pub(crate) fn offset(&self) -> usize {
        self.offset
    }

    pub(crate) fn len(&self) -> usize {
        self.elements.len()
    }
    
    pub(crate) fn is_number(&self) -> bool {
        self.number
    }
}

/// `ListBuilder` is used to create `List` instances using the builder pattern.
/// This allows for a flexible and readable way to construct `List` with different
/// options by chaining method calls.
///
/// # Example
/// ```rust
/// ListBuilder::new()
///     .header(...)?
///     .default_flags(...)?
///     .number()
///     .build();
/// ```
pub struct ListBuilder {
    list: List,
}

impl ListBuilder {
    /// Constructs a new `ListBuilder`. 
    ///
    /// # Return
    /// `ListBuilder`: A new instance of `ListBuilder`.
    ///
    /// # Example
    /// ```rust
    /// let _ = ListBuilder::new();
    /// ```
    pub fn new() -> Self {
        ListBuilder { list: List::new(), }
    }

    /// Explicitly sets a `Header` component for the `List`. Unlike the `header`
    /// method, which takes a label and internally constructs a `Header`, this 
    /// method allows you to directly provide a preconstructed `Header` component.
    ///
    /// # Parameters
    /// - `header`: A `Header` component.
    ///
    /// # Returns
    /// - `ListBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Create a `Header` component.
    /// let header = Header::new(...)?;
    ///
    /// // Set a preconstructed `Header` component.
    /// ListBuilder::new()
    ///     .header_expl(header);
    /// ```
    pub fn header_expl(mut self, header: Header) -> Self {
        self.list.header = Some(header);
        self
    }

    /// Sets a `Header` component for the `List`.
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text to display in the header.
    ///
    /// # Returns
    /// - `Ok(ListBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Sets a `Header` component with the label "Welcome".
    /// List::new()
    ///     .header("Welcome")?;
    /// ```
    #[inline]
    pub fn header(self, label: &str) -> FtuiResult<Self> {
        Ok(self.header_expl(Header::new(label)?))
    }

    /// Sets the default `TextFlags` to be used when adding elements to the `List`.
    ///
    /// # Parameters
    /// - `flags`: The `TextFlags` to apply to elements unless explicitly overridden.
    ///
    /// # Returns
    /// - `Ok(ListBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Set a default red color for all elements added to the list, unless overridden.
    /// ListBuilder::new()
    ///     .default_flags(tui::TextFlags::COLOR_RED)?;
    /// ```
    pub fn default_flags(mut self, flags: TextFlags) -> FtuiResult<Self> {
        Text::ensure_compatible_flags(&flags)?;
        self.list.default_flags = Some(flags);
        Ok(self)
    }

    /// Enables numbering for the `List`, adding a number prefix to each element.
    ///
    /// # Returns
    /// - `Self`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// ListBuilder::new()
    ///     .number();
    /// ```
    pub fn number(mut self) -> Self {
        self.list.number = true;
        self
    }

    /// Finalizes the construction of a `List`. This method should be called
    /// after all desired options have been set using the builder pattern.
    /// It consumes `self` and returns the completed `List`.
    ///
    /// # Returns
    /// - `List`: Returns the created `List`.
    ///
    /// # Example
    /// ```rust
    /// ListBuilder::new()
    ///     .header(...)?
    ///     .default_flags(...)?
    ///     .number()
    ///     .build(); // Finalize and retrieve the constructed list.
    /// ```
    pub fn build(self) -> List {
        self.list
    }
}
