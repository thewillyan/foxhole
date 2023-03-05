use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, Children, ContextProvider, Html, Properties, Reducible,
    UseReducerHandle,
};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Anchor {
    pub label: String,
    pub url: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub links: Vec<Anchor>,
}

impl Card {
    /// Constructs a card with no links.
    pub fn new(name: String) -> Self {
        Self {
            name,
            links: Vec::new(),
        }
    }

    /// Append a link to the end of the card.
    pub fn push_link(&mut self, anchor: Anchor) {
        self.links.push(anchor)
    }

    /// Removes the nth link.
    pub fn remove_link(&mut self, nth: usize) -> Anchor {
        self.links.remove(nth)
    }
}

pub type CardId = usize;
pub type LinkPos = usize;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkId {
    pub card: CardId,
    pub link: LinkPos,
}

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Cards {
    pub inner: Vec<Card>,
}

impl Cards {
    pub fn into_inner(self) -> Vec<Card> {
        self.inner
    }
}

#[derive(Clone, PartialEq)]
pub enum CardsHandler {
    // cards actions
    Add(String),
    Remove(CardId),
    Rename {
        card: CardId,
        new_name: String,
    },
    Swap {
        card1: CardId,
        card2: CardId,
    },
    AddLink {
        card_index: CardId,
        link: Anchor,
    },
    // link actions
    RemoveLink(LinkId),
    EditLink {
        link: LinkId,
        new_label: Option<String>,
        new_url: Option<String>,
    },
    SwapLinks {
        card: CardId,
        link1: LinkPos,
        link2: LinkPos,
    },
}

impl Reducible for Cards {
    type Action = CardsHandler;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut inner = (*self).clone().into_inner();
        match action {
            // card actions
            CardsHandler::Add(name) => {
                inner.push(Card::new(name));
            }
            CardsHandler::Remove(index) => {
                inner.remove(index);
            }
            CardsHandler::Rename { card, new_name } => inner.get_mut(card).unwrap().name = new_name,
            CardsHandler::Swap { card1, card2 } => inner.swap(card1, card2),
            // link actions
            CardsHandler::AddLink { card_index, link } => {
                inner.get_mut(card_index).unwrap().push_link(link);
            }
            CardsHandler::RemoveLink(LinkId { card, link }) => {
                inner.get_mut(card).unwrap().remove_link(link);
            }
            CardsHandler::EditLink {
                link: LinkId { card, link },
                new_label,
                new_url,
            } => {
                let link = inner.get_mut(card).unwrap().links.get_mut(link).unwrap();
                if let Some(label) = new_label {
                    link.label = label;
                }
                if let Some(url) = new_url {
                    link.url = url;
                }
            }
            CardsHandler::SwapLinks { card, link1, link2 } => {
                inner.get_mut(card).unwrap().links.swap(link1, link2);
            }
        }
        let cards = Rc::new(Cards { inner });
        LocalStorage::set("cards", Rc::clone(&cards).as_ref()).unwrap();
        cards
    }
}

pub type CardsContext = UseReducerHandle<Cards>;

#[derive(Clone, PartialEq, Properties)]
pub struct CardsProviderProps {
    pub children: Children,
}

#[function_component(CardsProvider)]
pub fn cards_provider(CardsProviderProps { children }: &CardsProviderProps) -> Html {
    let cards = use_reducer(|| LocalStorage::get::<Cards>("cards").unwrap_or_default());

    html! {
        <ContextProvider<CardsContext> context={cards}>
            { children.iter().collect::<Html>() }
        </ContextProvider<CardsContext>>
    }
}
