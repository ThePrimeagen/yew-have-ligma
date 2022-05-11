use yew::{Html, Properties, function_component, html};

#[derive(Properties, PartialEq, Debug)]
pub struct ItemProps {
    pub girth: usize,
    pub depth: usize,
    pub value: u32,
    pub bit: usize,
}

fn has_background(props: &ItemProps) -> bool {
    let mask = 0x1 << props.bit.saturating_sub(1);
    return props.value & mask > 0;
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
                girth={props.girth.saturating_sub(3)}
                bit={props.bit}
                depth={props.depth.saturating_sub(1)}
                value={props.value} />
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

