use crate::{generated::css_classes::C, Msg};
use comp_state::{topo, use_state};
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;
#[topo::nested]
pub fn pink_menu<T: Into<String>>(
    header: T,
    menu_items: &[(&str, &str)],
) -> Node<Msg> {
    let (menu_open, menu_open_access) = use_state(|| false);

    div![
        class![C.flex, C.flex_wrap C.py_2],
        div![
            class![C.w_full C.px_4],
            div![
                class![C.relative C.flex C.flex_wrap C.items_center C.justify_between C.px_2 C.py_3 "navbar-expand-lg" C.bg_pink_500 C.rounded],
                div![
                    class![C.container C.px_4 C.mx_auto C.flex C.flex_wrap C.items_center C.justify_between],
                    div![
                        class![C.w_full C.relative C.flex C.justify_between C.lg__w_auto C.px_4 C.lg__static C.lg__block C.lg__justify_start],
                        a![
                            class![C.text_sm C.font_bold C.leading_relaxed C.inline_block C.mr_4 C.py_2 C.whitespace_no_wrap C.uppercase C.text_white],
                            attrs![At::Href => "#pablo"],
                            header.into()
                        ],
                        button![
                            class![C.text_white C.cursor_pointer C.text_xl C.leading_none C.px_3 C.py_2 C.border C.border_solid C.border_transparent C.rounded C.bg_transparent C.block C.lg__hidden C.outline_none C.focus__outline_none],
                            attrs![At::Type=> "button"],
                            on_click(move |_| menu_open_access.set(!menu_open)),
                            i![ class!["fas fa-bars"]]
                        ],
                    ],
                    div![
                        class![C.lg__flex C.flex_grow C.items_center],
                        if menu_open {
                            class![C.flex]
                        } else {
                            class![C.hidden]
                        },
                        id!("example-navbar-info"),
                        ul![
                            class![C.flex C.flex_col C.lg__flex_row C.list_none ],
                                menu_items.iter().map( |(text,link)|
                                {
                                    li![class!["nav-item"],a![
                                    attrs![At::Href=>link],
                                    class![C.px_3 C.py_2 C.flex C.items_center C.text_xs C.uppercase C.font_bold C.leading_snug C.text_white C.hover__opacity_80],
                                    text
                                    ]
                                ]
                            }
                            ).collect::<Vec<Node<Msg>>>()
                        ]
                    ]
            ]
        ]
    ]
    ]
}
