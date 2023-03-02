use crate::components::{
    cards::cards_ctx::{Anchor, CardsContext, CardsHandler, CardsProvider},
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

type CardId = usize;
type LinkId = usize;

#[derive(Clone, PartialEq)]
enum CardFormAct {
    Add,
    Rename(CardId),
}

#[derive(Clone, PartialEq)]
enum LinkFormAct {
    Add(CardId),
    Edit { card: CardId, link: LinkId },
}

#[function_component(CardList)]
fn card_list() -> Html {
    let cards = use_context::<CardsContext>().unwrap();

    // card form
    let card_form_action = use_state_eq(|| None);
    let card_form_hide = use_state_eq(|| true);

    let card_name = match *card_form_action {
        Some(CardFormAct::Rename(index)) => cards.inner[index].name.clone(),
        _ => AttrValue::default(),
    };
    let input = Input::new(AttrValue::from("New card name:")).value(card_name);
    let card_inputs = vec![input];

    let change_card = {
        let cards = cards.clone();
        let hidden = card_form_hide.clone();
        use_callback(
            move |inputs_values: Option<Vec<String>>, form_action| {
                if let (Some(values), Some(action)) = (inputs_values, (**form_action).clone()) {
                    // don't allow empty name
                    let name = match values.into_iter().next() {
                        Some(val) if !val.is_empty() => AttrValue::from(val),
                        _ => return,
                    };

                    let op = match action {
                        CardFormAct::Add => CardsHandler::Add(name),
                        CardFormAct::Rename(id) => CardsHandler::Rename {
                            card_index: id,
                            new_name: name,
                        },
                    };
                    cards.dispatch(op);
                }
                form_action.set(None);
                hidden.set(true);
            },
            card_form_action.clone(),
        )
    };

    // link form
    let link_form_action = use_state_eq(|| None);
    let link_form_hide = use_state_eq(|| true);

    let (link_label, link_url) = match *link_form_action {
        Some(LinkFormAct::Edit { card, link }) => {
            let link = &cards.inner[card].links[link];
            (link.label.clone(), link.url.clone())
        }
        _ => (AttrValue::default(), AttrValue::default()),
    };
    let label_input = Input::new(AttrValue::from("Label:")).value(link_label);
    let url_input = Input::new(AttrValue::from("URL:")).value(link_url);
    let link_inputs = vec![label_input, url_input];

    let change_link = {
        let cards = cards.clone();
        let hidden = link_form_hide.clone();
        use_callback(
            move |input_values: Option<Vec<String>>, form_action| {
                if let (Some(mut values), Some(action)) = (input_values, (**form_action).clone()) {
                    let url = AttrValue::from(values.pop().unwrap_or_default());
                    let label = AttrValue::from(values.pop().unwrap_or_default());

                    if url.is_empty() && label.is_empty() {
                        return;
                    }

                    let op = match action {
                        LinkFormAct::Add(index) => CardsHandler::AddLink {
                            card_index: index,
                            link: Anchor { label, url },
                        },
                        LinkFormAct::Edit { card, link } => CardsHandler::EditLink {
                            card_index: card,
                            link_index: link,
                            new_label: if label.is_empty() { None } else { Some(label) },
                            new_url: if url.is_empty() { None } else { Some(url) },
                        },
                    };
                    cards.dispatch(op);
                }
                form_action.set(None);
                hidden.set(true);
            },
            link_form_action.clone(),
        )
    };

    // card form callbacks
    let add_card_form = {
        let hidden = card_form_hide.clone();
        let form_action = card_form_action.clone();
        Callback::from(move |_| {
            form_action.set(Some(CardFormAct::Add));
            hidden.set(false);
        })
    };

    let rename_card = {
        let form_action = card_form_action;
        let hidden = card_form_hide.clone();
        Callback::from(move |card_id| {
            form_action.set(Some(CardFormAct::Rename(card_id)));
            hidden.set(false);
        })
    };

    // link form callbacks
    let add_link = {
        let form_action = link_form_action.clone();
        let hidden = link_form_hide.clone();
        Callback::from(move |card_id| {
            form_action.set(Some(LinkFormAct::Add(card_id)));
            hidden.set(false);
        })
    };

    let edit_link = {
        let form_action = link_form_action;
        let hidden = link_form_hide.clone();
        Callback::from(move |(card, link)| {
            form_action.set(Some(LinkFormAct::Edit { card, link }));
            hidden.set(false);
        })
    };

    // convert cards into Html
    let cards: Html = (0..cards.inner.len())
        .map(|id| {
            html! {
                <LinkCard key={format!("card{id}")} {id} rename_card={rename_card.clone()}
                    add_link={add_link.clone()} edit_link={edit_link.clone()}
                />
            }
        })
        .collect();

    html! {
        <div class={classes!("cards")}>
            {cards}
            <div class={classes!("buttons")}>
                <button class={classes!("add-card")} onclick={add_card_form}>{"Add card"}</button>
            </div>
            <div class={classes!("forms")}>
                <EditForm inputs={card_inputs} hidden={*card_form_hide} save={change_card}/>
                <EditForm inputs={link_inputs} hidden={*link_form_hide} save={change_link}/>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct LinkCardProps {
    id: usize,
    rename_card: Callback<usize>,
    add_link: Callback<usize>,
    edit_link: Callback<(usize, usize)>,
}

#[function_component(LinkCard)]
fn link_card(props: &LinkCardProps) -> Html {
    let id = props.id;
    let cards = use_context::<CardsContext>().unwrap();
    let card_name = &cards.inner[id].name;
    let links = &cards.inner[id].links;

    // callbacks
    let rm_card = {
        let cards = cards.clone();
        Callback::from(move |_| cards.dispatch(CardsHandler::Remove(id)))
    };

    let rename_card = {
        let rename_card = props.rename_card.clone();
        Callback::from(move |_| rename_card.emit(id))
    };

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

            // link callbacks
            let edit_link = {
                let edit_link = props.edit_link.clone();
                Callback::from(move |_| edit_link.emit((id, link_id)))
            };

            let rm_link = {
                let cards = cards.clone();
                Callback::from(move |_| {
                    let action = CardsHandler::RemoveLink {
                        card_index: id,
                        link_index: link_id,
                    };
                    cards.dispatch(action);
                })
            };

            // link to html
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
