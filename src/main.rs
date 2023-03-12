use std::{rc::Rc, str::FromStr};

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::{
    classes, function_component, html, use_reducer, ContextProvider, Html, Reducible,
    UseReducerHandle,
};

mod components;

use components::{cards::LinkCards, Bar, Greeting};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    White,
}

impl Theme {
    pub fn toggle(&mut self) {
        match self {
            Self::Dark => *self = Self::White,
            Self::White => *self = Self::Dark,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dark" => Ok(Self::Dark),
            "white" => Ok(Self::White),
            _ => Err(format!("Invalid theme '{s}'")),
        }
    }
}

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            Self::Dark => "dark".to_owned(),
            Self::White => "white".to_owned(),
        }
    }
}

pub enum CtxAction {
    ToggleTheme,
    ToggleEdit,
}

#[derive(Clone, PartialEq)]
pub struct GlobalData {
    pub theme: Theme,
    pub editable: bool,
}

impl Reducible for GlobalData {
    type Action = CtxAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut data = (*self).clone();
        match action {
            CtxAction::ToggleTheme => {
                data.theme.toggle();
                LocalStorage::set("theme", data.theme.to_string()).unwrap();
            },
            CtxAction::ToggleEdit => data.editable = !data.editable,
        }
        Rc::new(data)
    }
}

pub type GlobalCtx = UseReducerHandle<GlobalData>;

#[function_component(App)]
fn app() -> Html {
    let theme = match LocalStorage::get::<String>("theme") {
        Ok(val) => Theme::from_str(val.as_str()).unwrap(),
        Err(_) => {
            let default = Theme::default();
            LocalStorage::set("theme", default.to_string()).unwrap();
            default
        }
    };

    let global_ctx = use_reducer(|| GlobalData {
        theme,
        editable: false,
    });
    let app_theme = global_ctx.theme.to_string();

    html! {
        <ContextProvider<GlobalCtx> context={global_ctx}>
            <div id="app" class={classes!(app_theme)}>
                <Bar />
                <header>
                    <h1>{ "Foxhole" }</h1>
                    <Greeting/>
                </header>
                <LinkCards/>
            </div>
        </ContextProvider<GlobalCtx>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
