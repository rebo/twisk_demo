// use_ref

use comp_state::{topo, use_state, StateAccess};
use seed::*;

#[topo::nested]
pub fn use_ref() -> StateAccess<Option<web_sys::HtmlElement>> {
    use_state::<Option<web_sys::HtmlElement>, _>(|| None).1
}

pub trait LazyHtmlElementStateAccessTrait {
    fn current(&self) -> Option<web_sys::HtmlElement>;
    fn selector(&self) -> String;
    fn class_name(&self) -> String;
}

impl LazyHtmlElementStateAccessTrait
    for StateAccess<Option<web_sys::HtmlElement>>
{
    fn class_name(&self) -> String {
        format!("selector{:#?}", self.id)
    }

    fn selector(&self) -> String {
        format!(".{}", self.class_name())
    }

    fn current(&self) -> Option<web_sys::HtmlElement> {
        if self.hard_get().is_none() {
            match document().query_selector(&self.selector()) {
                Ok(selection) => {
                    if let Some(Ok(html_element)) = selection.map(
                        wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>,
                    ) {
                        self.set(Some(html_element));
                    }
                },
                _ => return None,
            }
        }
        self.hard_get()
    }
}
