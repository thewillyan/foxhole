use yew::{
    classes, function_component, html, use_state, use_state_eq, AttrValue, Callback, Html,
    Properties,
};

use super::edit::{EditForm, Input};

#[derive(Clone, PartialEq)]
pub struct Anchor {
    pub label: AttrValue,
    pub url: AttrValue,
}

#[derive(Clone, PartialEq)]
pub struct CardData {
    name: AttrValue,
    links: Vec<Anchor>,
}

#[allow(dead_code)]
impl CardData {
    /// Constructs a card with no links.
    pub fn new(name: AttrValue) -> Self {
        Self {
            name,
            links: Vec::new(),
        }
    }

    /// Constructs a card with links from a vector of arrays of the form `[label, url]`.
    pub fn from(name: AttrValue, links: Vec<[AttrValue; 2]>) -> Self {
        let links = links
            .into_iter()
            .map(|[label, url]| Anchor { label, url })
            .collect::<Vec<_>>();

        Self { name, links }
    }

    /// Append a link to the end of the card.
    pub fn push_link(&mut self, anchor: Anchor) {
        self.links.push(anchor)
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    pub data: CardData,
    pub add_link: Callback<Anchor>,
}

#[function_component(Card)]
pub fn card(CardProps { data, add_link }: &CardProps) -> Html {
    let CardData { name, links } = data;

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

    let save_link = {
        let add_link = add_link.clone();
        let hide_form = hide_form.clone();
        Callback::from(move |mut input_values: Vec<String>| {
            let url = input_values.pop().unwrap_or_default();
            let label = input_values.pop().unwrap_or_default();

            if !(url.is_empty() || label.is_empty()) {
                let link = Anchor {
                    label: AttrValue::from(label),
                    url: AttrValue::from(url),
                };
                add_link.emit(link);
                hide_form.emit(());
            }
        })
    };

    let links: Html = links
        .iter()
        .enumerate()
        .map(|(i, link)| {
            let Anchor { label, url } = link;
            html! {
                <a class={classes!("card-link")} key={format!("link{i}")} href={url}>{label}</a>
            }
        })
        .collect();

    html! {
    <div class={classes!("card")}>
        <h3 class={classes!("card-name")}>{ name }</h3>
            { links }
            <button onclick={show_form}>{ "Add link" }</button>
            <EditForm {inputs} hidden={*hidden} save={save_link} cancel={hide_form}/>
        </div>
    }
}

#[function_component(Cards)]
pub fn cards() -> Html {
    let cards = use_state(Vec::new);
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
            if card_name.is_empty() {
                return;
            }

            let card = CardData::new(AttrValue::from(card_name));
            let new_cards = vec![(*cards).clone(), vec![card]]
                .into_iter()
                .flatten()
                .collect();
            cards.set(new_cards);
            hide_form.emit(());
        })
    };

    let add_link = {
        let cards = cards.clone();
        Callback::from(move |args: (usize, Anchor)| {
            let (card_index, link) = args;
            let mut new_cards = (*cards).clone();
            new_cards.get_mut(card_index).unwrap().push_link(link);
            cards.set(new_cards);
        })
    };

    // convert cards into Html
    let cards: Html = (*cards)
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let add_link = {
                let add_link = add_link.clone();
                Callback::from(move |link: Anchor| add_link.emit((i, link)))
            };
            html! { <Card key={format!("card{i}")} data={(*card).clone()} {add_link}/> }
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
