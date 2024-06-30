/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

#![allow(non_snake_case)]

// ----------------------------------------------------------------
use dioxus::prelude::*;

use hello_dioxus::{cpt, event, props, rsx};

// ----------------------------------------------------------------

fn main() {
    launch(App);
}

// ----------------------------------------------------------------

fn App() -> Element {
    rsx! {
        rsx::Rsx {}

        // ----------------------------------------------------------------

        cpt::About {}

        // ----------------------------------------------------------------

        props::Likes { score: 42 }

        props::Title {
            title: "Some Title",
            subtitle: "Some Subtitle"
        }

        props::ExplicitOption {
            title: "Explicit::Some Title",
            subtitle: Some("Explicit::Some SubTitle".to_string())
        }

        props::DefaultComponent {}

        props::IntoComponent {
            text: "some &str"
        }

        props::TitleCard {
            title: "TitleCard"
        }

        props::TitleCardCpt {
            title: "Cpt.TitleCard"
        }

        props::Clickable {
            href: "https://www.youtube.com/watch?v=C-M2hs3sXGo",
            body: rsx! {
                "How to " i { "not" } " be seen"
            }
        }

        props::ChildrenClickable {
            href: "https://www.youtube.com/watch?v=C-M2hs3sXGo",
            "How to "
            i { "not" }
            " be seen"
        }

        // ----------------------------------------------------------------

        button {
            onclick: move |event| println!("Clicked! {event:?}"), "click me!"
        }

        div {
            onclick: move |_event| {},
            "outer"
            button {
                onclick: move |event| {
                    event.stop_propagation();
                },
                "inner"
            }
        }

        a {
            href: "https://example.com",
            prevent_default: "onclick",
            onclick: |_| log::info!("link clicked"),
            "example.com"
        }

        event::FancyButton {
            onclick: move |event| println!("Clicked! {event:?}"),
        }

        event::FancyButton {
            onclick: move |event| {
                spawn(async move {
                    println!("Clicked! {event:?}");
                });
            },
        }

        event::CustomFancyButton {
            onclick: move |event| {
                spawn(async move {
                    println!("Clicked! {event:?}");
                });
            },
        }
    }
}
