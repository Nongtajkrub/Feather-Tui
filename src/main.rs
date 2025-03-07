mod termui;
use termui as tui;

tui_trg_new_trigger_func!(up_trig_func, arg, {
    match arg.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 'w',
        None => false,
    }
});

tui_trg_new_trigger_func!(down_trig_func, arg, {
    match arg.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 's',
        None => false,
    }
});

tui_trg_new_trigger_func!(selc_trig_func, arg, {
    match arg.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 'e',
        None => false,
    }
});

tui_trg_new_trigger_func!(quit_trig_func, arg, {
    match arg.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 'q',
        None => false,
    }
});

tui_cbk_new_callback_func!(callback, arg, {
    println!("{}", arg.downcast_ref::<u32>().expect("Expect Number As Arg"));
});

#[inline]
fn get_key_char() -> std::option::Option<char> {
    tui::inp::key_char().expect(tui::inp::READ_KEY_FAIL_ERRMSG)
}

fn main() {
    let mut key_char: std::option::Option<char> = get_key_char();

    let mut container = tui::con::Container::new()
        .with_header(tui::cpn::hed::Header::new("Welcome".to_string()))
        .with_option(
            tui::cpn::opt::Option::new(
                "Settings".to_string(),
                tui::cbk::Callback::new(callback, 1u32)))
        .with_option(
            tui::cpn::opt::Option::new("Credits".to_string(),
            tui::cbk::Callback::new(callback, 2u32)))
        .with_text(
            tui::cpn::txt::Text::new(
                "Text".to_string(), 
                tui::cpn::txt::TextFlags::COLOR_YELLOW_BACK |
                tui::cpn::txt::TextFlags::ALIGN_RIGHT))
        .with_selector(
            tui::sel::Selector::new(
                tui::trg::Trigger::new(up_trig_func, key_char),
                tui::trg::Trigger::new(down_trig_func, key_char),
                tui::trg::Trigger::new(selc_trig_func, key_char)));


    let mut renderer = tui::ren::Renderer::new(40, 20);
    let mut should_update = true;
    let mut quit_trig = tui::trg::Trigger::new(quit_trig_func, key_char);

    tui::ren::ready();

    loop {
        if quit_trig.check() {
            break;
        }

        if should_update {
            renderer.clear();
            renderer.render(&mut container);
            renderer.draw();
        }

        key_char = get_key_char();
        container.selector_mut().update_trig_arg(key_char, key_char, key_char);
        quit_trig.update_arg(key_char);
        should_update = container.looper();
    }

    tui::ren::unready();
}
