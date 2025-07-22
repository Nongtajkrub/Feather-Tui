use crate::error::FtuiResult;
use std::io;
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
    ct::terminal::enable_raw_mode()?;
    ct::execute!(
        io::stdout(),
        ct::terminal::EnterAlternateScreen,
        ct::terminal::Clear(ct::terminal::ClearType::All),
        ct::cursor::MoveTo(0, 0), ct::cursor::Hide)?;

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
    ct::terminal::disable_raw_mode()?;
    ct::execute!(
        io::stdout(),
        ct::cursor::Show,
        ct::terminal::LeaveAlternateScreen)?;

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
#[inline]
pub fn clear() -> FtuiResult<()> {
    ct::execute!(io::stdout(), ct::terminal::Clear(ct::terminal::ClearType::All))?;
    Ok(())
}

