use crate::mdc_sys::MDCSelect;
use boolinator::Boolinator;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

pub mod item;
pub use item::Item;

pub struct Select {
    changed_callback: Closure<dyn FnMut(web_sys::CustomEvent)>,
    inner: Option<MDCSelect>,
    node_ref: NodeRef,
}

#[derive(Properties, Clone, PartialEq)]
pub struct SelectProps {
    pub children: Children,
    pub select_width_class: String,
    pub id: String,
    #[prop_or_default]
    pub label: Option<&'static str>,
    #[prop_or_default]
    pub fixed_position: bool,
    #[prop_or_default]
    pub absolute_position: Option<(i32, i32)>,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<SelectChangeEventData>,
    #[prop_or_default]
    pub selected_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectChangeEventData {
    pub value: String,
    pub index: i64,
}

pub enum SelectMsg {
    Changed(SelectChangeEventData),
}

impl Component for Select {
    type Message = SelectMsg;
    type Properties = SelectProps;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|data| SelectMsg::Changed(data));
        let closure = Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
            e.stop_propagation();
            let event_data = e.detail().into_serde::<SelectChangeEventData>().expect(
                "Expected a JS object in the format { \"value\": string, \"index\": number }",
            );
            callback.emit(event_data);
        }) as Box<dyn FnMut(web_sys::CustomEvent)>);
        Self {
            changed_callback: closure,
            inner: None,
            node_ref: NodeRef::default(),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(elem) = self.node_ref.cast::<Element>() {
                let select = MDCSelect::new(elem);
                if let Some(selected_value) = ctx.props().selected_value.clone() {
                    select.set_value(selected_value.as_str());
                }
                select.listen("MDCSelect:change", &self.changed_callback);

                self.inner = Some(select);
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SelectMsg::Changed(data) => ctx.props().onchange.emit(data),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = classes![
            "mdc-select",
            "mdc-select--filled",
            ctx.props().select_width_class.clone(),
            ctx.props().label.is_none().as_some("mdc-select--no-label")
        ];
        let menu_classes = classes![
            "mdc-menu",
            "mdc-menu-surface",
            "mdc-select__menu",
            ctx.props().select_width_class.clone(),
            ctx.props()
                .fixed_position
                .as_some("mdc-menu-surface--fixed")
                .or_else(|| Some("mdc-menu-surface--fullwidth"))
        ];
        let label_id = format!("{}-label", &ctx.props().id);
        let selected_text_id = format!("{}-selected-text", &ctx.props().id);
        let label = if ctx.props().label.is_none() {
            html! {}
        } else {
            html! {
                <span id={label_id.clone()} class="mdc-floating-label">
                    { ctx.props().label.as_ref().unwrap() }
                </span>
            }
        };
        html! {
            <div id={ctx.props().id.clone()} class={classes} ref={self.node_ref.clone()}>
                <div class="mdc-select__anchor"
                    role="button"
                    aria-haspopup="listbox"
                    aria-expanded="false"
                    aria-labelledby={ format!("{} {}", &label_id, &selected_text_id)}>

                    <span class="mdc-select__ripple"></span>
                    { label }
                    <span class="mdc-select__selected-text-container">
                        <span id={selected_text_id} class="mdc-select__selected-text"></span>
                    </span>
                    <span class="mdc-select__dropdown-icon">
                    <svg
                        class="mdc-select__dropdown-icon-graphic"
                        viewBox="7 10 10 5" focusable="false">
                        <polygon
                            class="mdc-select__dropdown-icon-inactive"
                            stroke="none"
                            fill-rule="evenodd"
                            points="7 10 12 15 17 10">
                        </polygon>
                        <polygon
                            class="mdc-select__dropdown-icon-active"
                            stroke="none"
                            fill-rule="evenodd"
                            points="7 15 12 10 17 15">
                        </polygon>
                    </svg>
                    </span>
                    <span class="mdc-line-ripple"></span>
                </div>
                <div class={menu_classes}>
                    <ul class="mdc-deprecated-list"
                        role="listbox"
                        aria-label={format!("{} listbox", ctx.props().label.unwrap_or_default())}>
                        { ctx.props().children.clone() }
                    </ul>
                </div>
            </div>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(inner) = &self.inner {
            inner.unlisten("MDCSelect:change", &self.changed_callback);
            inner.destroy();
        }
    }
}
