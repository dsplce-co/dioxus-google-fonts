# dioxus-google-fonts

> 💅 Declarative Google Fonts embedding for [Dioxus](https://dioxuslabs.com) — done right.

This crate provides two declarative macros for working with [Google Fonts](https://fonts.google.com) in a **Dioxus** app:

- `google_fonts_url!` — generates the Google Fonts `<link>` URL
- `google_fonts!` — renders an actual `document::Link` node for easy use in `rsx!`

---

## 🖤 Features

✅ Links are generated at compile time<br>
✅ Declarative macro syntax  
✅ Supports multiple fonts, weights and italics  
✅ Auto-generates `ital,wght@...` combinations  
✅ Automatically adds `display=swap`  
✅ Works out of the box with **Dioxus Web + SSR**

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
# In your main Dioxus app
dioxus-google-fonts = "0.1"

# This is a procedural macro crate, it will bring in `proc-macro2`, `syn`, etc.
```

---

## 🧪 Example

```rust
use dioxus_google_fonts::google_fonts_url;

fn main() -> Element {
    let url = google_fonts_url!([
        ("Roboto", wght = [400, 700], ital = [(1, 700)]),
        ("Manrope", wght = ["200..800"])
    ]);
    
    assert_eq!(
        url,
        "https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&family=Manrope:wght@200..800&display=swap"
    );
}
```


```rust
use dioxus::prelude::*;
use dioxus_google_fonts::google_fonts;

fn App() -> Element {
    rsx! {
        { google_fonts!([
            ("Roboto", wght = [400, 700], ital = [(1, 700)]),
            ("Mukta", wght = ["200..900"])
        ]) }

        h1 { "Hello, typographically!" }
    }
}

fn main() {
    dioxus::web::launch(App);
}
```

---

## 🧠 Supported Syntax

Each font entry uses a tuple-like DSL:

```rust
("Font Name", wght = [weights...], ital = [(italic_flag, weight)...])
```

- `wght`: weights, as numbers (`400`, `700`) or ranges (`"200..800"`)
- `ital`: tuples like `(1, 700)` → italic, weight
- `ital,wght@...` style gets auto-generated
- `display=swap` is always added to the URL (sensible default)

---

## 🧱 Example Output

```text
https://fonts.googleapis.com/css2?
  family=Roboto:ital,wght@1,700&
  family=Manrope:wght@200..800&
  display=swap
```

---

## 📁 Repo & Contributions

📦 Crate: [crates.io/crates/dioxus-google-fonts](https://crates.io/crates/dioxus-google-fonts)  
🛠️ Repo: [github.com/dsplce-co/dioxus-google-fonts](https://github.com/dsplce-co/dioxus-google-fonts)

PRs welcome! Let’s make Dioxus + typography best friends. 🖤

---

## 📄 License

MIT or Apache-2.0, at your option.
