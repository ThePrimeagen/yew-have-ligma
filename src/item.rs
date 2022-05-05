use std::{rc::Rc, cmp::max};

use yew::{Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub girth: usize,
    pub depth: usize,
    pub buffer: Rc<Vec<u8>>,
    pub bit: usize,
}

fn has_background(props: &ItemProps) -> bool {
    let offset_byte = props.bit / 8;
    let offset_bit = 0x1 << (max(0, (props.bit % 8) - 1));
    return props.buffer[offset_byte] & offset_bit > 0;
}

fn render_inners(props: &ItemProps) -> yew::Html {
    if props.depth == 0 {
        return html! {
            {props.bit}
        };
    }

    return html! {
        <>
            <Item
                girth={props.girth - 3}
                bit={props.bit}
                depth={props.depth - 1}
                buffer={props.buffer.clone()} />
        </>
    };
}

#[function_component(Item)]
pub fn item(props: &ItemProps) -> Html {
    let is_even: bool = props.depth & 0x1 != 1;
    let mut color = "#ff9500";
    if is_even == has_background(props) {
        color = "#0385ff";
    }

    let girth = props.girth;

    return html! {
        <div
            style={format!("border: 1px solid #DDD;height: 69px;width: {girth}px; background-color: {color}")}>
            {render_inners(props)}
        </div>
    };
}

