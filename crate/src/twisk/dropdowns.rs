use crate::{
    generated::css_classes::C,
    popper,
    popper::Popper,
    use_ref::{use_ref, LazyHtmlElementStateAccessTrait},
    Msg,
};
use comp_state::{topo, use_state};
use enclose::enclose as e;
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

#[topo::nested]
pub fn dropdown<T: Into<String>>(
    main_contents: T,
    dropdown_contents: &[(&str, &str)],
) -> Node<Msg> {
    let (dropdown_show, dropdown_show_access) = use_state(|| false);

    let btn_ref = use_ref();
    let popover_ref = use_ref();

    let btn_classname = btn_ref.class_name();
    let popover_classname = popover_ref.class_name();
    div![
        class![C.flex, C.flex_wrap],
        div![
            class![C.w_full C.sm__w_6of12 C.md__w_4of12],
            div![
                class![C.relative C.inline_flex C.align_middle C.w_full],
                button![
                    class![C.text_white C.font_bold C.uppercase C.text_sm C.px_6 C.py_3 C.rounded C.shadow C.hover__shadow_lg C.outline_none C.focus__outline_none C.mr_1 C.mb_1 C.bg_gray_800 btn_classname.as_ref()],
                    style![St::MinWidth => "12rem"],
                    style![St::Transition => "all .15s ease"],
                    attrs![At::Type => "button"],
                    on_click(move |_| {
                        if dropdown_show {
                            if let Some(_element) = btn_ref.current() {
                                dropdown_show_access.set(false);
                            }
                        } else if let (Some(btn), Some(popover)) =
                                (btn_ref.current(), popover_ref.current())
                            {
                                let example = popper::Options {
                                    placement: "bottom-start".to_string(),
                                };
                                let value =
                                    JsValue::from_serde(&example).unwrap();
                                Popper::new(btn, popover, value);
                                dropdown_show_access.set(true);
                            }
                        }),
                        main_contents.into()
                    ],
                div![
                    if dropdown_show {
                        class![C.block]
                    } else {
                        class![C.hidden]
                    },
                    class![C.bg_gray_800]
                    ,
                    class![C.text_base C.z_50 C.float_left C.py_2 C.list_none C.text_left C.rounded C.shadow_lg C.mt_1],
                    class![popover_classname.as_ref()],
                    style![St::MinWidth => "12rem"],
                    dropdown_contents.iter().map( |(text,link)|
                    {a![
                        attrs![At::Href=>link],
                        class![ C.text_sm C.py_2 C.px_4 C.font_normal C.block C.w_full C.whitespace_no_wrap C.bg_transparent C.text_white],
                        text
                    ]
                    }
                ).collect::<Vec<Node<Msg>>>()
                ]
            ]
        ]
    ]
}

