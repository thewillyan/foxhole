use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, AttrValue, Children, ContextProvider, Html, Properties,
    Reducible, UseReducerHandle,
};

#[derive(Clone, PartialEq)]
pub struct Anchor {
    pub label: AttrValue,
    pub url: AttrValue,
}

#[derive(Clone, PartialEq)]
pub struct Card {
    pub name: AttrValue,
    pub links: Vec<Anchor>,
}

#[allow(dead_code)]
impl Card {
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

    /// Removes the nth link.
    pub fn remove_link(&mut self, nth: usize) -> Anchor {
        self.links.remove(nth)
    }
}

#[allow(dead_code)]
pub enum CardEdit {
    Add(AttrValue),
    Remove(usize),
    AddLink {
        card_index: usize,
        link: Anchor,
    },
    RemoveLink {
        card_index: usize,
        link_index: usize,
    },
}

#[derive(Clone, PartialEq)]
pub struct Cards {
    pub inner: Vec<Card>,
}

impl Cards {
    pub fn new() -> Self {
        Cards { inner: Vec::new() }
    }

    pub fn into_inner(self) -> Vec<Card> {
        self.inner
    }
}

impl Reducible for Cards {
    type Action = CardEdit;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut inner = (*self).clone().into_inner();
        match action {
            CardEdit::Add(name) => {
                if name.is_empty() {
                    return self;
                }
                let card = Card::new(name);
                inner.push(card);
            }
            CardEdit::Remove(index) => {
                inner.remove(index);
            }
            CardEdit::AddLink { card_index, link } => {
                inner.get_mut(card_index).unwrap().push_link(link);
            }
            CardEdit::RemoveLink {
                card_index,
                link_index,
            } => {
                inner.get_mut(card_index).unwrap().remove_link(link_index);
            }
        }
        Rc::new(Cards { inner })
    }
}

pub type CardsContext = UseReducerHandle<Cards>;

#[derive(Clone, PartialEq, Properties)]
pub struct CardsProviderProps {
    pub children: Children,
}

#[function_component(CardsProvider)]
pub fn cards_provider(CardsProviderProps { children }: &CardsProviderProps) -> Html {
    let cards = use_reducer(Cards::new);

    html! {
        <ContextProvider<CardsContext> context={cards}>
            { children.iter().collect::<Html>() }
        </ContextProvider<CardsContext>>
    }
}
