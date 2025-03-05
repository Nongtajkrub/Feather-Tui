mod termui;
use termui as tui;

tui_trg_new_trigger_func!(up_trig_func, arg, {
    false
});

tui_trg_new_trigger_func!(down_trig_func, arg, {
    false
});

tui_trg_new_trigger_func!(selc_trig_func, arg, {
    true
});

tui_cbk_new_callback_func!(callback, arg, {
    println!("{}", arg.downcast_ref::<u32>().expect("Expect Number As Arg"));
});

fn main() {
    match tui::inp::read("Input Something".to_string()) {
        Ok(input) => {
            println!("You inputed {}", input);
        }
        Err(e) => {
            println!("Fail to get input: {}", e);
        }
    }

    /*
    let mut container = tui::con::Container::new()
        .with_header(tui::cpn::hed::Header::new("Welcome".to_string()))
        .with_option(
            tui::cpn::opt::Option::new(
                "Settings".to_string(),
                tui::cbk::Callback::new(callback, 1u32)))
        .with_text(
            tui::cpn::txt::Text::new(
                "Text".to_string(), 
                tui::cpn::txt::TextFlags::COLOR_YELLOW_BACK |
                tui::cpn::txt::TextFlags::ALIGN_RIGHT))
        .with_selector(
            tui::sel::Selector::new(
                tui::trg::Trigger::new(up_trig_func, 0),
                tui::trg::Trigger::new(down_trig_func, 0),
                tui::trg::Trigger::new(selc_trig_func, 0)));


    let mut renderer = tui::ren::Renderer::new(40, 20);

    tui::ren::ready();

    container.looper();

    renderer.clear();
    renderer.render(&mut container);
    renderer.draw();

    std::thread::sleep(std::time::Duration::from_secs(2));
    tui::ren::unready();
    */
}
