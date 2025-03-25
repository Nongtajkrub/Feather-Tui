# 🦀 Feather-Tui

![Language](https://img.shields.io/badge/language-Rust-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)
![Status](https://img.shields.io/badge/status-WIP-yellow)

> A Rust rewrite of the terminal UI library I originally wrote in C for my school management project.

⚠️ **This is my first-ever Rust project** ⚠️  


Feather-Tui is a simple terminal UI library designed to provide building blocks for text-based user interfaces. It started life as a small C library in my school management system project, aiming to offer an easy-to-use UI framework for terminal applications. Now, I’m rewriting it in Rust to learn the language and (hopefully) improve both performance and maintainability.

---

## 📊 Progress

![](https://geps.dev/progress/70)

* 🚧 Feather-Tui is still under development.  
* ✅ Some parts are complete, while others are only partially done.  
* ✨ These parts may get refined or improved in the future.  
* 👍 For now, they’re good enough to work with.  

---

## 📝 Changelog (v0.2.5)

* **(🚨 Breaking Change 🚨)** The methods `with_header`, `with_option`, and `with_text` in `Container` now take the required data as arguments to create the components, instead of accepting the components directly.

---

## 📦 Crates

https://crates.io/crates/feather-tui

---

## 🚀 Usage

https://github.com/Nongtajkrub/Feather-Tui/wiki

The Wiki is still not fully complete so here an example.

```rust
use feather_tui as tui;

// Define trigger functions for handling keyboard input
// This macro creates a function that checks if 'w' key was pressed (for moving up)
tui::trg_new_trigger_func!(up_trig_func, key_char, {
    match key_char.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 'w',  // Return true if 'w' is pressed
        None => false,          // Return false if no key was pressed
    }
});

// Create trigger function for moving down with 's' key
tui::trg_new_trigger_func!(down_trig_func, key_char, {
    match key_char.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 's',  // Return true if 's' is pressed
        None => false,
    }
});

// Create trigger function for selection with 'e' key
tui::trg_new_trigger_func!(selc_trig_func, key_char, {
    match key_char.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 'e',  // Return true if 'e' is pressed
        None => false,
    }
});

// Create trigger function for quitting with 'q' key
tui::trg_new_trigger_func!(quit_trig_func, key_char, {
    match key_char.downcast_ref::<std::option::Option<char>>().unwrap() {
        Some(c) => *c == 'q',  // Return true if 'q' is pressed
        None => false,
    }
});

// Define callback function that executes when a menu option is selected
tui::cbk_new_callback_func!(callback_func, argument, {
    // Convert the u32 argument to a string to display which option was selected
    let number_as_str =
        argument.downcast_ref::<u32>().expect("Expect callback argument to be a u32").to_string();
    
    // Create a new renderer with dimensions 40x20
    tui::ren::Renderer::new(40, 20)
        .simple_draw(
            &mut tui::con::Container::new()
                .with_header(
                    tui::cpn::hed::Header::new("Callback Trigger"))  // Add header to container
                .with_text(
                    tui::cpn::txt::Text::new(
                        &format!("Option {} selected", number_as_str),  // Display which option was selected
                        tui::cpn::txt::TextFlags::COLOR_RED_BACK)));    // Red background for the text
    
    // Pause for 1 second to show the selection message
    std::thread::sleep(std::time::Duration::from_secs(1));
});

// Helper function to read keyboard input
#[inline]
fn read_key() -> std::option::Option<char> {
    tui::inp::key_char().expect(tui::inp::READ_KEY_FAIL_ERRMSG)
}

fn main() {
    // Initialize key_char with the first keypress
    let mut key_char: std::option::Option<char> = read_key();
    
    // Create main container with various UI components
    let mut container = tui::con::Container::new()
        .with_header(tui::cpn::hed::Header::new("Main Menu"))  // Add header "Main Menu"
        .with_option(
            tui::cpn::opt::Option::new(
                "Option1",  // First menu option text
                tui::cbk::Callback::new(callback_func, 1u32)))  // Link to callback with argument 1
        .with_option(
            tui::cpn::opt::Option::new(
                "Option2",  // Second menu option text
                tui::cbk::Callback::new(callback_func, 2u32)))  // Link to callback with argument 2
        .with_text(
            tui::cpn::txt::Text::new(
                "Text",  // Additional text element
                tui::cpn::txt::TextFlags::COLOR_YELLOW_BACK |  // Yellow background
                tui::cpn::txt::TextFlags::ALIGN_RIGHT))       // Right-aligned
        .with_selector(
            tui::sel::Selector::new(
                tui::trg::Trigger::new(up_trig_func, key_char),      // Up trigger ('w' key)
                tui::trg::Trigger::new(down_trig_func, key_char),    // Down trigger ('s' key)
                tui::trg::Trigger::new(selc_trig_func, key_char)));  // Select trigger ('e' key)
    
    // Create renderer with 40x20 dimensions
    let mut renderer = tui::ren::Renderer::new(40, 20);
    
    // Create quit trigger with 'q' key
    let mut quit_trig = tui::trg::Trigger::new(quit_trig_func, key_char);
    
    // Flag to determine if UI needs to be redrawn
    let mut should_update = true;
    
    // Initialize the renderer
    tui::ren::ready();
    
    // Main application loop
    loop {
        // Read the latest keypress
        key_char = read_key();
        
        // Update all triggers with the latest keypress
        container.selector_mut().update_trig_arg(key_char, key_char, key_char);
        quit_trig.update_arg(key_char);
        
        // Check if quit was triggered
        if quit_trig.check() {
            break;  // Exit the loop if 'q' was pressed
        }
        
        // Redraw the UI if necessary
        if should_update {
            renderer.clear();        // Clear the screen
            renderer.render(&mut container);  // Render the container
            renderer.draw();         // Draw to screen
        }
        
        // Process any container events and determine if update is needed next iteration
        should_update = container.looper();
    }
    
    // Clean up renderer when application ends
    tui::ren::unready();
}
```

---

## 🏗️ Dependencies

`bitflags` `crossterm`

---

## 🌱 Related Projects

- [Original C Version (part of my school management system)](https://github.com/nongtajkrub/school-management)
