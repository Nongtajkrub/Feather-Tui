mod termui;

use termui as tui;

tui_trg_new_trigger_func!(up_trig_func, arg, {
    return false;
});

tui_trg_new_trigger_func!(down_trig_func, arg, {
    return false;
});

tui_trg_new_trigger_func!(selc_trig_func, arg, {
    return false;
});

tui_cbk_new_callback_func!(callback_func, arg, {
    println!("Hello from 1!");
});

fn main() {
    let mut container1 = tui::container::Container::new()
        .with_header(tui::cpn::hed::Header::new(String::from("Welcom")))
        .with_option(
            tui::cpn::opt::Option::new(String::from("Settings"),
            tui::cbk::Callback::new(callback_func, 0)))
        .with_option(
            tui::cpn::opt::Option::new(String::from("Credit"),
            tui::cbk::Callback::new(callback_func, 0)))
        .with_selector(
            tui::sel::Selector::new(
                tui::trg::Trigger::new(up_trig_func, 0),
                tui::trg::Trigger::new(down_trig_func, 0),
                tui::trg::Trigger::new(selc_trig_func, 0)));

    let mut renderer = tui::ren::Renderer::new(40, 20);
    let mut menu = tui::mnu::Menu::new(&mut container1);
    let mut should_update = true;

   tui::ren::ready();

    for _ in 0..3 {
        if should_update {
            renderer.clear();
            renderer.render(menu.current());
            renderer.draw();
        }

        should_update = menu.current_mut().looper();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    tui::ren::unready();
}
