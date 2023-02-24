use yew::{classes, function_component, html, use_state, AttrValue, Callback, Html};

use super::EditForm;

#[function_component(Greeting)]
pub fn greeting() -> Html {
    let user_name = use_state(|| AttrValue::from("UserName"));

    let labels = vec![AttrValue::from("Your user name:")];
    let hide_name_form = use_state(|| true);

    let toggle_hide_form = {
        let hide_name_form = hide_name_form.clone();
        Callback::from(move |hide_state: bool| hide_name_form.set(!hide_state))
    };

    let save_name = {
        let name = user_name.clone();
        Callback::from(move |mut values: Vec<String>| {
            let value = values.pop().unwrap_or_default();
            name.set(AttrValue::from(value));
        })
    };

    let on_name_click = {
        let hide_state = *hide_name_form;
        let toggle_hide = toggle_hide_form.clone();
        Callback::from(move |_| toggle_hide.emit(hide_state))
    };

    html! {
        <div class={classes!("greeting")}>
            <p>
                {"Welcome, "}
                <span class={classes!("name")} onclick={on_name_click}>{ (*user_name).clone() }</span>
                {"!"}
            </p>
            <EditForm {labels} hidden={*hide_name_form} hide={toggle_hide_form} save={save_name}/>
        </div>
    }
}
