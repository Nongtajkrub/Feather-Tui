mod termui;

use termui as tui;

tui_trg_new_trigger_func!(up_trig_func, arg, {
    return false;
});

tui_trg_new_trigger_func!(down_trig_func, arg, {
    return false;
});

tui_trg_new_trigger_func!(selc_trig_func, arg, {
    return true;
});

tui_cbk_new_callback_func!(callback_func1, arg, {
    println!("Hello from 1!");
});

tui_cbk_new_callback_func!(callback_func2, arg, {
    println!("Hello from 2!");
});

fn main() {
    let mut container = tui::container::Container::new();
    let mut renderer = tui::ren::Renderer::new(40, 20);
    let mut should_update = true;

    container.set_header(tui::cpn::hed::Header::new(String::from("Welcome!")));

    container.add_option(
        tui::cpn::opt::Option::new(String::from("Option1"),
        tui::cbk::Callback::new(callback_func1, 0)));
    container.add_option(
        tui::cpn::opt::Option::new(String::from("Option2"),
        tui::cbk::Callback::new(callback_func2, 0)));

    container.set_selector(tui::sel::Selector::new(
            tui::trg::Trigger::new(up_trig_func, 0),
            tui::trg::Trigger::new(down_trig_func, 0),
            tui::trg::Trigger::new(selc_trig_func, 0)));

    tui::ren::ready();

    for _ in 0..3 {
        if should_update {
            renderer.clear();
            renderer.render(&container);
            renderer.draw();
        }

        should_update = container.looper();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    tui::ren::unready();
}
