use std::collections::HashMap;

pub fn send_mr(title: &str, message: &str) {
    let webhook = "";

    let mut body = HashMap::new();
    body.insert("title", &title);
    body.insert("text", &message);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(webhook)
        .json(&body)
        .header("Content-Type", "application/json")
        .send()
        .expect("Failed to post MR to teams");
}
