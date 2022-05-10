use gloo::timers::callback::Timeout;
use yew::{Html, function_component, html, Properties, use_state, use_effect_with_deps, UseStateHandle};

use crate::item::Item;

#[derive(Properties, PartialEq, Debug)]
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

#[function_component(ReactSucks)]
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
            depth={props.depth}
            value={count} /> };
    });

    return html! {
        <>
            {for items}
        </>
    };
}

