use dioxus::prelude::*;

#[component]
pub fn Introduction() -> Element {
    rsx! {
        div { class: "w-full h-[calc(100vh-4rem)]",
            iframe {
                class: "w-full h-full border-none",
                src: "/assets/project-defense.html",
            }
        }
    }
}
