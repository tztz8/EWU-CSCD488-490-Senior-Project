use yew::prelude::*;

/// Variants for [Collapse]
#[derive(PartialEq)]
pub enum CollapseVariant {
    Incomplete,
    Complete,
    Invalid,
}

/// Properties for [Collapse]   
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The text to display on the button.
    pub label: AttrValue,
    /// The id of the element to collapse.
    pub target: AttrValue,
    /// The variant of the button.
    #[prop_or(CollapseVariant::Incomplete)]
    pub variant: CollapseVariant,
}

/// The [Collapse] component provides a styled collapse button.
/// Collapses the element with the given id when clicked.
#[function_component(Collapse)]
pub fn collapse(props: &Props) -> Html {
    let get_btn_variant = |variant: &CollapseVariant| match variant {
        CollapseVariant::Incomplete => "btn-light",
        CollapseVariant::Complete => "btn-success",
        CollapseVariant::Invalid => "btn-warning",
    };

    let get_icon_variant = |variant: &CollapseVariant| match variant {
        CollapseVariant::Incomplete => html! { <i class="fas fa-minus fa-xl me-2"></i>},
        CollapseVariant::Complete => html! { <i class="fas fa-check fa-xl me-2"></i>},
        CollapseVariant::Invalid => html! { <i class="fas fa-xmark fa-xl me-2"></i>},
    };

    html! {
        <div class="col-12 mt-3">
            <button
                type="button"
                class={ format!("btn shadow text-start col-auto {}", get_btn_variant(&props.variant)) }
                data-bs-toggle="collapse"
                data-bs-target={ props.target.clone() }
                style="background-image: linear-gradient(135deg, #FFFFFF20, #00000020)">
            { get_icon_variant(&props.variant) }
            { &*props.label }
            </button>
        </div>
    }
}
