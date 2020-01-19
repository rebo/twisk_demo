use crate::{generated::css_classes::C, Msg};
use comp_state::{topo, use_state};
use enclose::enclose as e;
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

#[topo::nested]
pub fn modal(contents: Node<Msg>) -> Node<Msg> {
    let (show_modal, show_modal_access) = use_state(|| false);

    div![
        button![
            class![C.bg_pink_500 C.text_white C.font_bold C.uppercase C.text_sm C.px_6 C.py_3 C.rounded C.shadow C.hover__shadow_lg C.outline_none C.focus__outline_none C.mr_1 C.mb_1],
            attrs![At::Type=>"button"],
            style![St::Transition =>"all .15s ease"],
            on_click(
                e!((show_modal_access) move |_| show_modal_access.set(true))
            ),
            "Open regular modal"
        ],
        if !show_modal {
            vec![empty![]]
        } else {
            vec![
                div![
                    class![C.justify_center C.items_center C.flex C.overflow_x_hidden C.overflow_y_auto C.fixed C.inset_0 C.z_50 C.outline_none C.focus__outline_none],
                    on_click(
                        e!((show_modal_access) move |_| show_modal_access.set(false))
                    ),
                    div![
                        class![C.relative C.w_auto C.my_6 C.mx_auto C.max_w_3xl],
                        // content
                        div![
                            class![C.border_0 C.rounded_lg C.shadow_lg C.relative C.flex C.flex_col C.w_full C.bg_white C.outline_none C.focus__outline_none],
                            // header
                            div![
                                class![C.flex C.items_start C.justify_between C.p_5 C.border_b C.border_solid C.border_gray_300 C.rounded_t],
                                h3![
                                    class![C.text_3xl C.font_semibold],
                                    "Modal Title"
                                ],
                                button![
                                    class![C.p_1 C.ml_auto C.bg_transparent C.border_0 C.text_black C.opacity_10 C.float_right C.text_3xl C.leading_none C.font_semibold C.outline_none C.focus__outline_none],
                                    on_click(
                                        e!((show_modal_access) move |_| show_modal_access.set(false))
                                    ),
                                    span![
                                        class![C.bg_transparent C.text_black C.opacity_10 C.h_6 C.w_6 C.text_2xl C.block C.outline_none C.focus__outline_none],
                                        "×"
                                    ]
                                ],
                            ],
                            div![
                                class![C.relative C.p_6 C.flex_auto],
                                p![
                                    class![C.my_4 C.text_gray_600 C.text_lg C.leading_relaxed],
                                    contents
                                ],
                            ],
                            // footer
                            div![
                                class![C.flex C.items_center C.justify_end C.p_6 C.border_t C.border_solid C.border_gray_300 C.rounded_b],
                                button![
                                    class![C.text_red_500 C.bg_transparent C.font_bold C.uppercase C.px_6 C.py_2 C.text_sm C.outline_none C.focus__outline_none C.mr_1 C.mb_1],
                                    attrs![At::Type => "button"],
                                    style![St::Transition =>"all .15s ease"],
                                    on_click(
                                        e!((show_modal_access) move |_| show_modal_access.set(false))
                                    ),
                                    "Close"
                                ],
                                button![
                                    class![C.bg_green_500 C.text_white C.font_bold C.uppercase C.text_sm C.px_6 C.py_3 C.rounded C.shadow C.hover__shadow_lg C.outline_none C.focus__outline_none C.mr_1 C.mb_1],
                                    attrs![At::Type => "button"],
                                    style![St::Transition =>"all .15s ease"],
                                    on_click(
                                        e!((show_modal_access) move |_| show_modal_access.set(false))
                                    ),
                                    "Save Changes"
                                ]
                            ]
                        ]
                    ]
                ],
                div![class![C.opacity_20 C.fixed C.inset_0 C.z_40 C.bg_black]],
            ]
        }
    ]
}
