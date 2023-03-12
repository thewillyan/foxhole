use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, AttrValue, Callback, Html, NodeRef, Properties};

#[derive(Clone, PartialEq)]
pub struct Input {
    label: AttrValue,
    value: Option<AttrValue>,
    place_holder: Option<AttrValue>,
}

impl Input {
    pub fn new(label: AttrValue) -> Self {
        Input {
            label,
            value: None,
            place_holder: None,
        }
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.value = Some(value);
        self
    }

    #[allow(dead_code)]
    pub fn place_holder(mut self, place_holder: AttrValue) -> Self {
        self.place_holder = Some(place_holder);
        self
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct EditFormProps {
    pub inputs: Vec<Input>,
    pub hidden: bool,
    pub save: Callback<Option<Vec<String>>>,
}

#[function_component(EditForm)]
pub fn edit_form(
    EditFormProps {
        inputs,
        hidden,
        save,
    }: &EditFormProps,
) -> Html {
    let input_refs: Vec<_> = (0..inputs.len()).map(|_| NodeRef::default()).collect();

    let inputs: Html = inputs
        .iter()
        .enumerate()
        .map(|(i, input)| {
            let name = format!("input{i}");
            let placeholder = &input.place_holder;
            let value = &input.value;
            let input_ref = &input_refs[i];
            html! {
                <div key={format!("form-item-{i}")}>
                    <label>{ &input.label }</label>
                    <input type="text" {name} {placeholder} {value} ref={input_ref}/>
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
            save.emit(Some(input_values));
        })
    };
    let cancel_on_click = {
        let save = save.clone();
        Callback::from(move |_| save.emit(None))
    };

    html! {
        <div class={classes!("edit-form")} hidden={*hidden}>
            { inputs }
            <div class={classes!("buttons")}>
                <button onclick={save_on_click}>{"Save"}</button>
                <button onclick={cancel_on_click}>{"Cancel"}</button>
            </div>
        </div>
    }
}
