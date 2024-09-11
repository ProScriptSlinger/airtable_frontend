extern crate reqwest_wasm;

use bounce::BounceRoot;
use yew::prelude::*;

mod components;
mod controllers;
mod apis;


#[function_component]
fn App() -> Html {
    html! {
        <BounceRoot>
            <main>
                <div class="bg-[#191919]">
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
