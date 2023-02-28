use crate::components::{
    cards::cards_ctx::{Anchor, CardsContext, CardsProvider, ModifyCards},
    edit::{EditForm, Input},
};
use yew::{
    classes, function_component, html, use_callback, use_context, use_state_eq, AttrValue,
    Callback, Html, Properties,
};

#[function_component(LinkCards)]
pub fn link_cards() -> Html {
    html! {
        <CardsProvider>
            <CardList />
        </CardsProvider>
    }
}

#[function_component(CardList)]
fn card_list() -> Html {
    let cards = use_context::<CardsContext>().unwrap();

    let mut input = Input::new(AttrValue::from("New card name:"));
    input.set_value(AttrValue::default());
    let inputs = vec![input];

    let hidden = use_state_eq(|| true);
    let hide_form = {
        let hidden = hidden.clone();
        Callback::from(move |_| hidden.set(true))
    };

    let curr_card = use_state_eq(|| None);
    let select_card = {
        let curr_card = curr_card.clone();
        Callback::from(move |id: Option<usize>| curr_card.set(id))
    };

    let change_card = {
        let cards = cards.clone();
        let hidden = hidden.clone();
        use_callback(
            move |values: Vec<String>, id| {
                let name = AttrValue::from(values.into_iter().next().unwrap_or_default());
                let action = match **id {
                    Some(id) => ModifyCards::Rename {
                        card_index: id,
                        new_name: name,
                    },
                    None => ModifyCards::Add(name),
                };
                hidden.set(true);
                cards.dispatch(action);
            },
            curr_card,
        )
    };

    let show_add_form = {
        let hidden = hidden.clone();
        let select_card = select_card.clone();
        Callback::from(move |_| {
            select_card.emit(None);
            hidden.set(false);
        })
    };

    // convert cards into Html
    let cards: Html = (0..cards.inner.len())
        .map(|id| {
            let rename_card = {
                let hidden = hidden.clone();
                let select_card = select_card.clone();
                Callback::from(move |card_id| {
                    select_card.emit(Some(card_id));
                    hidden.set(false);
                })
            };

            html! { <LinkCard key={format!("card{id}")} {rename_card} {id} />}
        })
        .collect();

    html! {
        <div class={classes!("cards")}>
            {cards}
            <EditForm {inputs} hidden={*hidden} save={change_card} cancel={hide_form} />
            <button class={classes!("add-card")} onclick={show_add_form}>{"Add card"}</button>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct LinkCardProps {
    id: usize,
    rename_card: Callback<usize>,
}

#[function_component(LinkCard)]
fn link_card(props: &LinkCardProps) -> Html {
    let cards = use_context::<CardsContext>().unwrap();
    let id = props.id;

    // add link menu
    let mut label_input = Input::new(AttrValue::from("Link label:"));
    label_input.set_value(AttrValue::default());
    let mut url_input = Input::new(AttrValue::from("URL:"));
    url_input.set_value(AttrValue::default());
    let add_link_inputs = vec![label_input, url_input];

    let add_link_hidden = use_state_eq(|| true);
    let show_add_link = {
        let hidden = add_link_hidden.clone();
        Callback::from(move |_| hidden.set(false))
    };
    let hide_add_link = {
        let hidden = add_link_hidden.clone();
        Callback::from(move |_| hidden.set(true))
    };

    let add_link = {
        let cards = cards.clone();
        let hide = hide_add_link.clone();
        Callback::from(move |mut input_values: Vec<String>| {
            let url = input_values.pop().unwrap_or_default();
            let label = input_values.pop().unwrap_or_default();

            if !(url.is_empty() || label.is_empty()) {
                let link = Anchor {
                    label: AttrValue::from(label),
                    url: AttrValue::from(url),
                };
                cards.dispatch(ModifyCards::AddLink {
                    card_index: id,
                    link,
                });
                hide.emit(());
            }
        })
    };

    // rename card
    let rename_on_click = {
        let rename_card = props.rename_card.clone();
        Callback::from(move |_| rename_card.emit(id))
    };

    // remove card
    let rm_card = {
        let cards = cards.clone();
        Callback::from(move |_| cards.dispatch(ModifyCards::Remove(id)))
    };

    let card_name = &cards.inner[id].name;

    html! {
        <div class={classes!("card")}>
            <h3 class={classes!("card-name")}>{ card_name }</h3>
            <Links card_id={id}/>
            <div class={classes!("buttons")}>
                <button onclick={show_add_link}>{ "Add link" }</button>
                <button onclick={rm_card}>{ "Remove Card" }</button>
                <button onclick={rename_on_click}>{ "Rename card" }</button>
            </div>
            <div class={classes!("edit-forms")}>
                <EditForm inputs={add_link_inputs} hidden={*add_link_hidden} save={add_link} cancel={hide_add_link}/>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct LinksProps {
    card_id: usize,
}

#[function_component(Links)]
fn links(props: &LinksProps) -> Html {
    let cards = use_context::<CardsContext>().unwrap();
    let card_id = props.card_id;

    // edit link menu
    let mut label_input = Input::new(AttrValue::from("Label:"));
    label_input.set_value(AttrValue::default());
    let mut url_input = Input::new(AttrValue::from("URL:"));
    url_input.set_value(AttrValue::default());
    let inputs = vec![label_input, url_input];

    let edit_link_hidden = use_state_eq(|| true);
    let toggle_edit_link = {
        let hidden = edit_link_hidden.clone();
        Callback::from(move |_| hidden.set(!*hidden))
    };

    let selected_link = use_state_eq(|| None);
    let change_seleted = {
        let selected_link = selected_link.clone();
        Callback::from(move |link_id: usize| selected_link.set(Some(link_id)))
    };

    let edit_link = {
        let toggle_hide = toggle_edit_link.clone();
        let cards = cards.clone();
        use_callback(
            move |mut values: Vec<String>, link| {
                let link_id = link.unwrap();
                let url = AttrValue::from(values.pop().unwrap_or_default());
                let label = AttrValue::from(values.pop().unwrap_or_default());
                let action = ModifyCards::EditLink {
                    card_index: card_id,
                    link_index: link_id,
                    new_label: if label.is_empty() { None } else { Some(label) },
                    new_url: if url.is_empty() { None } else { Some(url) },
                };
                cards.dispatch(action);
                toggle_hide.emit(());
            },
            selected_link,
        )
    };

    // links into Html
    let links = &cards.inner[card_id].links;
    let links: Html = links
        .iter()
        .enumerate()
        .map(|(i, link)| {
            let Anchor { label, url } = link;

            let show_edit_link = {
                let toggle_hide = toggle_edit_link.clone();
                let change_seleted = change_seleted.clone();
                Callback::from(move |_| {
                    change_seleted.emit(i);
                    toggle_hide.emit(());
                })
            };

            let rm_link = {
                let cards = cards.clone();
                Callback::from(move |_| {
                    let action = ModifyCards::RemoveLink {
                        card_index: card_id,
                        link_index: i,
                    };
                    cards.dispatch(action);
                })
            };

            html! {
                <div class={classes!("card-link")}>
                    <a key={format!("link{i}")} href={url}>{label}</a>
                    <div class={classes!("buttons")}>
                        <button onclick={rm_link}>{ "Remove link" }</button>
                        <button onclick={show_edit_link}>{ "Edit link" }</button>
                    </div>
                </div>
            }
        })
        .collect();

    html! {
        <div class={classes!("links")}>
            {links}
            <EditForm {inputs} hidden={*edit_link_hidden} save={edit_link} cancel={toggle_edit_link}/>
        </div>
    }
}
