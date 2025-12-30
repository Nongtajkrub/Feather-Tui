use crate::components::Text;
use crate::components::TextFlags;
use crate::error::FtuiError;
use crate::error::FtuiResult;
use crate::renderer::Renderer;
use crate::util::id::IdGenerator;
use crate::util::id::GeneratedId;
use crate::util::number as num;
use crate::util::RenderableMut;

/// A specialized variant of `Container` designed to display data in a vertical 
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
#[derive(Debug, PartialEq, Eq)]
pub struct List {
    header: Option<Text>,
    footer: Option<Text>,
    elements: Vec<Text>,
    offset: usize,
    default_flags: Option<TextFlags>,
    is_numbered: bool,
    id_generator: IdGenerator,
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
            footer: None,
            elements: vec![],
            offset: 0,
            default_flags: None,
            is_numbered: false,
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
    ) -> FtuiResult<GeneratedId> {
        let flags = flags.into();
        let id = self.id_generator.get_id(); 

        self.elements.push(Text::with_id(label, flags.or(self.default_flags), id)?);
        Ok(id)
    }

    /// Adds multiple text elements to the `List`.
    ///
    /// # Parameters
    /// - `labels`: An iterable collection of items that can be converted to strings.
    /// - `flags`: A set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Notes
    /// - The bitwise OR operator combines flags like this: `flag1 | flag2 | flag3`
    /// - A `List` element is just a `Text` component.
    ///
    /// # Returns
    /// - `Ok(())`: Returns nothing. 
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// let mut list = ListBuilder::new().build();
    ///
    /// list.add_many(
    ///     vec!["Element1", "Element2"],
    ///     TextFlags::COLOR_RED | TextFlags::STYLE_BOLD)?;
    /// ```
    pub fn add_many<T>(
        &mut self,
        labels: impl IntoIterator<Item = T>, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<()> 
    where
        T: ToString,
    {
        let flags = flags.into();

        for label in labels {
            self.add(label, flags)?;
        }
        
        Ok(())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Attempts to scroll the `List` up by one position.
    ///
    /// # Returns
    /// - `true` Ff the `List` was successfully scrolled up.
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
    /// - `Some(usize)`: The index of the element if a match is found.
    /// - `None`: If no element with the specified ID exists.
    ///
    /// # Example
    /// ```rust
    /// // Create a new `List`.
    /// let mut list = ListBuilder::new().build();
    ///
    /// // Add an element and get its ID.
    /// let id = list.add(...)?;
    ///
    /// // Retrieve the index of the element by its ID.
    /// let index = list.find_id(id)?;
    /// ```
    #[inline]
    pub fn find_id(&self, id: GeneratedId) -> Option<usize> {
        self.elements.iter().position(|element| element.id() == id)
    }

    /// Returns the index of the first element that matches the given label.
    ///
    /// # Parameters
    /// - `label`: The label of the element to search for.
    ///
    /// # Returns
    /// - `Some(usize)`: The index of the first element with the specified label.
    /// - `None`: If no element with the given label exists.
    ///
    /// # Example
    /// ```rust
    /// // Create a new `List`.
    /// let mut list = ListBuilder::new().build();
    ///
    /// // Add an element with a label and retrieve its ID.
    /// let id = list.add("label!", ...)?;
    ///
    /// // Find the index of the element by its label.
    /// let index = list.find_label("label!");
    /// ```
    #[inline]
    pub fn find_label(&self, label: &str) -> Option<usize> {
        self.elements.iter().position(|element| element.label() == label)
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

    /// Returns a mutable reference to the element at the given index, if it exists.
    ///
    /// # Parameters
    /// - `i`: The index of the element to retrieve.
    ///
    /// # Returns
    /// - `Ok(&mut Text)`: A mutable reference to the element at the specified index.
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
    pub fn at_mut(&mut self, i: usize) -> FtuiResult<&mut Text> {
        if i < self.elements.len() {
            Ok(&mut self.elements[i])
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

    fn len(&self) -> usize {
        self.elements.len()
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
        ListBuilder {
            list: List::new(),
        }
    }

    /// Sets a header for the `List`.
    ///
    /// # Notes
    /// The header behaves similarly to a `Text` component and can display
    /// styled text using the provided flags.
    ///
    /// # Parameters
    /// - `label`: A type that impl `ToString` representing the text for the header.
    /// - `flags`: A set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Returns
    /// - `Ok(ListBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Sets a header with the label "Welcome" and no style.
    /// ListBuilder::new()
    ///     .header("Welcome", None)?;
    /// ```
    #[inline]
    pub fn header(
        mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        self.list.header = Some(Text::new(label, flags)?);
        Ok(self)
    }

    /// Sets a footer for the `List`.
    ///
    /// # Notes
    /// The footer behaves similarly to a `Text` component and can display
    /// styled text using the provided flags.
    ///
    /// # Parameters
    /// - `label`: A type that impl `ToString` representing the text for the footer.
    /// - `flags`: A set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Returns
    /// - `Ok(ListBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Sets a footer with the label "Welcome" and no style.
    /// ListBuilder::new()
    ///     .header("Welcome", None)?;
    /// ```
    #[inline]
    pub fn footer(
        mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        self.list.footer = Some(Text::new(label, flags)?);
        Ok(self)
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
        flags.ensure_compatibility()?;
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
        self.list.is_numbered = true;
        self
    }

    /// Renders the current `List` directly to the terminal without
    /// creating and returning a new one.
    ///
    /// # Parameters
    /// - `renderer`: A mutable type that implements `AsMut<Renderer>`.
    ///
    /// # Returns
    /// - `Ok(())` if the list was successfully drawn.
    /// - `Err(FtuiError)` if rendering failed.
    ///
    /// # Example
    /// ```rust
    /// ListBuilder::new()
    ///     .header(...)?
    ///     .instant_draw(Renderer::new(...))?;
    /// ```
    pub fn instant_draw(mut self, mut renderer: impl AsMut<Renderer>) -> FtuiResult<()> {
        renderer.as_mut().draw(&mut self.list)
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

impl RenderableMut<Renderer> for List {
    fn render(&mut self, renderer: &mut Renderer) -> FtuiResult<()> {
        let (width, height) = renderer.get_dimensions();
        let skip_top = if self.header.is_some() { 1 } else { 0 };  
        let skip_bottom = if self.footer.is_some() { 1 } else { 0 };
        let max_elements = (height - 1) as usize - skip_bottom;
        let num_prefix = if self.is_numbered {
            (num::digits(self.len() as u64) + 2) as usize 
        } else { 0 };

        renderer.clear();

        if let Some(header) = &mut self.header {
            header.render(renderer)?;
        }

        if let Some(footer) = &mut self.footer {
            renderer.render_text_as_footer(footer)?;
        }
        
        for (i, elt) in self
            .elements
            .iter_mut()
            .skip(self.offset)
            .take(max_elements)
            .enumerate() 
        {
            renderer.ensure_label_inbound(elt.len())?;
            elt.resolve_pos_custom_len(width, elt.len() + num_prefix);

            let line = renderer.line_mut(i + skip_top);

            if self.is_numbered {
                line.edit(
                    &format!("{}. {}", i + 1 + self.offset, elt.label()), elt.pos());
            } else {
                line.edit(elt.label(), elt.pos());
            }

            line.add_ansi_many(elt.styles());
        }

        Ok(())
    }
}
