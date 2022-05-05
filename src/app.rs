use std::{fmt::Display, rc::Rc};

use gloo::timers::callback::{Interval, Timeout};
use styled_yew::web_sys::console;
use yew::{function_component, html, Properties, use_state, use_effect_with_deps, UseStateHandle};

use crate::{item::Item, buffer::random_buffer};
use anyhow::Result;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub size: usize,
    pub depth: usize,
    pub interval: u32,
}

impl Default for AppProps {
    fn default() -> AppProps {
        return AppProps {
            size: 69,
            depth: 7,
            interval: 3,
        }
    }
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let render_count: UseStateHandle<u32> = use_state(|| 0);
    let interval = props.interval;

    let count = *render_count;

    use_effect_with_deps(move |_| {
        let handle = Timeout::new(interval, move || {
            render_count.set(*render_count + 1);
        });
        return move || {
            handle.cancel();
        };
    }, count);

    let items = (0..32).map(|i| { return html! {
        <Item
            girth={props.size}
            bit={i}
            depth={7}
            value={count} /> };
    });

    return html! {
        <div style={"background-color: #555; width: 100%; height: 100%; position: absolute;"}>
            {for items}
        </div>
    };
}

