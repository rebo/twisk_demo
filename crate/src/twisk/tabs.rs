use crate::{generated::css_classes::C, Msg};
use comp_state::{topo, use_state};
use enclose::enclose as e;
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

#[topo::nested]
pub fn tabs_3<T: Into<String>>(
    first_tab: (T, Node<Msg>),
    second_tab: (T, Node<Msg>),
    third_tab: (T, Node<Msg>),
) -> Node<Msg> {
    let (open_tab, open_tab_access) = use_state(|| 1);
    let (first_tab_title, first_tab_content) = first_tab;
    let (second_tab_title, second_tab_content) = second_tab;
    let (third_tab_title, third_tab_content) = third_tab;
    div![
        class![C.flex C.flex_wrap],
        div![
            class![C.w_full],
            ul![
                class![C.flex C.mb_0 C.list_none C.flex_wrap C.pt_3 C.pb_4 C.flex_row],
                li![
                    class![C._mb_px C.mr_2 C.last__mr_0 C.flex_auto C.text_center],
                    a![
                        class![C.text_xs C.font_bold C.uppercase C.px_5 C.py_3 C.shadow_lg C.rounded C.block C.leading_normal],
                        if open_tab == 1 {
                            class![C.text_white C.bg_pink_600]
                        } else {
                            class![C.text_pink_600 C.bg_white]
                        },
                        on_click(
                            e!((open_tab_access) move |_| open_tab_access.set(1))
                        ),
                        first_tab_title.into(),
                    ]
                ],
                li![
                    class![C._mb_px C.mr_2 C.last__mr_0 C.flex_auto C.text_center],
                    a![
                        class![C.text_xs C.font_bold C.uppercase C.px_5 C.py_3 C.shadow_lg C.rounded C.block C.leading_normal],
                        if open_tab == 2 {
                            class![C.text_white C.bg_pink_600]
                        } else {
                            class![C.text_pink_600 C.bg_white]
                        },
                        on_click(
                            e!((open_tab_access) move |_| open_tab_access.set(2))
                        ),
                        second_tab_title.into(),
                    ]
                ],
                li![
                    class![C._mb_px C.mr_2 C.last__mr_0 C.flex_auto C.text_center],
                    a![
                        class![C.text_xs C.font_bold C.uppercase C.px_5 C.py_3 C.shadow_lg C.rounded C.block C.leading_normal],
                        if open_tab == 3 {
                            class![C.text_white C.bg_pink_600]
                        } else {
                            class![C.text_pink_600 C.bg_white]
                        },
                        on_click(
                            e!((open_tab_access) move |_| open_tab_access.set(3))
                        ),
                        third_tab_title.into(),
                    ]
                ]
            ],
            div![
                class![C.relative C.flex C.flex_col C.min_w_0 C.break_words C.bg_white C.w_full C.mb_6 C.shadow_lg C.rounded],
                div![
                    class![C.px_4 C.py_5 C.flex_auto],
                    div![
                        class!["tab_content" "tab_space"],
                        div![
                            if open_tab == 1 {
                                class![C.block]
                            } else {
                                class![C.hidden]
                            },
                            id!("link1"),
                            first_tab_content,
                        ],
                        div![
                            if open_tab == 2 {
                                class![C.block]
                            } else {
                                class![C.hidden]
                            },
                            id!("link1"),
                            second_tab_content,
                        ],
                        div![
                            if open_tab == 3 {
                                class![C.block]
                            } else {
                                class![C.hidden]
                            },
                            id!("link1"),
                            third_tab_content,
                        ]
                    ]
                ]
            ]
        ]
    ]
}
