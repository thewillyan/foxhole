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

#[allow(dead_code)]
impl Card {
    /// Constructs a card with no links.
    pub fn new(name: String) -> Self {
        Self {
            name,
            links: Vec::new(),
        }
    }

    /// Constructs a card with links from a vector of arrays of the form `[label, url]`.
    pub fn from(name: String, links: Vec<[String; 2]>) -> Self {
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

    /// Removes the nth link.
    pub fn remove_link(&mut self, nth: usize) -> Anchor {
        self.links.remove(nth)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum CardsHandler {
    // cards
    Add(String),
    Remove(usize),
    Rename {
        card_index: usize,
        new_name: String,
    },
    // card links
    AddLink {
        card_index: usize,
        link: Anchor,
    },
    RemoveLink {
        card_index: usize,
        link_index: usize,
    },
    EditLink {
        card_index: usize,
        link_index: usize,
        new_label: Option<String>,
        new_url: Option<String>,
    },
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

impl Reducible for Cards {
    type Action = CardsHandler;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut inner = (*self).clone().into_inner();
        match action {
            Self::Action::Add(name) => {
                inner.push(Card::new(name));
            }
            Self::Action::Remove(index) => {
                inner.remove(index);
            }
            Self::Action::Rename {
                card_index,
                new_name,
            } => inner.get_mut(card_index).unwrap().name = new_name,
            Self::Action::AddLink { card_index, link } => {
                inner.get_mut(card_index).unwrap().push_link(link);
            }
            Self::Action::RemoveLink {
                card_index,
                link_index,
            } => {
                inner.get_mut(card_index).unwrap().remove_link(link_index);
            }
            Self::Action::EditLink {
                card_index,
                link_index,
                new_label,
                new_url,
            } => {
                let link = inner
                    .get_mut(card_index)
                    .unwrap()
                    .links
                    .get_mut(link_index)
                    .unwrap();
                if let Some(label) = new_label {
                    link.label = label;
                }
                if let Some(url) = new_url {
                    link.url = url;
                }
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
