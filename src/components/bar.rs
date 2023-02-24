use yew::{Html, html, function_component, classes};

#[function_component(Bar)]
pub fn bar() -> Html {
    html! {
        <div class={classes!("bar")}>
            <button>{ "Change Theme" }</button>
        </div>
    }
}

