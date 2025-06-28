use dioxus::prelude::*;
mod components;
mod backend;
use crate::components::*;
use backend::save_dog;

static CSS: Asset = asset!("/assets/main.css");


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn Title() -> Element {

    rsx! {
        div {
            id: "title",
            h1 { "hot dog "}
        }
    }
} 

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn DogView() -> Element {
   
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .unwrap()
            .json::<DogApi>()
            .await
        .unwrap()
            .message
    });
   rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
           button {
                id: "save",
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = save_dog(current).await;
                },
                "save"
            }
    
    
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet {href: CSS}
        //Title {}
        //DogView {}
        Router::<Route> {}
    }
}


#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,
    #[route("/favorites")]
    Favorites,
}


