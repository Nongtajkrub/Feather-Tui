mod termui;

use termui as tui;

fn main() {
    let header = tui::components::header::Header::new(String::from("Header"));
    let option = tui::components::option::Option::new(String::from("Option1"));
    let text = tui::components::text::Text::new(String::from("Text"));
    let mut container = tui::container::Container::new();

    container.set_header(header);
    container.add_option(option);
    container.add_text(text);

    let mut renderer = tui::renderer::Renderer::new(20, 10);
    
    renderer.render(&container);
    renderer.draw();
}
