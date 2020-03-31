#[macro_use]
extern crate reqwest;
// #[macro_use] 
extern crate hyper;

use serde::{Deserialize, Serialize};
use serde_json::{Value};


use reqwest::Client;
use futures::executor::block_on;

#[tokio::main]
async fn main() {

    block_on(send());



}

async fn send() {
    let email_content = format!("Hi jon this is an email from Rust. TIME=");
    let api_key = "<put key here>";

    println!("Starting to send_email. email_content: {:?}, api_key: {:?}", email_content, api_key);

    // Get the fields for the email
    let email_to = "jonc@destini.com";
    let email_from = "digital-communications@concordservicing.com";
    let email_subject = format!("A message from - {}", "destini");

    // Create the body of the request
    let filled_email_struct = format!(
        r#"{{"personalizations": [{{"to": [{{"email": "{}"}}],"subject": "{}"}}],"from": {{"email": "{}"}},"content": [{{"type": "text/html","value": "{}"}}]}}"#, 
        email_to, email_subject, email_from, email_content
    );

    println!("filled_email_struct: \n{}", filled_email_struct);

    let url = "https://api.sendgrid.com/v3/mail/send";

    // TODO look into passing this in to increase performance
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .body(filled_email_struct)
        .send()
        .await
        .unwrap();

//        "Content-Type: application/json"


//    let response_text = res.text().unwrap();
    println!("Response STATUS send_email reqwest: {:?}", res.status());
    println!("Response TEXT send_email reqwest: {:?}", res.text().await.unwrap());

    // TODO if the response status is not 200 then an error needs to be generated
    // TODO need to deal with the errors that could come back from reqwest
}