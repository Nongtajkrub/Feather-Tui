use std::io::Write;
use crate::tui::ren;

pub fn read(promt: String) -> std::io::Result<String> {
    ren::unready();

    print!("{} -> ", promt);
    std::io::stdout().flush()?;

    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    ren::ready();
    Ok(line)
}
