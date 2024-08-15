use yew::prelude::*;
use wasm_bindgen::prelude::*;

// Define your main component
struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { "Welcome to the Decentralized Book Manager!" }
            </div>
        }
    }
}

// Initialize the application
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
