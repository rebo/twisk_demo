use crate::{generated::css_classes::C, Msg};
use seed::{prelude::*, *};

pub fn _simple_image<T: Into<String>>(src: T, alt: T) -> Node<Msg> {
    div![
        class![C.flex C.flex_wrap C.justify_center],
        div![
            class![C.w_6of12 C.sm__w_4of12 C.px_4],
            img![
                attrs![At::Src => src.into(), At::Alt=>alt.into()],
                class![C.shadow C.rounded C.max_w_full C.h_auto C.align_middle C.border_none]
            ]
        ]
    ]
}

pub fn _circle_image<T: Into<String>>(src: T, alt: T) -> Node<Msg> {
    div![
        class![C.flex C.flex_wrap C.justify_center],
        div![
            class![C.w_1of2 C.sm__w_1of2 C.px_4],
            img![
                attrs![At::Src => src.into(), At::Alt=>alt.into()],
                class![C.shadow C.rounded_full C.max_w_full C.h_auto C.align_middle C.border_none]
            ]
        ]
    ]
}

pub fn simple_raised_image<T: Into<String>>(src: T, alt: T) -> Node<Msg> {
    div![
        class![C.flex C.flex_wrap C.justify_center],
        div![
            class![C.w_6of12 C.sm__w_4of12 C.px_4],
            img![
                attrs![At::Src => src.into(), At::Alt=>alt.into()],
                class![C.shadow_lg C.rounded C.max_w_full C.h_auto C.align_middle C.border_none]
            ]
        ]
    ]
}

pub fn logo_raised_image<T: Into<String>>(src: T, alt: T) -> Node<Msg> {
    div![
        class![C.flex C.flex_wrap C.items_center C.justify_center],
        div![
            // class![C.w_1of2 C.sm__w_1of2],
            img![
                attrs![At::Src => src.into(), At::Alt=>alt.into()],
                class![C.object_cover C.h_18 C.p_1 C.shadow_lg C.rounded_full  C.max_w_full C.align_middle C.border_none]
            ]
        ]
    ]
}
