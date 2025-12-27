use std::u16;

use crate::error::FtuiResult;
use crate::error::FtuiError;
        
use crossterm as ct;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimension {
    width: u16,
    height: u16,
}

impl Dimension {
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
    pub fn custom(width: u16, height: u16) -> FtuiResult<Self> {
        let (term_width, term_height) = ct::terminal::size()?;

        if width > term_width || height > term_height {
            Err(FtuiError::DimensionsTerminalToSmall)
        } else {
            Ok(Self {
                width,
                height,
            })
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
    pub fn fullscreen() -> FtuiResult<Self> {
        let (width, height) = ct::terminal::size()?;
        
        Ok(Self {
            width: width as u16,
            height: height as u16,
        })
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
    pub fn fullwidth(height: u16) -> FtuiResult<Self> {
        let (width, term_height) = ct::terminal::size()?;
        
        if height > term_height {
            Err(FtuiError::DimensionsTerminalToSmall)
        } else {
            Ok(Self { 
                width: width as u16,
                height: height,
            })
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
    pub fn fullheight(width: u16) -> FtuiResult<Self> {
        let (term_width, height) = ct::terminal::size()?;
        
        if width > term_width {
            Err(FtuiError::DimensionsTerminalToSmall)
        } else {
            Ok(Self { 
                width: width,
                height: height as u16,
            })
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}
