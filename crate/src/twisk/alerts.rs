use crate::{generated::css_classes::C, Msg};
use comp_state::{topo, StateAccess};
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

#[topo::nested]
pub fn pink_alert<T: Into<String>>(
    contents: T,
    access: StateAccess<bool>,
) -> Node<Msg> {
    let show_alert = access.hard_get();

    if show_alert {
        div![
            class![C.bg_pink_500 C.text_white C.px_6 C.py_4 C.border_0 C.rounded C.relative C.mb_4],
            span![
                class!(C.text_xl C.inline_block C.mr_5 C.align_middle),
                i![class!("fas" "fa-bell")]
            ],
            span![
                class![C.inline_block C.align_middle C.mr_8],
                contents.into()
            ],
            button![
                class![C.absolute C.bg_transparent C.text_2xl C.font_semibold C.leading_none C.right_0 C.top_0 C.mt_4 C.mr_6 C.outline_none C.focus__outline_none],
                on_click(move |_| access.set(false)),
                span!["x"]
            ]
        ]
    } else {
        empty![]
    }
}
