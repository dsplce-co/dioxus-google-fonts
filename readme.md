> We're dsplce.co, check out our work on our website: [dsplce.co](https://dsplce.co) 🖤

# dioxus-google-fonts

[![Dioxus](https://img.shields.io/badge/Dioxus-000000?style=for-the-badge&logo=rust&logoColor=white)](https://dioxuslabs.com/)
[![crates.io downloads](https://img.shields.io/crates/d/dioxus-google-fonts?style=for-the-badge&color=%23FF0346)](https://crates.io/crates/dioxus-google-fonts)
[![crates.io size](https://img.shields.io/crates/size/dioxus-google-fonts?style=for-the-badge)](https://crates.io/crates/dioxus-google-fonts)
[![License](https://img.shields.io/crates/l/dioxus-google-fonts.svg?style=for-the-badge)](https://crates.io/crates/dioxus-google-fonts)
[![crates.io](https://img.shields.io/crates/v/dioxus-google-fonts?style=for-the-badge&color=%230F80C1)](https://crates.io/crates/dioxus-google-fonts)

💅 Declarative [Google Fonts](https://fonts.google.com) embedding for [Dioxus](https://dioxuslabs.com) — done at compile time, the way you'd want it.

`dioxus-google-fonts` gives you two macros so you never hand-write a `fonts.googleapis.com` URL again — you declare the families, weights and italics you want, and the right `<link>` falls out at compile time.

- `google_fonts_url!` — builds the Google Fonts URL (a plain `&'static str`)
- `google_fonts!` — renders a ready-to-drop `document::Link` node for `rsx!`

---

## 🖤 Features

- **Built at compile time** — the URL is baked into the binary as a `&'static str`; zero runtime cost, no string-fiddling
- **Declarative, tuple-based syntax** — declare families, weights and italics; the macro writes the URL
- **Multiple fonts in one call** — weights and italics across as many families as you like
- **Axis string generated for you** — the fiddly `wght@...` / `ital,wght@...` part is assembled automatically
- **`display=swap` by default** — the sensible default you'd otherwise forget, always appended
- **Web + SSR** — works out of the box with Dioxus on both

---

## Table of Contents

- [🖤 Features](#-features)
- [📦 Installation](#-installation)
- [🧪 Usage](#-usage)
  - [Generate a Google Fonts URL](#generate-a-google-fonts-url)
  - [Drop a `<link>` straight into `rsx!`](#drop-a-link-straight-into-rsx)
- [🧠 Syntax](#-syntax)
  - [A note on `ital`](#a-note-on-ital)
- [🛠️ Compatibility](#%EF%B8%8F-compatibility)
- [📁 Repo & Contributions](#-repo--contributions)
- [📄 License](#-license)

⸻

## 📦 Installation

Add it with cargo:

```bash
cargo add dioxus-google-fonts
```

Or drop it into your `Cargo.toml` by hand:

```toml
[dependencies]
dioxus-google-fonts = "0.1"
```

It's a procedural-macro crate, so it quietly pulls in `syn`, `quote` and `proc-macro2` — nothing for you to wire up. It carries **no `dioxus` dependency of its own**, so a single version works across Dioxus releases — see the [compatibility table](#%EF%B8%8F-compatibility).

⸻

## 🧪 Usage

### Generate a Google Fonts URL

`google_fonts_url!` expands to a string literal, so there's nothing to evaluate at runtime — what you see is what ends up in the binary:

```rust
use dioxus_google_fonts::google_fonts_url;

let url = google_fonts_url!([
    ("Open Sans", wght = [400, 700]),
    ("Mukta", wght = ["200..900"])
]);

assert_eq!(
    url,
    "https://fonts.googleapis.com/css2?family=Open+Sans:wght@400;700&family=Mukta:wght@200..900&display=swap"
);
```

Spaces in font names are handled for you (`Open Sans` → `Open+Sans`), and `display=swap` is appended whether you remember it or not.

### Drop a `<link>` straight into `rsx!`

`google_fonts!` wraps the URL in a `document::Link`, so you can embed it directly in your component — no manual `<link rel="stylesheet">` plumbing:

```rust
use dioxus::prelude::*;
use dioxus_google_fonts::google_fonts;

fn App() -> Element {
    rsx! {
        { google_fonts!([
            ("Roboto", ital = [(0, 400), (1, 700)]),
            ("Mukta", wght = ["200..900"])
        ]) }

        h1 { "Hello, typographically!" }
    }
}

fn main() {
    dioxus::launch(App);
}
```

⸻

## 🧠 Syntax

Each font is a tuple — a name, then any axes you want:

```rust
("Font Name", wght = [weights...], ital = [(italic_flag, weight)...])
```

- `wght` — weights as integers (`400`, `700`) or range strings (`"200..900"`)
- `ital` — tuples of `(italic_flag, weight)`, e.g. `(0, 400)` for upright, `(1, 700)` for bold italic
- the `wght@...` / `ital,wght@...` axis string is generated for you
- `display=swap` is always tacked onto the final URL

### A note on `ital`

Heads up — right now `ital` and `wght` don't get merged into one combined `ital,wght@0,400;1,700` axis. The moment a family has an `ital` list, **those tuples become the source of truth** for it, and any standalone `wght` numbers on that same family are ignored. So put *every* weight you want — upright and italic alike — into the `ital` list:

```rust
// Roboto: regular 400 + bold italic 700
google_fonts_url!([
    ("Roboto", ital = [(0, 400), (1, 700)])
]);
// → ...family=Roboto:ital,wght@0,400;1,700&display=swap
```

Use `wght` on its own when you don't need italics, and `ital` on its own when you do.

⸻

## 🛠️ Compatibility

| Dioxus version | `dioxus-google-fonts` version |
|:---------------|:------------------------------|
| `0.7`          | `0.1`                         |
| `0.6`          | `0.1`                         |

Built on **Rust edition 2024**.

⸻

## 📁 Repo & Contributions

🛠️ **Repo**: [https://github.com/dsplce-co/dioxus-google-fonts](https://github.com/dsplce-co/dioxus-google-fonts)<br>
📦 **Crate**: [https://crates.io/crates/dioxus-google-fonts](https://crates.io/crates/dioxus-google-fonts)

PRs welcome — let's make Dioxus and typography best friends. 🖤

⸻

## 📄 License

MIT or Apache-2.0, at your option.
