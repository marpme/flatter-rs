use yew::prelude::*;

use crate::layout::*;
use crate::properties::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Layout>
            <Video />
        </Layout>
    }
}
