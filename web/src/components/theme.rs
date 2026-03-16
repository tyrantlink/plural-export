use daisy_rsx::ToolTip;
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdMoon, LdSun}
};

use crate::hooks::use_theme;

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme = use_theme();
    let mut animate = use_signal(|| false);

    rsx! {
      ToolTip {
        text: "Click: Light / Dark\n\nShift+Click: Toggle Gradient",
        class: "tooltip-bottom",
        button {
          class: "btn btn-ghost btn-square theme-toggle-btn",
          aria_label: "Toggle theme",
          onclick: move |event: MouseEvent| {
              animate.set(true);
              *theme.write() = if event.modifiers().shift() {
                  theme().toggle_gradient()
              } else {
                  theme().toggle()
              };
          },
          // TODO do this less bad
          Icon {
            icon: LdSun,
            class: if animate() { "animate-[theme-icon-spin_200ms_ease-out_forwards] absolute hidden [[data-theme=light]_&]:block [[data-theme=white]_&]:block" } else { "absolute hidden [[data-theme=light]_&]:block [[data-theme=white]_&]:block" },
          }
          Icon {
            icon: LdMoon,
            class: if animate() { "animate-[theme-icon-spin_200ms_ease-out_forwards] absolute [[data-theme=light]_&]:hidden [[data-theme=white]_&]:hidden" } else { "absolute [[data-theme=light]_&]:hidden [[data-theme=white]_&]:hidden" },
          }
        }
      }
    }
}
