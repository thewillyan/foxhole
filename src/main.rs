use yew::{classes, function_component, html, Html};

mod components;

use components::{Bar, Greeting, cards::Cards};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class={classes!("app")}>
            <Bar/>
            <header>
                <h1>{ "Foxhole" }</h1>
                <Greeting/>
            </header>
            <Cards/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
