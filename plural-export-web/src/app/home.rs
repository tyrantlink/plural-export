use dioxus::prelude::*;
use plural_export::{
    Export,
    ExportType,
    models::{PluralKitExportV1, PluralKitExportV2, TupperboxExport}
};

use crate::components::ThemeToggle;


fn convert(input: &str, format: &str) -> (String, Option<String>) {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return (String::new(), None);
    }

    let export = match serde_json::from_str::<Export>(trimmed) {
        Ok(export) => export,
        Err(error) => return (format!("Error: {error}"), None)
    };

    let types = ExportType::list();
    let Some(export_type) = types.get(format) else {
        return (format!("Unknown format: {format}"), None);
    };

    let converted = match export_type {
        ExportType::PluralKitV1 => {
            export.transmute::<PluralKitExportV1>(&mut Vec::new())
        }
        ExportType::PluralKitV2 => {
            export.transmute::<PluralKitExportV2>(&mut Vec::new())
        }
        ExportType::Tupperbox => {
            export.transmute::<TupperboxExport>(&mut Vec::new())
        }
    };

    (
        serde_json::to_string_pretty(&converted)
            .unwrap_or_else(|error| format!("Serialization error: {error}")),
        Some(export_type.to_string())
    )
}

#[component]
pub fn Home() -> Element {
    let mut input = use_signal(String::new);
    let mut output = use_signal(String::new);
    let mut detected_type = use_signal(|| None);
    let mut format = use_signal(|| {
        ExportType::list()
            .keys()
            .copied()
            .next()
            .unwrap_or("PluralKit v2")
            .to_owned()
    });
    #[cfg(target_arch = "wasm32")]
    let mut debounce_handle: Signal<Option<i32>> = use_signal(|| None);

    let format_options = ExportType::list();
    let mut sorted_formats: Vec<&'static str> =
        format_options.keys().copied().collect();
    sorted_formats.sort_unstable();

    rsx! {
      div { class: "flex flex-col h-screen gap-4 p-4",
        div { class: "flex items-center justify-between shrink-0",
          span { class: "text-lg font-semibold text-[color:var(--fg)]",
            "silly little export converter"
          }
          ThemeToggle {}
        }
        div { class: "grid grid-cols-[1fr_1fr] gap-4 flex-1 min-h-0",
          div { class: "flex flex-col gap-2 min-h-0",
            div { class: "flex items-center justify-between shrink-0 h-8",
              span { class: "text-sm font-medium text-[color:var(--muted-fg)] uppercase tracking-wider",
                "Input"
              }
              // ? you can't use a raw identifier here i am enrage
              if let Some(type_) = detected_type() {
                span { class: "text-[0.8rem] text-[color:var(--muted-fg)]",
                  "Detected: {type_}"
                }
              }
            }
            textarea {
              class: "text-box focus:border-[color:var(--muted-fg)]",
              placeholder: "format input.",
              spellcheck: false,
              oninput: move |event: Event<FormData>| {
                  let value = event.value();

                  #[cfg(target_arch = "wasm32")]
                  {
                      use wasm_bindgen::{JsCast, prelude::*};

                      if let Some(handle) = debounce_handle() {
                          let window = web_sys::window()
                              .expect("window should be available on web");
                          window.clear_timeout_with_handle(handle);
                      }

                      input
                          .set(
                              serde_json::from_str::<serde_json::Value>(value.trim())
                                  .and_then(|value| serde_json::to_string_pretty(&value))
                                  .unwrap_or_else(|_| value.clone()),
                          );
                      let fmt = format();
                      let closure = Closure::once(move || {
                          let (converted, r#type) = convert(&value, &fmt);
                          output.set(converted);
                          detected_type.set(r#type);
                      });
                      let window = web_sys::window().expect("window should be available on web");
                      let handle = window
                          .set_timeout_with_callback_and_timeout_and_arguments_0(
                              closure.as_ref().unchecked_ref(),
                              50,
                          )
                          .unwrap();
                      closure.forget();
                      debounce_handle.set(Some(handle));
                  }
                  #[cfg(not(target_arch = "wasm32"))]
                  {
                      let (converted, r#type) = convert(&value, &format());
                      output.set(converted);
                      detected_type.set(r#type);
                      input.set(value);
                  }
              },
              value: input(),
            }
          }
          div { class: "flex flex-col gap-2 min-h-0",
            div { class: "flex items-center justify-between shrink-0 h-8",
              span { class: "text-sm font-medium text-[color:var(--muted-fg)] uppercase tracking-wider",
                "Output"
              }
              select {
                class: "format-select focus:border-[color:var(--muted-fg)]",
                onchange: move |event| {
                    let selected = event.value();
                    let (converted, r#type) = convert(&input(), &selected);
                    output.set(converted);
                    detected_type.set(r#type);
                    format.set(selected);
                },
                for label in sorted_formats {
                  option {
                    value: label,
                    selected: format() == label,
                    {label}
                  }
                }
              }
            }
            textarea {
              class: "text-box cursor-default focus:border-[color:var(--muted-fg)]",
              placeholder: "aaaaaaaaaaaaaaaaaaaaaa converted output",
              spellcheck: false,
              readonly: true,
              value: output(),
            }
          }
        }
      }
    }
}
