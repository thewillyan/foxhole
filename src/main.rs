use std::str::FromStr;

use gloo_storage::{LocalStorage, Storage};
use yew::{classes, function_component, html, use_state_eq, Callback, Html};

mod components;

use components::{cards::LinkCards, Bar, Greeting};

#[derive(Clone, PartialEq)]
pub enum Theme {
    Dark,
    White,
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

#[function_component(App)]
fn app() -> Html {
    let theme = use_state_eq(|| match LocalStorage::get::<String>("theme") {
        Ok(val) => Theme::from_str(val.as_str()).unwrap(),
        Err(_) => {
            let default = Theme::default();
            LocalStorage::set("theme", default.to_string()).unwrap();
            default
        }
    });

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = match *theme {
                Theme::Dark => Theme::White,
                Theme::White => Theme::Dark,
            };
            LocalStorage::set("theme", new_theme.to_string()).unwrap();
            theme.set(new_theme);
        })
    };

    html! {
        <div id="app" class={classes!((*theme).to_string())}>
            <Bar {toggle_theme}/>
            <header>
                <h1>{ "Foxhole" }</h1>
                <Greeting/>
            </header>
            <LinkCards/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
