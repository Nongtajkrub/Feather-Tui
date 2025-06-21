use crate::{
    components::{Text, TextFlags}, error::{FtuiError, FtuiResult},
    util::id::IdGenerator, renderer::Renderer,
};

#[doc = "⚠️ **Experimental** ⚠️"]
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
    header: Option<Text>,
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

    /// Adds a new element to the `List`.
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the element label.
    /// - `flags`: A set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Notes
    /// - The bitwise OR operator combines flags like this: `flag1 | flag2 | flag3`
    /// - A `List` element is just a `Text` component.
    ///
    /// # Returns
    /// - `Ok(u16)`: Return the ID of the added element. 
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a new `List`.
    /// let mut list = ListBuilder::new().build();
    /// 
    /// // Add an element labeled "Element" with red text and bold styling.
    /// list.add("Element", TextFlags::COLOR_RED | TextFlags::STYLE_BOLD)?;
    /// ```
    pub fn add<'a>(
        &mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<u16> {
        let flags = flags.into();
        let label = label.to_string();

        let id = self.id_generator.get_id(); 

        match flags {
            Some(flags) if flags.contains(TextFlags::ALIGN_BOTTOM) =>
                return Err(FtuiError::TextFlagAlignBottomWithListElement),
            Some(flags) => self.elements.push(Text::with_id(label, flags, id)?),
            None => self.elements.push(Text::with_id(label, self.default_flags, id)?),
        }

        Ok(id)
    }

    /// Attempts to scroll the `List` up by one position.
    ///
    /// # Returns
    /// - `true` if the list was successfully scrolled up.
    /// - `false`: The `List` fail to scroll up (already at the top). 
    ///
    /// # Example
    /// ```rust
    /// // Create a new `List`.
    /// let mut list = ListBuilder::new().build();
    ///
    /// // Add two elements to the list.
    /// list.add(...)?;
    /// list.add(...)?;
    ///
    /// // Initially, the list is at the bottom after scrolling down.
    /// list.scroll_down();
    ///
    /// // Now it can scroll back up.
    /// assert_eq!(list.scroll_up(), true);
    /// ```
    pub fn scroll_up(&mut self) -> bool {
        if self.offset != 0 {
            self.offset -= 1;
            true
        } else {
            false
        }
    }

    /// Attempts to scroll the `List` down by one position.
    ///
    /// # Returns
    /// - `true` if the list was successfully scrolled down.
    /// - `false`: The `List` fail to scroll down (already at the bottom). 
    ///
    /// # Example
    /// ```rust
    /// // Create a new `List`.
    /// let mut list = ListBuilder::new().build();
    ///
    /// // Add two elements to the list.
    /// list.add(...)?;
    /// list.add(...)?;
    ///
    /// // The list can scroll down since it's not at the bottom yet.
    /// assert_eq!(list.scroll_down(), true);
    /// ```
    pub fn scroll_down(&mut self) -> bool {
        if self.offset < self.elements.len() - 1 {
            self.offset += 1;
            true
        } else {
            false
        }
    }

    /// Finds the index of an element by its ID.
    ///
    /// # Parameters
    /// - `id`: The ID of the `Text` component to search for.
    ///
    /// # Returns
    /// - `Ok(usize)`: The index of the element with the specified ID.
    /// - `Err(FtuiError)`: Return an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a new `List`.
    /// let mut list = ListBuilder::new().build();
    ///
    /// // Add an element to the list and retrieve its ID.
    /// let id = list.add(...)?;
    ///
    /// // Find the index of the element by its ID.
    /// let index = list.find(id)?;
    /// ```
    pub fn find(&self, id: u16) -> FtuiResult<usize> {
        self.elements
            .iter()
            .position(|element| element.id() == id)
            .ok_or(FtuiError::ListNoElementById)
    }

    /// Returns a reference to the element at the given index, if it exists.
    ///
    /// # Parameters
    /// - `i`: The index of the element to retrieve.
    ///
    /// # Returns
    /// - `Ok(&Text)`: A reference to the element at the specified index.
    /// - `Err(FtuiError)`: Returns an error. 
    ///
    /// # Example
    /// ```rust
    /// // Create a new `List`.
    /// let mut list = ListBuilder::new().build();
    ///
    /// // Add elements to the list.
    /// list.add(...)?;
    /// list.add(...)?;
    ///
    /// // Access the first element.
    /// list.at(0)?;
    /// ```
    pub fn at(&self, i: usize) -> FtuiResult<&Text> {
        if i < self.elements.len() {
            Ok(&self.elements[i])
        } else {
            Err(FtuiError::ListIndexOutOfBound)
        }
    }

    /// Removes the element at the specified index, if it exists.
    ///
    /// # Parameters
    /// - `i`: The index of the element to remove.
    ///
    /// # Returns
    /// - `Ok(())`: If the element was successfully removed.
    /// - `Err(FtuiError)`: If the index is out of bounds.
    ///
    /// # Example
    /// ```rust
    /// // Create a new `List`.
    /// let mut list = ListBuilder::new().build();
    ///
    /// // Add elements to the list.
    /// list.add(...)?;
    /// list.add(...)?;
    ///
    /// // Remove the first element from the list.
    /// list.remove(0)?;
    /// ```
    pub fn remove(&mut self, i: usize) -> FtuiResult<()> {
        if i < self.elements.len() {
            self.elements.remove(i);
            Ok(())
        } else {
            Err(FtuiError::ListIndexOutOfBound)
        }
    }

    #[inline]
    pub fn draw(&mut self, width: u16, height: u16) -> FtuiResult<()> {
        Renderer::new(width, height).simple_draw_list(self)
    }

    #[inline]
    pub fn draw_fullscreen(&mut self) -> FtuiResult<()> {
        Renderer::fullscreen()?.simple_draw_list(self)
    }

    #[inline]
    pub fn draw_expl(&mut self, renderer: &mut Renderer) -> FtuiResult<()> {
        renderer.simple_draw_list(self)
    }

    pub fn render(&mut self, renderer: &mut Renderer) -> FtuiResult<()> {
        renderer.clear();
        renderer.render_list(self)
    }

    pub(crate) fn header_mut(&mut self) -> &mut Option<Text> {
        &mut self.header
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
    pub fn header_expl(mut self, header: Text) -> Self {
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
    pub fn header(
        self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        Ok(self.header_expl(Text::new(label, flags)?))
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

    pub fn element(
        mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        self.list.add(label, flags)?;
        Ok(self)
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
