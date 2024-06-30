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

// ----------------------------------------------------------------

pub fn Rsx() -> Element {
    let large_font = true;

    let comments = "live <b>dangerously</b>";

    let coordinates = (42, 0);
    let country = "es";

    let txt = "Dioxus";

    rsx! {
        div {
            "style": "width: 20px; height: 20px; background-color: red;",
        }

        div {
            class: if large_font {
                "text-xl"
            },
            "Hello, world!"
        }

        button {
            // attributes / listeners
            // children
            "Hello, World!"
        }

        img {
            src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
            class: "primary_button",
            width: "10px"
        }

        input {
            placeholder: "Enter your name",
            value: "photowey",
            oninput: |_event| {
                println!("Input changed!");
            },
            r#type: "text",
            color: "red"
        }

        div {
            dangerous_inner_html: comments
        }

        div {
            hidden: false,
            "Hello, photowey!"
        }

        div {
            class: "country-{country}",
            left: "{coordinates.0}",
            top: "{coordinates.1}",
            // arbitrary expressions are allowed,
            // as long as they don't contain `{}`
            div { "{country.to_uppercase()}" },
            div { "7*6" }
            // {} can be escaped with {{}}
            div { "{{}}" }
        }

        ol {
            li { "First Item" }
            li { "Second Item" }
            li { "Third Item" }
        }

        p { "First Item" }
        p { "Second Item" }

        span {
            { txt.to_uppercase() },
            { (0..10).map(|i| rsx!{ "{i}" }) }
        }

        div {
            for i in 0..3 {
                div { "{i}" }
            }
        }

        div {
            { (0..3).map(|i| rsx! { div { "{i}" } }) }
        }

        if true {
            div { "statement: true" }
        }
    }
}
