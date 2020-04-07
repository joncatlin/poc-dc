extern crate reqwest;
extern crate hyper;

use serde::{Serialize};
use serde_json::{Value};


//use reqwest::Client;
use futures::executor::block_on;

// The structures that comprise the message sent to the REST API for an email
#[derive(Serialize)]
struct MessageBody {
  personalizations: Vec<Personalizations>,
  from: Email,
  content: Vec<Content>,
}

#[derive(Serialize)]
struct Personalizations {
  to: Vec<Email>,
  subject: String,
}

#[derive(Serialize)]
struct Email {
  email: String,
}

#[derive(Serialize)]
struct To {
  to: Vec<Email>,
}

// Need to escape the reserved word
#[derive(Serialize)]
struct Content {
  r#type: String,
  value: String,
}


#[tokio::main]
async fn main() {


    let mut template_fields = Vec::<String>::new();

//    let accounts = vec!("account1".to_string(), "account2".to_string());
    let accounts = vec!("account1".to_string());

    let email_vendor_token = "<put token here>".to_string();

    // Get the fields for the account
    for account in accounts {
        let email_content = format!(r#"
          <p>Hi jon this is an email from Rust. TIME=</p>
          <div>
            <ul>
              <p>Item1</p>
              <p>Item1</p>
              <p>Item1</p>
              <p>Item1</p>
              <p>Item1</p>
              <p>Item1</p>
            </ul>
            <ol>
              <p>Tew1</p>
              <p>Tew2</p>
            </ol>
          </div>
          <p>This is the imge from manorfarm nurseries.com</p>
          <img alt="MFN Image" src="http://manorfarmnurseries.com/wp-content/themes/yootheme/cache/HomePage-b4d85cc4.jpeg"
            width="400 height="400"
          >
          <p>End of the image</p>
        "#);
        let account_fields = get_account_fields(&account, &template_fields);
        block_on(send(&account_fields, email_content, &email_vendor_token));
    }





}

async fn send(account_fields: &Value, email_content: String, api_key: &String) {

    println!("Starting to send_email. email_content: {:?}, api_key: {:?}", email_content, api_key);

    // Get the fields for the email
    // let email_to = "jonc@destini.com";
    // let email_from = "digital-communications@concordservicing.com";
    // let email_subject = format!("A message from - {}", "destini");

    // Get the fields for the email
    let email_to = &account_fields["email"].as_str().unwrap();
    let email_from = &account_fields["email_from"].as_str().unwrap();
    let email_subject = format!("A message from - {}", account_fields["client_name"].as_str().unwrap());

    let tos = vec!(Email {email: email_to.to_string()});

    let ps = vec!(Personalizations {
      to: tos,
      subject: email_subject,
    });

    let ct = vec!(Content {r#type: "text/html".to_string(), value: email_content});
    let ef = Email {email: email_from.to_string()};

    let msg = MessageBody {
      personalizations: ps,
      from: ef,
      content: ct,
    };

    let json_msg = serde_json::to_string(&msg).expect("Failed to convert msg to json_msg");


    // struct MessageBody {
    //   personalizatios: Vec<Personalizations>,
    //   from: Email,
    //   content: Content,
    // }

    // struct Personalizations {
    //   to: Vec<Email>,
    //   subject: String,
    // }

    // struct Email {
    //   email: String,
    // }

    // struct To {
    //   to: Vec<Email>,
    // }

    // struct Content {
    //   type: String,
    //   value: String,
    // }

    // Create the body of the request
    // let filled_email_struct = format!(
    //     r#"{{"personalizations": [{{"to": [{{"email": "{}"}}],"subject": "{}"}}],"from": {{"email": "{}"}},"content": [{{"type": "text/html","value": "{}"}}]}}"#, 
    //     email_to, email_subject, email_from, email_content
    // );

    // println!("filled_email_struct: \n{}", filled_email_struct);

    let url = "https://api.sendgrid.com/v3/mail/send";

    // TODO look into passing this in to increase performance
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .body(json_msg)
        .send()
        .await
        .unwrap();

// //        "Content-Type: application/json"


// //    let response_text = res.text().unwrap();
    println!("Response STATUS send_email reqwest: {:?}", res.status());
    println!("Response TEXT send_email reqwest: {:?}", res.text().await.unwrap());

    // TODO if the response status is not 200 then an error needs to be generated
    // TODO need to deal with the errors that could come back from reqwest
}
















//************************************************************************
pub fn get_account_fields (account: &String, template_fields: &Vec<String>) -> Value {

    // Call CSS to get a list of fields for a given account. In the future this would be a call to CSS
    // but for the POC this is a local call to get dummy data.
    let fields = css_get_account_fields (account, template_fields);
    let v: Value = serde_json::from_str(&*fields).unwrap();
    v
}


//************************************************************************
pub fn css_get_account_fields (_account: &String, _template_fields: &Vec<String>) -> String {

    let data: String = String::from(r#"{
        "id": 1,
        "first_name": "Jon",
        "last_name": "Catlin",
        "email": "jonc@destini.com",
        "gender": "m",
        "address1": "5315 Portage Alley",
        "address2": "",
        "address3": "",
        "city": "Washington",
        "state": "District of Columbia",
        "zip": "20051",
        "accounts": [
          {
            "days_delinquent": 17,
            "amount_due": 1852.83,
            "account_number": "853095139-5"
          },
          {
            "days_delinquent": 22,
            "amount_due": 2468.48,
            "account_number": "046270384-3"
          },
          {
            "days_delinquent": 3,
            "amount_due": 2023.51,
            "account_number": "912733932-7"
          },
          {
            "days_delinquent": 9,
            "amount_due": 2264.79,
            "account_number": "014258677-3"
          }
        ],
        "currency": "£",
        "client_id": "tt",
        "client_name": "Thousand Trails",
        "email_from": "digital-communications@concordservicing.com",
        "phone_mobile": "+14805169974"
    }"#);

    data
}

