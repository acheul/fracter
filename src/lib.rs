use wasm_bindgen::prelude::*;
use sycamore::prelude::*;
pub use seoul::Isomorphism;


impl<T: Isomorphism + Default + PartialEq + Clone> Fracter for T {
  fn name(&self) -> Option<String> {
    if self==&Self::default() {
      None
    } else {
      Some(self.title().to_lowercase())
    }
  }

  fn from(name: &str) -> Self {
    let name = name.to_lowercase();
    for x in Self::list().iter() {
      if let Some(t) = x.name() {
        if name==t {
          return x.clone();
        }
      }
    }
    Self::default()
  }
}

pub trait Fracter: Default + PartialEq {

  /// default -> None
  fn name(&self) -> Option<String>;

  fn from(name: &str) -> Self;

  fn get_hash() -> String {
    let x = gloo_utils::window().location().hash().unwrap_throw();
    x.replace("#", "")
  }

  fn from_hash() -> Self {
    Self::from(&Self::get_hash())
  }
  
  fn set_hash(x: &str) {
    gloo_utils::window().location().set_hash(x).unwrap_throw()
  }

  fn init() -> Signal<Self> {

    let fracter = create_signal(Self::default());
    provide_context(fracter);

    on_mount(move || {

      let f = Self::from_hash();
      if fracter.with(|x| x!=&f) {
        fracter.set(f);
      }

      create_effect(on(fracter, move || {
        if let Some(name) = fracter.with(|x| x.name()) {
          if Self::get_hash() != name {
            Self::set_hash(&name);
  
            let history = gloo_utils::window().history().unwrap_throw();
            history.push_state_with_url(&JsValue::NULL, &name, Some(&format!("#{}", name))).unwrap_throw();
          }
        } else {
          Self::set_hash("");
        }
      }));

      let cb = Closure::<dyn FnMut(_)>::new(move |_: web_sys::PopStateEvent| {
        let f = Self::from_hash();
        if fracter.with(|x| x!=&f) {
          fracter.set(f);
        }
      });
      let window = gloo_utils::window();
      window.add_event_listener_with_callback("popstate", cb.as_ref().unchecked_ref()).unwrap_throw();
  
      on_cleanup(move || {
        window.remove_event_listener_with_callback("popstate", cb.as_ref().unchecked_ref()).unwrap_throw();
      });
    });

    fracter
  }
}