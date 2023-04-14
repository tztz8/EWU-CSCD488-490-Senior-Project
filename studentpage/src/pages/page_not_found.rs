use yew::prelude::*;

#[function_component]
pub fn PageNotFound() -> Html {
    html! {
        <div class="card shadow text-danger border-danger">
            <div class="card-body">
                <h5 class="card-title"> {"Page not found! "} </h5>
                <p class="card-text"> { "Page does not exist." } </p>
            </div>
        </div>
    }
}