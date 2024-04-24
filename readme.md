# 🍫 Fracter

Fragment hash based router, for Sycamore

[![Crates.io](https://img.shields.io/crates/v/fracter?style=flat-square)](https://crates.io/crates/fracter)
[![docs.rs](https://img.shields.io/docsrs/fracter?style=flat-square)](https://docs.rs/fracter)

[Live demo](https://acheul.github.io/fracter/)

`Fracter` trait will be default implemented for [`Isomorphism`](https://crates.io/crates/seoul) + `Default` + `PartialEq` implemented enum type.

Supports Sycamore version 0.9.0 or later.

# Ex
```rust
use sycamore::prelude::*;
use fracter::{Fracter, Isomorphism};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Isomorphism)]
#[isomorphism(list=[Index, Notice, Contact])]
pub enum Fragment {
  #[default] Index,
  Notice,
  Contact
}

#[component]
pub fn App<G: Html>() -> View<G> {

  // `init` will handle everything: captures hash change, updates history, etc.
  let fragment = Fragment::init();

  view! {
    main() {
      // Use fragment as if router
      (match fragment.get() {
        Fragment::Notice => view! { "📢 Notice" },
        Fragment::Contact => view! { "🔭 Contact" },
        _ => view! { "✅ Index" },
      })
    }
  }
}
```