use crate::components::button::{Button, ButtonVariant};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// Properties for [NavBar].
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The onclick callback for logging out.
    pub logout: Callback<MouseEvent>,
}

/// The [Navbar] component provides a styled navbar for the application.
/// See <https://getbootstrap.com/docs/5.3/components/navbar/>
#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    html! {
        <nav class="border border-secondary-subtle rounded shadow
                    font-weight-bold text-light
                    navbar ps-3 pe-2 py-1 my-3">
            <span class="navbar-link">
                <Link<Route> to={ Route::Home } classes="link-light link-underline-opacity-0
                    link-underline-opacity-50-hover">
                    <h2 class="mb-1"> { "Status Reports" } </h2>
                </Link<Route>>
            </span>
            <span>
                <Button
                    variant={ ButtonVariant::Dark }
                    class="my-1"
                    label="Logout"
                    onclick={ &props.logout }/>
            </span>
        </nav>
    }
}
