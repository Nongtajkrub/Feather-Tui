use crate::ren;
use crossterm as ct;
use std::io::Write;

pub const READ_KEY_FAIL_ERRMSG: &str = "Input: Fail to read key events from the terminal.";

/// Reads a line of input from the user after displaying a prompt.
///
/// # Parameters
/// * `prompt`: A `&sr` containing the message to display before user input.
///
/// # Returns
/// * `Ok(String)`: The user’s input as a `String`, including the newline character.
/// * `Err(std::io::Error)`: An error if reading from `stdin` fails.
///
/// # Notes
/// * The returned `String` includes the newline (`\n`). Use `.trim()` if necessary.
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // Get the user input and print it out if error occure print the error
/// match tui::inp::line("Input Something") {
///     Ok(e) => println!("User Input {}", e),
///     Err(e) => eprintln!("Error: {}", e),
/// };
/// ```
pub fn line(promt: &str) -> std::io::Result<String> {
    ren::unready();

    print!("{} -> ", promt);
    std::io::stdout().flush()?;

    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    ren::ready();
    Ok(line)
}

/// Reads a key press event as `KeyCode` from the terminal without blocking.
///
/// # Returns
/// * `Ok(Some(KeyCode))`: If a key event is detected.
/// * `Ok(None)`: If no key event is detected.
/// * `Err(std::io::Error)`: If an error occurs while interacting with the terminal.
///
/// # Notes
/// * This function does not block waiting for input.
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // Get the user key input as `KeyCode` and print it out
/// match tui::inp::key() {
///     Ok(Some(key)) => println!("Key pressed: {:?}", key),
///     Ok(None) => println!("No key press detected"),
///     Err(e) => eprintln!("Error reading key: {}", e),
/// }
/// ```
pub fn key() -> std::io::Result<std::option::Option<ct::event::KeyCode>> {
    let mut key_code: std::option::Option<ct::event::KeyCode> = None;
    
    ct::terminal::enable_raw_mode()?;

    if ct::event::poll(std::time::Duration::from_millis(0))? {
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
/// * `code`: The `KeyCode` to convert.
///
/// # Returns
/// * `Some(char)`: If the `KeyCode` represents a printable character.
/// * `None`: If the `KeyCode` is not a character (e.g., arrow keys, function keys).
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // Capture user keyboard input as a KeyCode.
/// // If reading fails, terminate with an error message.
/// let key_code = inp::key().expect(inp::READ_KEY_FAIL_ERRMSG);
///
/// // If a key was pressed, attempt to convert it to a character.
/// match key_code {
///     Some(code) => match inp::keycode_to_char(code) {
///         // Print the character if it's a printable key.
///         Some(c) => println!("Key pressed: {}", c), 
///         None => println!("Unprintable KeyCode"), 
///     },
///     // No key was pressed, exit the function.
///     None => return, 
/// }
/// ```
pub fn keycode_to_char(code: ct::event::KeyCode) -> std::option::Option<char> {
    match code {
        ct::event::KeyCode::Char(c) => Some(c),
        _ => None,
    }
}

/// Reads a key press event as a `char` from the terminal without blocking.
///
/// # Returns
/// - `Ok(Some(char))` if a printable key was pressed.
/// - `Ok(None)` if a non-printable key was pressed or no input was detected.
/// - `Err(std::io::Error)` if an I/O error occurs while reading input.
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // Capture user keyboard input as a character and print it out if possible. 
/// match tui::inp::key_char() {
///     Ok(Some(c)) => println!("Key pressed: {}", c),
///     Ok(None) => println!("No key pressed or no printable key pressed"),
///     Err(e) => eprintln!("Error reading key: {}", e),
/// }
/// ```
pub fn key_char() -> std::io::Result<std::option::Option<char>> {
    match key()? {
        Some(code) => Ok(keycode_to_char(code)),
        None => Ok(None),
    }
} 
