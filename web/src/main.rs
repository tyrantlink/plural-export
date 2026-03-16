#![feature(const_default)]
#![feature(const_trait_impl)]

#[cfg(feature = "server")]
mod api;
mod app;
mod components;
mod hooks;

use self::app::App;


fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        #[expect(clippy::absolute_paths, reason = "i no no wanna")]
        let router = dioxus::server::router(App);

        Ok(router)
    })
}
