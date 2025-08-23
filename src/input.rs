use std::io;
use std::io::Write;

use crossterm as ct;

use crate::error::FtuiResult;

/// Reads a line of input from the user after displaying a prompt.
///
/// # Parameters
/// - `prompt`: A `&str` containing the message to display before user input.
///
/// # Returns
/// - `Ok(String)`: The user’s input as a `String`, including the newline character.
/// - `Err(FtuiError)`: Returns an error.
///
/// # Notes
/// - The returned `String` includes the newline (`\n`). Use `.trim()` if necessary.
///
/// # Example
/// ```rust
/// // Get the user input and print it out if error occure print the error
/// match line("Input Something") {
///     Ok(e) => println!("User Input {}", e),
///     Err(e) => eprintln!("Error: {}", e),
/// };
/// ```
pub fn line(promt: &str) -> FtuiResult<String> {
    let mut stdout = io::stdout();
    ct::execute!(
        stdout,
        ct::terminal::Clear(ct::terminal::ClearType::All), ct::cursor::Show)?;

    print!("{} -> ", promt);
    stdout.flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    ct::execute!(
        stdout,
        ct::terminal::Clear(ct::terminal::ClearType::All),
        ct::cursor::Hide, ct::cursor::MoveTo(0, 0))?;
    Ok(line)
}

/// Reads a key press event as `KeyCode` from the terminal without blocking.
///
/// # Returns
/// - `Ok(Some(KeyCode))`: If a key event is detected.
/// - `Ok(None)`: If no key event is detected.
/// - `Err(FtuiError)`: Returns an error. 
///
/// # Notes
/// - This function does not block waiting for input.
///
/// # Example
/// ```rust
/// fn main() -> FtuiResult<()> {
///     // Get the user key input as `KeyCode` and print it out
///     match key()? {
///         Some(key) => println!("Key pressed: {:?}", key),
///         None => println!("No key press detected"),
///     };
///
///     Ok(())
/// }
/// ```
pub fn key() -> FtuiResult<Option<ct::event::KeyCode>> {
    let mut key_code: Option<ct::event::KeyCode> = None;
    ct::terminal::enable_raw_mode()?;

    if ct::event::poll(std::time::Duration::from_millis(16))? {
        match ct::event::read()? {
            ct::event::Event::Key(event) => {
                key_code = Some(event.code);
            }
            _ => {}
        }
    }

    ct::terminal::disable_raw_mode()?;
    Ok(key_code)
} 

/// Converts a `KeyCode` into its corresponding character, if applicable.
///
/// # Parameters
/// - `code`: The `KeyCode` to convert.
///
/// # Returns
/// - `Some(char)`: If the `KeyCode` represents a printable character.
/// - `None`: If the `KeyCode` is not a character (e.g., arrow keys, function keys).
///
/// # Example
/// ```rust
/// fn main() -> FtuiResult<()> {
///     // Capture user keyboard input as a KeyCode.
///     // If reading fails, terminate with an error.
///     let key_code = key()?;
///
///     // If a key was pressed, attempt to convert it to a character.
///     match key_code {
///         Some(code) => match keycode_to_char(code) {
///             // Print the character if it's a printable key.
///             Some(c) => println!("Key pressed: {}", c), 
///             None => println!("Unprintable KeyCode"), 
///         },
///         // No key was pressed, exit the function.
///         None => return, 
///     }
///
///     Ok(())
/// }
/// ```
pub fn keycode_to_char(code: ct::event::KeyCode) -> Option<char> {
    match code {
        ct::event::KeyCode::Char(c) => Some(c),
        _ => None,
    }
}

/// Reads a key press event as a `char` from the terminal without blocking.
///
/// # Returns
/// - `Ok(Some(char))`: if a printable key was pressed.
/// - `Ok(None)`: if a non-printable key was pressed or no input was detected.
/// - `Err(FtuiError)`: Returns an error.
///
/// # Example
/// ```rust
/// fn main() -> FtuiResult<()> {
///     // Capture user keyboard input as a character and print it out if
///     // possible. 
///     match key_char()? {
///         Some(c) => println!("Key pressed: {}", c),
///         None => println!("No key pressed or no printable key pressed"),
///     }
/// }
/// ```
pub fn key_char() -> FtuiResult<Option<char>> {
    match key()? {
        Some(code) => Ok(keycode_to_char(code)),
        None => Ok(None),
    }
} 

pub fn wait_for_keypress() -> FtuiResult<()> {
    loop {
        if let Some(_) = key()? {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    Ok(())
}
