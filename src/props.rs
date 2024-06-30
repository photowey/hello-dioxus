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

#[derive(PartialEq, Clone, Props)]
pub struct LikesProps {
    score: i32,
}

#[derive(PartialEq, Clone, Props)]
pub struct OptionalProps {
    title: String,
    subtitle: Option<String>,
}

#[derive(PartialEq, Clone, Props)]
pub struct ExplicitOptionProps {
    title: String,
    #[props(! optional)]
    subtitle: Option<String>,
}

#[derive(PartialEq, Clone, Props)]
pub struct DefaultProps {
    #[props(default = 42)]
    number: i64,
}

#[derive(PartialEq, Props, Clone)]
pub struct IntoProps {
    #[props(into)]
    text: String,
}

#[derive(PartialEq, Clone, Props)]
pub struct TitleCardProps {
    title: String,
}

#[derive(PartialEq, Clone, Props)]
pub struct ClickableProps {
    href: String,
    body: Element,
}

#[derive(PartialEq, Clone, Props)]
pub struct ChildrenClickableProps {
    href: String,
    children: Element,
}

// ----------------------------------------------------------------

pub fn Likes(props: LikesProps) -> Element {
    rsx! {
        div {
            "This post has "
            b { "{props.score}" }
            " likes"
        }
    }
}

pub fn Title(props: OptionalProps) -> Element {
    rsx! {
        h1 {
            "{props.title}: ",
            {
                props.subtitle.unwrap_or_else(|| "No subtitle provided".to_string())
            }
        }
    }
}

pub fn ExplicitOption(props: ExplicitOptionProps) -> Element {
    rsx! {
        h1 {
            "{props.title}: ",
            {
                props.subtitle.unwrap_or_else(|| "No subtitle provided".to_string())
            }
        }
    }
}

pub fn DefaultComponent(props: DefaultProps) -> Element {
    rsx! {
        h1 {
            "{props.number}"
        }
    }
}

pub fn IntoComponent(props: IntoProps) -> Element {
    rsx! {
        h1 {
            "{props.text}"
        }
    }
}

pub fn TitleCard(props: TitleCardProps) -> Element {
    rsx! {
        h1 { "{props.title}" }
    }
}

#[component]
pub fn TitleCardCpt(title: String) -> Element {
    rsx! {
        h1 { "{title}" }
    }
}

pub fn Clickable(props: ClickableProps) -> Element {
    rsx! {
        a {
            href: "{props.href}",
            class: "fancy-button", { props.body }
        }
    }
}

pub fn ChildrenClickable(props: ChildrenClickableProps) -> Element {
    rsx! {
        a {
            href: "{props.href}",
            class: "fancy-button", { props.children }
        }
    }
}
