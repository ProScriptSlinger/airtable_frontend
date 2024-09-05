extern crate reqwest_wasm;

use bounce::BounceRoot;
use yew::prelude::*;

mod components;
mod controllers;


#[function_component]
fn App() -> Html {
    html! {
        <BounceRoot>
            <main>
                <div class="bg-black">
                    <components::sidebar::Sidebar />
                    <components::content::Content />
                </div>
            </main>
        </BounceRoot>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
