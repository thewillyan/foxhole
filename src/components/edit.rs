use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, AttrValue, Callback, Html, NodeRef, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct EditFormProps {
    pub labels: Vec<AttrValue>,
    pub hidden: bool,
    pub hide: Callback<bool>,
    pub save: Callback<Vec<String>>,
}

#[function_component(EditForm)]
pub fn edit_form(
    EditFormProps {
        labels,
        hidden,
        hide,
        save,
    }: &EditFormProps,
) -> Html {
    let input_refs = vec![NodeRef::default(); labels.len()];
    let inputs: Html = labels
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let input_ref = &input_refs[i];
            html! {
                <div key={format!("form-item-{i}")}>
                    <label>{ label }</label>
                    <input type="text" name={format!("input{i}")} ref={input_ref}/>
                </div>
            }
        })
        .collect();

    let save_on_click = {
        let save = save.clone();
        Callback::from(move |_| {
            let input_values: Vec<String> = input_refs
                .iter()
                .map(|input| input.cast::<HtmlInputElement>().unwrap().value())
                .collect();
            save.emit(input_values);
        })
    };

    let hidden = *hidden;
    let hide_on_click = {
        let hide = hide.clone();
        Callback::from(move |_| hide.emit(hidden))
    };

    html! {
        <div class={classes!("edit-form")} {hidden}>
            { inputs }
            <div class={classes!("buttons")}>
                <button onclick={save_on_click}>{"Save"}</button>
                <button onclick={hide_on_click}>{"Cancel"}</button>
            </div>
        </div>
    }
}
