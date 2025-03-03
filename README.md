# 🦀 Termui - Terminal UI Library

![Language](https://img.shields.io/badge/language-Rust-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)
![Status](https://img.shields.io/badge/status-WIP-yellow)

## About

> A Rust rewrite of the terminal UI library I originally wrote in C for my school management project.

⚠️ **This is my first-ever Rust project — expect some experiments, mistakes, and learning-in-public moments along the way.** ⚠️

---

**Termui** is a **simple terminal UI library** designed to provide building blocks for text-based user interfaces.  
It started life as a small C library in my **school management system project**, aiming to offer an **easy-to-use UI framework** for terminal applications. Now, I’m rewriting it in **Rust** to learn the language and (hopefully) improve both performance and maintainability.

---

## 📊 Progress

![](https://geps.dev/progress/20)

* 🚧 Termui is still under development.  
* ✅ Some parts are complete, while others are only partially done.  
* ✨ These parts may get refined or improved in the future.  
* 👍 For now, they’re good enough to work with.  

---

## 🚀 Usage

Here’s a step-by-step example demonstrating how to create a basic terminal UI using **Termui** in it current state.



### 1️⃣ Import Termui

```rust
mod termui;  // Import your termui module
use termui as tui;  // Use a shorter alias for convenience
```



### 2️⃣ Create Components

```rust
let header = tui::components::header::Header::new(String::from("Header"));
let option = tui::components::option::Option::new(String::from("Option"));
let text = tui::components::text::Text::new(String::from("Text"));
```

This creates 3 components:

| Component   | Description                           |
|-------------|---------------------------------------|
| ✨ Header   | Displays a title at the top.          |
| 🛠️ Option   | Represent selectable options.         |
| 📝 Text     | Displays regular text content.        |



### 3️⃣ Make A Container

```rust
let mut container = tui::container::Container::new();
container.set_header(header);
container.add_option(option);
container.add_text(text);
```

The container holds and organizes all components, so the renderer can process them as a single unit.



### 4️⃣ Create a Renderer

```rust
let mut renderer = tui::renderer::Renderer::new(20, 10);
```

This sets up the renderer, which controls the drawing area (20 columns wide, 10 rows high in this case).



### 5️⃣ Ready The Renderer

```rust
tui::renderer::ready();
```

This prepares the terminal for drawing (like switching into a special "UI mode").



### 6️⃣ Render and Draw

```rust
renderer.render(&container);
renderer.draw();
```

This tells the renderer to process the container and its components, then draw everything onto the screen.



### 7️⃣ Delay And Exit

```rust
std::thread::sleep(std::time::Duration::from_secs(2));
tui::renderer::unready();
```

Without delaying, the program would quit instantly, so this gives you time to see the UI.  
Unreadying the renderer restores the terminal back to its normal state (important to avoid terminal glitches after quitting).

---

## 🌱 Related Projects

- [Original C Version (part of my school management system)](https://github.com/nongtajkrub/school-management)
- [This Rust Rewrite (current project)](https://github.com/nongtajkrub/termui)
