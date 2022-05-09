use react_vs_wasm_yew::app::ReactSucks;
use yew::Renderer;

fn main() {
    let renderer = Renderer::<ReactSucks>::new();

    // hydrates everything under body element, removes trailing
    // elements (if any).
    renderer.hydrate();
}