#[topo::nested]
pub fn small_dropdown<T: Into<String>>(
    main_contents: T,
    dropdown_contents: &[(&str, &str)],
) -> Node<Msg> {
    let (dropdown_show, dropdown_show_access) = use_state(|| false);

    let btn_ref = use_ref();
    let popover_ref = use_ref();

    let btn_classname = btn_ref.class_name();
    let popover_classname = popover_ref.class_name();
    div![
        class![C.flex, C.flex_wrap],
        div![
            class![C.w_full C.sm__w_6of12 C.md__w_4of12],
            div![
                class![C.relative C.inline_flex C.align_middle C.w_full],
                button![
                    class![C.text_white C.font_bold C.uppercase C.text_xs C.px_1 C.py_1 C.rounded C.shadow C.hover__shadow_lg C.outline_none C.focus__outline_none C.mr_1 C.mb_1 C.bg_gray_800 btn_classname.as_ref()],
                    style![St::MinWidth => "12rem"],
                    style![St::Transition => "all .15s ease"],
                    attrs![At::Type => "button"],
                    on_click(move |_| {
                        if dropdown_show {
                            if let Some(_element) = btn_ref.current() {
                                dropdown_show_access.set(false);
                            }
                        } else if let (Some(btn), Some(popover)) =
                                (btn_ref.current(), popover_ref.current())
                            {
                                let example = popper::Options {
                                    placement: "bottom-start".to_string(),
                                };
                                let value =
                                    JsValue::from_serde(&example).unwrap();
                                Popper::new(btn, popover, value);
                                dropdown_show_access.set(true);
                            }
                        }),
                        main_contents.into()
                    ],
                div![
                    if dropdown_show {
                        class![C.block]
                    } else {
                        class![C.hidden]
                    },
                    class![C.bg_gray_800]
                    ,
                    class![C.text_base C.z_50 C.float_left C.py_2 C.list_none C.text_left C.rounded C.shadow_lg C.mt_1],
                    class![popover_classname.as_ref()],
                    style![St::MinWidth => "12rem"],
                    dropdown_contents.iter().map( |(text,link)|
                    {a![
                        attrs![At::Href=>link],
                        class![ C.text_sm C.py_2 C.px_4 C.font_normal C.block C.w_full C.whitespace_no_wrap C.bg_transparent C.text_white],
                        text
                    ]
                    }
                ).collect::<Vec<Node<Msg>>>()
                ]
            ]
        ]
    ]
}
#[topo::nested]
pub fn small_action_dropdown<T: Into<String>, F: Fn(&str) + 'static>(
    main_contents: T,
    options_list: &'static [&str],
    callback: F,
) -> Node<Msg> {
    let rc_refcell_callback =
        std::rc::Rc::new(std::cell::RefCell::new(callback));
    let (dropdown_show, dropdown_show_access) = use_state(|| false);

    let btn_ref = use_ref();
    let popover_ref = use_ref();

    let btn_classname = btn_ref.class_name();
    let popover_classname = popover_ref.class_name();
    div![
        class![C.flex, C.flex_wrap],
        div![
            class![C.w_full C.sm__w_6of12 C.md__w_4of12],
            div![
                class![C.relative C.inline_flex C.align_middle C.w_full],
                button![
                    class![C.text_white C.font_bold C.uppercase C.text_xs C.px_1 C.py_1 C.rounded C.shadow C.hover__shadow_lg C.outline_none C.focus__outline_none C.mr_1 C.mb_1 C.bg_gray_800 btn_classname.as_ref()],
                    style![St::MinWidth => "12rem"],
                    style![St::Transition => "all .15s ease"],
                    attrs![At::Type => "button"],
                    on_click(move |_| {
                        if dropdown_show {
                            if let Some(_element) = btn_ref.current() {
                                dropdown_show_access.set(false);
                            }
                        } else if let (Some(btn), Some(popover)) =
                                (btn_ref.current(), popover_ref.current())
                            {
                                let example = popper::Options {
                                    placement: "bottom-start".to_string(),
                                };
                                let value =
                                    JsValue::from_serde(&example).unwrap();
                                Popper::new(btn, popover, value);
                                dropdown_show_access.set(true);
                            }
                        }),
                        main_contents.into()
                    ],
                div![
                    if dropdown_show {
                        class![C.block]
                    } else {
                        class![C.hidden]
                    },
                    class![C.bg_gray_800]
                    ,
                    class![C.text_base C.z_50 C.float_left C.py_2 C.list_none C.text_left C.rounded C.shadow_lg C.mt_1],
                    class![popover_classname.as_ref()],
                    style![St::MinWidth => "12rem"],
                    options_list.iter().map( |text|
                    {button![
                        class![ C.text_sm C.py_2 C.px_4 C.font_normal C.block C.w_full C.whitespace_no_wrap C.bg_transparent C.text_white],
                        text,
                        on_click( e!( (rc_refcell_callback) move |_| rc_refcell_callback.borrow()(text) )),
                    ]
                    }
                ).collect::<Vec<Node<Msg>>>()
                ]
            ]
        ]
    ]
}
