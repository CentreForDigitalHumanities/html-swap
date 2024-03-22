use core::panic;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn perform_swap(doc: &web_sys::Document, swap_element: web_sys::Element) -> Result<(), JsValue> {
    let id = swap_element.get_attribute("id").expect("id not found");
    let target = doc
        .get_element_by_id(&id)
        .expect("element not found");
    target.set_inner_html(&swap_element.inner_html());
    Ok(())
}

#[wasm_bindgen]
pub fn parse_swap_response(body: &str) -> Vec<web_sys::Element> {
    let doc = web_sys::Document::new().unwrap();
    let parser = doc.create_element("div").unwrap();
    parser.set_inner_html(body);
    let elements = parser.children();
    let mut vec = vec![];
    for i in 0..elements.length() {
        let element = elements.get_with_index(i).unwrap();
        vec.push(element);
    }
    vec
}

#[wasm_bindgen]
pub fn perform_page_update(elements: Vec<web_sys::Element>) {
    let doc = web_sys::window().unwrap().document().unwrap();
    for element in elements {
        perform_swap(&doc, element).unwrap();
    }
}

#[wasm_bindgen]
pub async fn anchor_swap(el: &web_sys::HtmlAnchorElement) {
    let href = el.href();
    let body = reqwest::get(&href).await.unwrap().text().await.unwrap();
    let update = parse_swap_response(&body);
    perform_page_update(update);
}

#[wasm_bindgen]
pub async fn form_swap(el: &web_sys::HtmlInputElement) {
    let form = el.form().unwrap();
    let mut form_params = vec![];
    for i in 0..form.elements().length() {
        let element = form.elements().get_with_index(i).unwrap();
        if let Some(input) = element.dyn_ref::<web_sys::HtmlInputElement>() {
            let name = input.name();
            let value = input.value();
            if name != "" && value != "" {
                form_params.push(format!("{}={}", name, value));
            }
        }
    };
    let action = form.action();
    let response_body = match form.method().as_str() {
        "get" => {
            let query_params = form_params.join("&");
            let url = format!("{}?{}", action, query_params);
            reqwest::get(&url).await.unwrap().text().await.unwrap()
        },
        "post" => {
            reqwest::Client::new()
                .post(&action)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(form_params.join("&"))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        },
        _ => panic!("Unsupported form method")
    };
    let update = parse_swap_response(&response_body);
    perform_page_update(update);
}


#[wasm_bindgen]
pub async fn swap_ctl(el: web_sys::Element) {
    let tag_name = el.tag_name().to_lowercase();
    match tag_name.as_str() {
        "a" => {
            let anchor_el = el.dyn_ref::<web_sys::HtmlAnchorElement>().unwrap();
            anchor_swap(anchor_el).await;
        },
        "input" => {
            let input_el = el.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
            // check if input_el type is submit
            if input_el.type_() == "submit" {
                form_swap(input_el).await;
            }
        },
        _ => ()
    }

}