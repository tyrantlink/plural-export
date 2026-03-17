use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Black,
    Dark,
    Light,
    White
}

impl Theme {
    // pub const fn dark_themes() -> [Self; 2] {
    //     [Self::Dark, Self::Black]
    // }

    // pub const fn light_themes() -> [Self; 2] {
    //     [Self::Light, Self::White]
    // }


    pub const fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }

    pub const fn toggle_gradient(self) -> Self {
        match self {
            Self::Light => Self::White,
            Self::White => Self::Light,
            Self::Dark => Self::Black,
            Self::Black => Self::Dark
        }
    }
}


#[cfg(not(target_arch = "wasm32"))]
impl Theme {
    #[expect(clippy::unused_self, reason = "TODO maybe i dunno")]
    pub const fn apply(self) {}

    pub const fn read_stored() -> Self {
        Self::default()
    }
}

#[cfg(target_arch = "wasm32")]
impl Theme {
    pub fn apply(self) {
        let Some(window) = web_sys::window() else {
            return;
        };

        if let Some(document) = window.document() &&
            let Some(element) = document.document_element()
        {
            let _ = element.set_attribute("data-theme", self.as_str());
        }

        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("theme", self.as_str());
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::Black => "black",
            Self::White => "white"
        }
    }

    pub fn read_stored() -> Self {
        web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("theme").ok().flatten())
            .and_then(|theme| std::str::FromStr::from_str(&theme).ok())
            .unwrap_or_default()
    }
}

impl const Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

impl std::str::FromStr for Theme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "black" => Ok(Self::Black),
            "white" => Ok(Self::White),
            _ => Err(())
        }
    }
}

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let theme = use_signal(Theme::read_stored);
    use_context_provider(|| theme);

    use_effect(move || {
        theme().apply();
    });

    rsx! {
        {children}
    }
}

pub fn use_theme() -> Signal<Theme> {
    use_context::<Signal<Theme>>()
}
