use std::{fmt::Display, rc::Rc};

use gloo::timers::callback::Interval;
use yew::{function_component, html, Properties, use_state, use_effect_with_deps};

use crate::{item::Item, buffer::random_buffer};
use anyhow::Result;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub size: usize,
    pub depth: usize,
    pub interval: u64,
}

impl Default for AppProps {
    fn default() -> AppProps {
        return AppProps {
            size: 69,
            depth: 7,
            interval: 250,
        }
    }
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let render_count = use_state(|| 0);
    let buffer_me_daddy = Rc::new(random_buffer(props.size));

    use_effect_with_deps(move |_| {
        let handle = Interval::new(1000, move || {
            render_count.set(*render_count + 1);
        });
        return move || {
            handle.cancel();
        };
    }, ());

    let items = (0..props.size).map(|i| { return html! {
        <Item
            girth={props.size}
            bit={i}
            depth={7}
            buffer={buffer_me_daddy.clone()} /> };
    });

    return html! {
        <>
            {for items}
        </>
    };
}

