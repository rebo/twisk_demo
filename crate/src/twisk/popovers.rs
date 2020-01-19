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
// use serde::{Deserialize, Serialize};

#[topo::nested]
pub fn popover() -> Node<Msg> {
    let (popover_show, popover_show_access) = use_state(|| false);
    let btn_ref = use_ref();
    let popover_ref = use_ref();

    let open_popover = e!( ( btn_ref, popover_ref,popover_show_access) move |_| {
    if let (Some(btn_ref),Some(popover_ref) ) = (btn_ref.current(), popover_ref.current()){
      let options =   popper::Options {
        placement: "left".to_string(),
     };
     let options = JsValue::from_serde(&options).unwrap();
       let _ = popover_ref.class_list().remove(&js_sys::Array::of1(&JsValue::from_str("hidden")));
       Popper::new(btn_ref, popover_ref, options);
              popover_show_access.set(true);
             }
    });

    let close_popover = e!( (popover_show_access)  move |_| {


      popover_show_access.set(false);
    });
    let btn_ref_classname = btn_ref.class_name();
    let popover_ref_classname = popover_ref.class_name();
    div![
        class![C.flex C.flex_wrap],
        div![
            class![C.w_full C.text_center],
            button![
                class![btn_ref_classname.as_ref()],
                class![C.bg_pink_500 C.text_white C.hover__bg_pink_600 C.font_bold C.uppercase C.text_sm C.px_6 C.py_3 C.rounded C.shadow C.hover__shadow_lg C.outline_none C.focus__outline_none C.mr_1 C.mb_1],
                attrs![At::Type=>"button"],
                style![St::Transition=> "all .15s ease"],
                if popover_show {
                    on_click(close_popover)
                } else {
                    on_click(open_popover)
                },
                class!["btn_ref"],
                "left PINK!"
            ],
            div![
                if popover_show {
                    class![]
                } else {
                    class![C.hidden]
                },
                class![ C.bg_pink_600 C.border_0 C.mr_3 C.block C.z_50 C.font_normal C.leading_normal C.text_sm C.max_w_xs C.text_left C.no_underline C.break_words C.rounded_lg],
                class![popover_ref_classname.as_ref()],
                div![
                    div![
                        class![
                  C.bg_pink_600 C.text_white C.opacity_70 C.font_semibold C.p_3 C.mb_0 C.border_b C.border_solid C.border_gray_200 C.uppercase C.rounded_t_lg],
                        "Popover Title",
                    ],
                    div![
            class![C.text_white C.p_3],
            "And here's some amazing content. It's very engaging. Right?"
          ]
                ]
            ]
        ]
    ]
}
