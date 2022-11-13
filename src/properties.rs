use serde::{Deserialize, Serialize};
use yew::functional::*;
use yew::html;

use crate::db::supabase::get_supabase_client;

#[derive(Serialize, Deserialize, Debug)]
struct Property {
    id: String,
    created_at: String,
    org: String,
    address: String,
    price: f32,
    sqmeter: f32,
    headline: String,
    thumbnail: String,

    #[serde(rename = "imageLinks")]
    image_links: Vec<String>,
    #[serde(rename = "propertyLink")]
    property_link: String,
}

#[function_component(Video)]
pub fn app() -> Html {
    let property = use_state(|| vec![]);
    {
        let property = property.clone();
        use_effect_with_deps(
            move |_| {
                let property = property.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = get_supabase_client()
                        .from("properties")
                        .select("*")
                        .execute()
                        .await
                        .unwrap();

                    let resp_text: Vec<Property> =
                        serde_json::from_str(&resp.text().await.unwrap()).unwrap();

                    property.set(resp_text)
                });
                || ()
            },
            (),
        );
    }

    let html_property = property.iter().map(|property| {
        html! {
            <p>{property.property_link.to_owned()}</p>
        }
    });

    html! {
        <>
            <div>
                <h3>{"Latest Properties"}</h3>
                { for html_property }
            </div>

        </>
    }
}
