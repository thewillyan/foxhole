use yew::{function_component, html, Html, classes};

mod components;

use components::{Bar, Greeting};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class={classes!("app")}>
            <Bar/> 
            <header>
                <h1>{ "Foxhole" }</h1>
                <Greeting/>
            </header>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
