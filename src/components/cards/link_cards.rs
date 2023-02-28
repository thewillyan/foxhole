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

    // targets
    let target_card = use_state_eq(|| None);
    let select_card = {
        let target = target_card.clone();
        Callback::from(move |id: Option<usize>| target.set(id))
    };

    let target_link = use_state_eq(|| None);
    let select_link = {
        let target = target_link.clone();
        Callback::from(move |id: Option<usize>| {
            target.set(id);
        })
    };

    // card form
    let card_name = match *target_card {
        Some(index) => cards.inner[index].name.clone(),
        None => AttrValue::default(),
    };
    let input = Input::new(AttrValue::from("New card name:")).value(card_name);
    let card_inputs = vec![input];

    let card_form_hidden = use_state_eq(|| true);
    let show_card_form = {
        let hidden = card_form_hidden.clone();
        let select_card = select_card.clone();
        Callback::from(move |_| {
            select_card.emit(None);
            hidden.set(false);
        })
    };

    let change_card = {
        let cards = cards.clone();
        let hidden = card_form_hidden.clone();
        use_callback(
            move |inputs_values: Option<Vec<String>>, id| {
                let values = match inputs_values {
                    Some(vals) => vals,
                    None => {
                        hidden.set(true);
                        return;
                    }
                };

                let name = match values.into_iter().next() {
                    Some(val) if !val.is_empty() => AttrValue::from(val),
                    _ => return,
                };

                let action = match **id {
                    Some(id) => ModifyCards::Rename {
                        card_index: id,
                        new_name: name,
                    },
                    None => ModifyCards::Add(name),
                };
                cards.dispatch(action);
                hidden.set(true);
            },
            target_card.clone(),
        )
    };

    // link form
    let (link_label, link_url) = match (*target_card, *target_link) {
        (Some(card_index), Some(link_index)) => {
            let link = &cards.inner[card_index].links[link_index];
            (link.label.clone(), link.url.clone())
        }
        _ => (AttrValue::default(), AttrValue::default()),
    };
    let label_input = Input::new(AttrValue::from("Label:")).value(link_label);
    let url_input = Input::new(AttrValue::from("URL:")).value(link_url);
    let link_inputs = vec![label_input, url_input];

    let link_form_hidden = use_state_eq(|| true);

    let change_link = {
        let cards = cards.clone();
        let hidden = link_form_hidden.clone();
        use_callback(
            move |input_values: Option<Vec<String>>, (card, link)| {
                let card_index = card.unwrap();

                let mut values = match input_values {
                    Some(vals) => vals,
                    None => {
                        hidden.set(true);
                        return;
                    }
                };

                let url = AttrValue::from(values.pop().unwrap_or_default());
                let label = AttrValue::from(values.pop().unwrap_or_default());

                if url.is_empty() && label.is_empty() {
                    return;
                }

                let action = match **link {
                    Some(link_index) => ModifyCards::EditLink {
                        card_index,
                        link_index,
                        new_label: if label.is_empty() { None } else { Some(label) },
                        new_url: if url.is_empty() { None } else { Some(url) },
                    },
                    None => ModifyCards::AddLink {
                        card_index,
                        link: Anchor { label, url },
                    },
                };
                cards.dispatch(action);
                hidden.set(true);
            },
            (target_card, target_link),
        )
    };

    // card callbacks
    let rename_card = {
        let hidden = card_form_hidden.clone();
        let select_card = select_card.clone();
        Callback::from(move |card_id| {
            select_card.emit(Some(card_id));
            hidden.set(false);
        })
    };

    let rm_card = {
        let cards = cards.clone();
        let select_card = select_card.clone();
        let select_link = select_link.clone();
        Callback::from(move |card_id: usize| {
            cards.dispatch(ModifyCards::Remove(card_id));
            select_card.emit(None);
            select_link.emit(None)
        })
    };

    let add_link = {
        let hidden = link_form_hidden.clone();
        let select_card = select_card.clone();
        let select_link = select_link.clone();
        Callback::from(move |card_id| {
            select_card.emit(Some(card_id));
            select_link.emit(None);
            hidden.set(false);
        })
    };

    let edit_link = {
        let hidden = link_form_hidden.clone();
        let select_card = select_card.clone();
        let select_link = select_link.clone();
        Callback::from(move |(card_id, link_id)| {
            select_card.emit(Some(card_id));
            select_link.emit(Some(link_id));
            hidden.set(false);
        })
    };

    let rm_link = {
        let cards = cards.clone();
        Callback::from(move |(card_index, link_index): (usize, usize)| {
            let action = ModifyCards::RemoveLink {
                card_index,
                link_index,
            };
            cards.dispatch(action);
            select_card.emit(None);
            select_link.emit(None);
        })
    };

    // convert cards into Html
    let cards: Html = (0..cards.inner.len())
        .map(|id| {
            html! {
                <LinkCard key={format!("card{id}")} {id} rename_card={rename_card.clone()}
                    rm_card={rm_card.clone()} add_link={add_link.clone()}
                    edit_link={edit_link.clone()} rm_link={rm_link.clone()}
                />
            }
        })
        .collect();

    html! {
        <div class={classes!("cards")}>
            {cards}
            <button class={classes!("add-card")} onclick={show_card_form}>{"Add card"}</button>
            <div class={classes!("forms")}>
                <EditForm inputs={card_inputs} hidden={*card_form_hidden} save={change_card}/>
                <EditForm inputs={link_inputs} hidden={*link_form_hidden} save={change_link}/>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct LinkCardProps {
    id: usize,
    rename_card: Callback<usize>,
    rm_card: Callback<usize>,
    add_link: Callback<usize>,
    edit_link: Callback<(usize, usize)>,
    rm_link: Callback<(usize, usize)>,
}

#[function_component(LinkCard)]
fn link_card(props: &LinkCardProps) -> Html {
    let id = props.id;
    let cards = use_context::<CardsContext>().unwrap();
    let card_name = &cards.inner[id].name;
    let links = &cards.inner[id].links;

    // rename card
    let rename_card = {
        let rename_card = props.rename_card.clone();
        Callback::from(move |_| rename_card.emit(id))
    };

    // remove card
    let rm_card = {
        let rm_card = props.rm_card.clone();
        Callback::from(move |_| rm_card.emit(id))
    };

    // add link to card
    let add_link = {
        let add_link = props.add_link.clone();
        Callback::from(move |_| add_link.emit(id))
    };

    // links into Html
    let links: Html = links
        .iter()
        .enumerate()
        .map(|(link_id, link)| {
            let Anchor { label, url } = link;

            let edit_link = {
                let edit_link = props.edit_link.clone();
                Callback::from(move |_| edit_link.emit((id, link_id)))
            };

            let rm_link = {
                let rm_link = props.rm_link.clone();
                Callback::from(move |_| rm_link.emit((id, link_id)))
            };

            html! {
                <div class={classes!("card-link")}>
                    <a key={format!("link{link_id}")} href={url}>{label}</a>
                    <div class={classes!("buttons")}>
                        <button onclick={edit_link}>{ "Edit link" }</button>
                        <button onclick={rm_link}>{ "Remove link" }</button>
                    </div>
                </div>
            }
        })
        .collect();

    html! {
        <div class={classes!("card")}>
            <h3 class={classes!("card-name")}>{ card_name }</h3>
            <div class={classes!("links")}>
                { links }
            </div>
            <div class={classes!("buttons")}>
                <button onclick={add_link}>{ "Add link" }</button>
                <button onclick={rename_card}>{ "Rename card" }</button>
                <button onclick={rm_card}>{ "Remove Card" }</button>
            </div>
        </div>
    }
}
