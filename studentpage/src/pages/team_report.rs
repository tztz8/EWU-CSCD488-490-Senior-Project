use yew::prelude::*;

pub struct TeamReport;
impl Component for TeamReport {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="col">
                // team reports page ...
                <p> { "Team Report" } </p>
            </div>
        }
    }
}