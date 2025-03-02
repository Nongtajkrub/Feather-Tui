mod termui;

use std::time::Duration;

use termui as tui;

fn main() {
    let header = tui::components::header::Header::new(String::from("Header"));
    let option1 = tui::components::option::Option::new(String::from("Option1"));
    let option2 = tui::components::option::Option::new(String::from("Option2"));
    let text = tui::components::text::Text::new(String::from("Text"));
    let mut container = tui::container::Container::new();

    container.set_header(header);
    container.add_option(option1);
    container.add_option(option2);
    container.add_text(text);

    let mut renderer = tui::renderer::Renderer::new(20, 10);
    
    tui::renderer::ready();

    renderer.render(&container);
    renderer.draw();

    std::thread::sleep(std::time::Duration::from_secs(2));
    tui::renderer::unready();
}
