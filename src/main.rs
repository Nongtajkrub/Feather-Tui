mod termui;

use termui as tui;

fn func(num: &u32) {
    println!("{}", num);
}

fn main() {
    let mut container = tui::con::Container::new();

    container.set_header(tui::cpn::hed::Header::new(String::from("Welcome")));

    container.add_option(tui::cpn::opt::Option::new(String::from("Option1")));
    container.add_option(tui::cpn::opt::Option::new(String::from("Option2")));

    let mut renderer = tui::ren::Renderer::new(20, 10);

    tui::ren::ready();

    renderer.render(&container);
    renderer.draw();

    std::thread::sleep(std::time::Duration::from_secs(2));
    tui::ren::unready();
}
