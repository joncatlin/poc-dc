use crate::SEND_TO_VENDOR;

use serde_json::{Value};
use serde::{Serialize};
use log::Level::Trace;


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


//************************************************************************
pub async fn send_email(account_fields: &Value, email_content: String, api_key: &String) -> Result<(), Box<dyn std::error::Error>> {

    // Get the fields for the email
    let email_to = &account_fields["email"].as_str().unwrap();
    let email_from = &account_fields["email_from"].as_str().unwrap();

    // TODO determine where the email subject is coming from in the final solution
    let email_subject = format!("A message from - {}", account_fields["client_name"].as_str().unwrap());

    // Create the body of the rest call
//    let tos = vec!(Email {email: email_to.to_string()});

    let ps = vec!(Personalizations {
      to: vec!(Email {email: email_to.to_string()}),
      subject: email_subject,
    });

//    let ct = vec!(Content {r#type: "text/html".to_string(), value: email_content});
//    let ef = Email {email: email_from.to_string()};

    let msg = MessageBody {
      personalizations: ps,
      from: Email {email: email_from.to_string()},
      content: vec!(Content {r#type: "text/html".to_string(), value: email_content}),
    };

    let json_msg = serde_json::to_string(&msg).expect("Failed to convert msg to json_msg");


    // Create the body of the request
    // let filled_email_struct = format!(
    //     r#"{{"personalizations": [{{"to": [{{"email": "{}"}}],"subject": "{}"}}],"from": {{"email": "{}"}},"content": [{{"type": "text/html","value": "{}"}}]}}"#, 
    //     email_to, email_subject, email_from, email_content
    // );
    // debug!("filled_email_struct contains: [{}]", filled_email_struct);

    let url = "https://api.sendgrid.com/v3/mail/send";

    // TODO look into passing this in to increase performance, instead of building it each time
    let client = reqwest::Client::new();

    // Only make the call to the vendor solution if env var set correctly. This allows testing volume without making the calls
    if *SEND_TO_VENDOR {

        let res = client
            .post(url)
            .bearer_auth(api_key)
            .header("Content-Type", "application/json")
            .body(json_msg)
            .send()
            .await
            .unwrap();

        // TODO if the response status is not 200 then an error needs to be generated
        // TODO need to deal with the errors that could come back from reqwest
        if !res.status().is_success() {
            error!("Response from send_email reqwest was failure. Status: {}, Text: {}", res.status(), res.text().await.unwrap());
        }
    }
    
    Ok(())
}


