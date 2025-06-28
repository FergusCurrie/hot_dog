use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
            id: "title",
            Link {
                to: Route::DogView,
                h1 { "Hot dog "}
            }
            Link {
                to: Route::Favorites, id: "heart", "heart"
            }
        }
        Outlet::<Route> {} // to render under this 
    }
}
