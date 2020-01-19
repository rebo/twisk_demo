// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]
#![feature(track_caller)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::enum_glob_use)]

mod generated;
mod page;
mod popper;
mod twisk;
mod use_ref;

use fixed_vec_deque::FixedVecDeque;
use generated::css_classes::C;
use seed::{prelude::*, Listener, *};
use slotmap::{DefaultKey, DenseSlotMap};
use Visibility::*;

use comp_state::topo;

const TITLE_SUFFIX: &str = "Blank Example";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

// ------ ------
// Before Mount
// ------ ------
fn before_mount(_: Url) -> BeforeMount {
    BeforeMount::new().mount_type(MountType::Takeover)
}

// ------ ------
//     Model
// ------ ------
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Visibility {
    pub fn toggle(&mut self) {
        *self = match self {
            Visible => Hidden,
            Hidden => Visible,
        }
    }
}

// We need at least 3 last values to detect scroll direction,
// because neighboring ones are sometimes equal.
type ScrollHistory = FixedVecDeque<[i32; 3]>;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub nationality: String,
    pub age: String,
    pub dob: String,
    pub shooting: u32,
    pub tackling: u32,
    pub passing: u32,
    pub speed: u32,
    pub image_url: String,
    pub key: DefaultKey,
}

pub struct Model {
    pub players: DenseSlotMap<DefaultKey, Player>,
    pub page: Page,
    pub scroll_history: ScrollHistory,
    pub menu_visibility: Visibility,
}

// ------ Page ------

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    NotFound,
}

impl Page {
    pub fn to_href(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::About => "/about",
            Self::NotFound => "/404",
        }
    }
}

impl From<Url> for Page {
    fn from(url: Url) -> Self {
        match url.path.first().map(String::as_str) {
            None | Some("") => Self::Home,
            Some("about") => Self::About,
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    seed_comp_helpers::init::<Msg, Model, _>(orders);
    orders.send_msg(Msg::UpdatePageTitle);

    let mut players = DenseSlotMap::new();
    players.insert_with_key(|k| Player {
        name: "Cristiano Ronaldo".to_string(),
        nationality: "Portuguese".to_string(),
        age: "34 years".to_string(),
        dob: "5 February 1985".to_string(),
        shooting: 92,
        tackling: 45,
        passing: 84,
        speed: 78,
        image_url: "/static/images/ronaldo.jpg".to_string(),
        key: k,
    });

    players.insert_with_key(|k| Player {
        name: "Lionel Messi".to_string(),
        nationality: "Argentine".to_string(),
        age: "32 years".to_string(),
        dob: "24 June 1987".to_string(),
        shooting: 92,
        tackling: 45,
        passing: 84,
        speed: 20,
        image_url: "/static/images/messi.jpg".to_string(),
        key: k,
    });
    players.insert_with_key(|k| Player {
        name: "Luis Suarez".to_string(),
        nationality: "Uruguayan".to_string(),
        age: "32 years".to_string(),
        dob: "24 January 1987".to_string(),
        shooting: 92,
        tackling: 45,
        passing: 84,
        speed: 78,
        image_url: "/static/images/suarez.jpg".to_string(),
        key: k,
    });
    players.insert_with_key(|k| Player {
        name: "Eden Hazard".to_string(),
        nationality: "Belgian".to_string(),
        age: "29 years".to_string(),
        dob: "7 January 1991 ".to_string(),
        shooting: 92,
        tackling: 45,
        passing: 84,
        speed: 78,
        image_url: "/static/images/eden.jpg".to_string(),
        key: k,
    });
    players.insert_with_key(|k| Player {
        name: "Mohamed Salah".to_string(),
        nationality: "Egyptian".to_string(),
        age: "27 years".to_string(),
        dob: "15 June 1992".to_string(),
        shooting: 92,
        tackling: 45,
        passing: 84,
        speed: 78,
        image_url: "/static/images/salah.jpg".to_string(),
        key: k,
    });

    let model = Model {
        players,
        page: url.into(),
        scroll_history: ScrollHistory::new(),
        menu_visibility: Hidden,
    };

    AfterMount::new(model).url_handling(UrlHandling::None)
}

// ------ ------
//    Routes
// ------ ------

pub fn routes(url: Url) -> Option<Msg> {
    // Urls which start with `static` are files => treat them as external links.
    if url.path.starts_with(&[STATIC_PATH.into()]) {
        return None;
    }
    Some(Msg::RouteChanged(url))
}

// ------ ------
// Window Events
// ------ ------

pub fn window_events(_: &Model) -> Vec<Listener<Msg>> {
    vec![raw_ev(Ev::Scroll, |_| {
        // Some browsers use `document.body.scrollTop`
        // and other ones `document.documentElement.scrollTop`.
        let mut position = body().scroll_top();
        if position == 0 {
            position = document()
                .document_element()
                .expect("cannot get document element")
                .scroll_top()
        }
        Msg::Scrolled(position)
    })]
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    RouteChanged(Url),
    UpdatePageTitle,
    ScrollToTop,
    Scrolled(i32),
    ToggleMenu,
    HideMenu,
    DoNothing,
}

impl std::default::Default for Msg {
    fn default() -> Self {
        Self::DoNothing
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(url) => {
            model.page = url.into();
            orders.send_msg(Msg::UpdatePageTitle);
        },
        Msg::UpdatePageTitle => {
            let title = match model.page {
                Page::Home => TITLE_SUFFIX.to_owned(),
                Page::About => format!("About - {}", TITLE_SUFFIX),
                Page::NotFound => format!("404 - {}", TITLE_SUFFIX),
            };
            document().set_title(&title);
        },
        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
        Msg::Scrolled(position) => {
            *model.scroll_history.push_back() = position;
        },
        Msg::ToggleMenu => model.menu_visibility.toggle(),
        Msg::HideMenu => {
            model.menu_visibility = Hidden;
        },
        Msg::DoNothing => {},
    }
}

// ------ ------
//     View
// ------ ------

// Notes:
// - \u{00A0} is the non-breaking space
//   - https://codepoints.net/U+00A0
//
// - "▶\u{fe0e}" - \u{fe0e} is the variation selector, it prevents ▶ to change to emoji in some browsers
//   - https://codepoints.net/U+FE0E

pub fn view(model: &Model) -> impl View<Msg> {
    view_component_root(model)
}

#[topo::nested]
pub fn view_component_root(model: &Model) -> Node<Msg> {
    div![
        class![
            C.fade_in => false,
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        match model.page {
            Page::Home => page::home::view(model),
            _ => page::home::view(model),
            /* Page::About => page::about::view().els(),
             * Page::NotFound => page::not_found::view().els(), */
        },
    ]
}
pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(routes)
        .window_events(window_events)
        .build_and_start();

    log!("App started.");
}
