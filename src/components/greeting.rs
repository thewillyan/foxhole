use gloo_storage::{LocalStorage, Storage};
use yew::{classes, function_component, html, use_state_eq, AttrValue, Callback, Html};

use super::edit::{EditForm, Input};

#[function_component(Greeting)]
pub fn greeting() -> Html {
    let user_name = use_state_eq(|| {
        // try to get user name from browser local data
        match LocalStorage::get::<String>("user_name") {
            Ok(name) => AttrValue::from(name),
            Err(_) => AttrValue::from("UserName"),
        }
    });

    // setup edit name form
    let mut input = Input::new(AttrValue::from("Your user name:"));
    input.set_value((*user_name).clone());
    let inputs = vec![input];

    let hide_state = use_state_eq(|| true);
    let on_name_click = {
        let hide_state = hide_state.clone();
        Callback::from(move |_| hide_state.set(false))
    };
    let hide = {
        let hide_state = hide_state.clone();
        Callback::from(move |()| hide_state.set(true))
    };

    let save_name = {
        let name = user_name.clone();
        let hide = hide.clone();
        Callback::from(move |values: Vec<String>| {
            let value = values.into_iter().next().unwrap_or_default();
            // store name on browser local storage
            if let Err(err) = LocalStorage::set("user_name", value.clone()) {
                web_sys::console::log_1(&format!("{err}").into());
            };
            name.set(AttrValue::from(value));
            hide.emit(())
        })
    };

    html! {
        <div class={classes!("greeting")}>
            <p>
                {"Welcome, "}
                <span class={classes!("name")} onclick={on_name_click}>{ &(*user_name) }</span>
                {"!"}
            </p>
            <EditForm {inputs} hidden={*hide_state} save={save_name} cancel={hide} />
        </div>
    }
}
