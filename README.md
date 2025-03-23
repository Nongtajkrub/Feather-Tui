# 🦀 Feather-Tui

![Language](https://img.shields.io/badge/language-Rust-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)
![Status](https://img.shields.io/badge/status-WIP-yellow)

> A Rust rewrite of the terminal UI library I originally wrote in C for my school management project.

⚠️ **This is my first-ever Rust project** ⚠️  


Termui is a simple terminal UI library designed to provide building blocks for text-based user interfaces. It started life as a small C library in my school management system project, aiming to offer an easy-to-use UI framework for terminal applications. Now, I’m rewriting it in Rust to learn the language and (hopefully) improve both performance and maintainability.

---

## 📊 Progress

![](https://geps.dev/progress/50)

* 🚧 Termui is still under development.  
* ✅ Some parts are complete, while others are only partially done.  
* ✨ These parts may get refined or improved in the future.  
* 👍 For now, they’re good enough to work with.  

---

## 📦 Crates

**Comming Soon**

---

## 🚀 Usage

I am really not unexpected people to actually use this crate. So here is a quick example I made in 5 minutes.

``` Rust
use feather_tui as tui;

tui::tui_trg_new_trigger_func!(up_trig_func, key_char, {
    match key_char.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 'w',
        None => false,
    }
});

tui::tui_trg_new_trigger_func!(down_trig_func, key_char, {
    match key_char.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 's',
        None => false,
    }
});

tui::tui_trg_new_trigger_func!(selc_trig_func, key_char, {
    match key_char.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 'e',
        None => false,
    }
});

tui::tui_trg_new_trigger_func!(quit_trig_func, key_char, {
    match key_char.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 'q',
        None => false,
    }
});

tui::tui_cbk_new_callback_func!(callback_func, argument, {
    println!("Callback Argument: {}", argument.downcast_ref::<u32>().expect("Expect callback argument to be a u32"));
});

#[inline]
fn read_key() -> std::option::Option<char> {
    tui::inp::key_char().expect(tui::inp::READ_KEY_FAIL_ERRMSG)
}

fn main() {
    let mut key_char: std::option::Option<char> = read_key();

    let mut container = tui::con::Container::new()
        .with_header(tui::cpn::hed::Header::new("Main Menu"))
        .with_option(
            tui::cpn::opt::Option::new(
                "Option1",
                tui::cbk::Callback::new(callback_func, 1u32)))
        .with_option(
            tui::cpn::opt::Option::new(
                "Option2",
                tui::cbk::Callback::new(callback_func, 2u32)))
        .with_text(
            tui::cpn::txt::Text::new(
                "Text", 
                tui::cpn::txt::TextFlags::COLOR_YELLOW_BACK |
                tui::cpn::txt::TextFlags::ALIGN_RIGHT))
        .with_selector(
            tui::sel::Selector::new(
                tui::trg::Trigger::new(up_trig_func, key_char),
                tui::trg::Trigger::new(down_trig_func, key_char),
                tui::trg::Trigger::new(selc_trig_func, key_char)));

    let mut renderer = tui::ren::Renderer::new(40, 20);
    let mut quit_trig = tui::trg::Trigger::new(quit_trig_func, key_char);
    let mut should_update = true;

    tui::ren::ready();
    
    loop {
        key_char = read_key();
        container.selector_mut().update_trig_arg(key_char, key_char, key_char);
        quit_trig.update_arg(key_char);

        if quit_trig.check() {
            break;
        }

        if should_update {
            renderer.clear();
            renderer.render(&mut container);
            renderer.draw();
        }

        should_update = container.looper();
    }

    tui::ren::unready();
}
```

---

## 🏗️ Dependencies

`bitflags` `crossterm`

---

## 🌱 Related Projects

- [Original C Version (part of my school management system)](https://github.com/nongtajkrub/school-management)
- [This Rust Rewrite (current project)](https://github.com/nongtajkrub/termui)
