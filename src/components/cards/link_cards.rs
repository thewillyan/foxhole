use super::cards_ctx::{Anchor, CardsContext, CardsProvider};
use crate::components::{
    cards::cards_ctx::CardEdit,
    edit::{EditForm, Input},
};
use yew::{
    classes, function_component, html, use_context, use_state_eq, AttrValue, Callback, Html,
    Properties,
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

    let mut input = Input::new(AttrValue::from("Card name:"));
    input.set_value(AttrValue::default());
    let inputs = vec![input];

    let hidden = use_state_eq(|| true);
    let hide_form = {
        let hidden = hidden.clone();
        Callback::from(move |_| hidden.set(true))
    };
    let show_form = {
        let hidden = hidden.clone();
        Callback::from(move |_| hidden.set(false))
    };

    let add_card = {
        let cards = cards.clone();
        let hide_form = hide_form.clone();
        Callback::from(move |inputs_values: Vec<String>| {
            let card_name = inputs_values.into_iter().next().unwrap_or_default();
            if !card_name.is_empty() {
                cards.dispatch(CardEdit::Add(AttrValue::from(card_name)));
                hide_form.emit(());
            }
        })
    };

    // convert cards into Html
    let cards: Html = (0..cards.inner.len())
        .map(|id| {
            html! { <LinkCard key={format!("card{id}")} {id} /> }
        })
        .collect();

    html! {
        <div class={classes!("cards")}>
            {cards}
            <button class={classes!("add-car")} onclick={show_form}>{"Add card"}</button>
            <EditForm {inputs} hidden={*hidden} save={add_card} cancel={hide_form} />
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct LinkCardProps {
    id: usize,
}

#[function_component(LinkCard)]
fn link_card(props: &LinkCardProps) -> Html {
    let cards = use_context::<CardsContext>().unwrap();
    let id = props.id;

    let mut label_input = Input::new(AttrValue::from("Link label:"));
    label_input.set_value(AttrValue::default());
    let mut url_input = Input::new(AttrValue::from("URL:"));
    url_input.set_value(AttrValue::default());
    let inputs = vec![label_input, url_input];

    let hidden = use_state_eq(|| true);
    let hide_form = {
        let hidden = hidden.clone();
        Callback::from(move |_| hidden.set(true))
    };
    let show_form = {
        let hidden = hidden.clone();
        Callback::from(move |_| hidden.set(false))
    };

    let add_link = {
        let cards = cards.clone();
        let hide_form = hide_form.clone();
        Callback::from(move |mut input_values: Vec<String>| {
            let url = input_values.pop().unwrap_or_default();
            let label = input_values.pop().unwrap_or_default();

            if !(url.is_empty() || label.is_empty()) {
                let link = Anchor {
                    label: AttrValue::from(label),
                    url: AttrValue::from(url),
                };
                cards.dispatch(CardEdit::AddLink {
                    card_index: id,
                    link,
                });
                hide_form.emit(());
            }
        })
    };
    let rm_card = {
        let cards = cards.clone();
        Callback::from(move |_| cards.dispatch(CardEdit::Remove(id)))
    };

    let card_name = &cards.inner[id].name;

    html! {
        <div class={classes!("card")}>
            <h3 class={classes!("card-name")}>{ card_name }</h3>
            <Links card_id={id}/>
            <div class={classes!("buttons")}>
                <button onclick={show_form}>{ "Add link" }</button>
                <button onclick={rm_card}>{ "Remove Card" }</button>
            </div>
            <EditForm {inputs} hidden={*hidden} save={add_link} cancel={hide_form}/>
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

    let links = &cards.inner[card_id].links;
    let links: Html = links
        .iter()
        .enumerate()
        .map(|(i, link)| {
            let Anchor { label, url } = link;

            let rm_link = {
                let cards = cards.clone();
                Callback::from(move |_| {
                    let act = CardEdit::RemoveLink {
                        card_index: card_id,
                        link_index: i,
                    };
                    cards.dispatch(act);
                })
            };

            html! {
                <div class={classes!("card-link")}>
                    <a key={format!("link{i}")} href={url}>{label}</a>
                    <button onclick={rm_link}>{ "Remove link" }</button>
                </div>
            }
        })
        .collect();

    html! {
        <div class={classes!("links")}>
            {links}
        </div>
    }
}
