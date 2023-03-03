use web_sys::MouseEvent;
use yew::{Html, html, function_component, classes, Callback, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct BarProps {
    pub toggle_theme: Callback<MouseEvent>
}

#[function_component(Bar)]
pub fn bar(BarProps { toggle_theme }: &BarProps) -> Html {
    html! {
        <div class={classes!("bar")}>
            <button onclick={(*toggle_theme).clone()}>{ "Change Theme" }</button>
        </div>
    }
}

