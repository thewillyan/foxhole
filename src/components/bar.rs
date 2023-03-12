use yew::{classes, function_component, html, use_context, Callback, Html};

use crate::{CtxAction, GlobalCtx};

#[function_component(Bar)]
pub fn bar() -> Html {
    let ctx = use_context::<GlobalCtx>().unwrap();
    let theme_button_hide = !ctx.editable;

    let toggle_theme = {
        let ctx = ctx.clone();
        Callback::from(move |_| ctx.dispatch(CtxAction::ToggleTheme))
    };
    let toggle_edit = { Callback::from(move |_| ctx.dispatch(CtxAction::ToggleEdit)) };

    html! {
        <div class={classes!("bar")}>
            <button onclick={toggle_theme} hidden={theme_button_hide}>{ "Change Theme" }</button>
            <button onclick={toggle_edit}>{ "Toggle Edit" }</button>
        </div>
    }
}
