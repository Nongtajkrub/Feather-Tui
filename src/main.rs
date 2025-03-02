mod termui;

use termui as tui;

tui_cbk_new_callback_func!(func, arg, {
    if let Some(num) = arg.downcast_ref::<u32>() {
        println!("{}", num);
    } else {
        println!("Not a num");
    }
});

fn main() {
    // let mut container = tui::con::Container::new();

    let callback = tui::cbk::Callback::new(func, 12);

    callback.call();

    /*
    container.set_header(tui::cpn::hed::Header::new(String::from("Welcome")));

    container.add_option(tui::cpn::opt::Option::new(String::from("Option1")));
    container.add_option(tui::cpn::opt::Option::new(String::from("Option2")));

    let mut renderer = tui::ren::Renderer::new(20, 10);

    tui::ren::ready();

    renderer.clear();
    renderer.render(&container);
    renderer.draw();

    std::thread::sleep(std::time::Duration::from_secs(2));

    renderer.clear();
    renderer.render(&container);
    renderer.draw();

    std::thread::sleep(std::time::Duration::from_secs(2));

    tui::ren::unready();
    */
}
