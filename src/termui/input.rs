use crate::tui::ren;
use crossterm as ct;
use std::io::Write;

pub fn line(promt: String) -> std::io::Result<String> {
    ren::unready();

    print!("{} -> ", promt);
    std::io::stdout().flush()?;

    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    ren::ready();
    Ok(line)
}

pub fn key() -> std::io::Result<std::option::Option<ct::event::KeyCode>> {
    let mut key_code: std::option::Option<ct::event::KeyCode> = None;
    
    ct::terminal::enable_raw_mode()?;

    if ct::event::poll(std::time::Duration::from_millis(100))? {
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

pub fn keycode_to_char(code: ct::event::KeyCode) -> std::option::Option<char> {
    match code {
        ct::event::KeyCode::Char(c) => Some(c),
        _ => None,
    }
}
