use crate::SEND_TO_VENDOR;

use uuid::Uuid;
use serde_json::{Value};
use serde::{Serialize};


// The structures that comprise the message sent to the REST API for an email
#[derive(Serialize)]
struct MessageBody {
  personalizations: Vec<Personalizations>,
  from: Email,
  content: Vec<Content>,
  tracking_settings: TrackingSettings,
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



#[derive(Serialize)]
struct TrackingSettings {
    click_tracking: ClickTracking,
    open_tracking: OpenTracking,
}

#[derive(Serialize)]
struct ClickTracking {
    enable: bool,
    enable_text: bool,
}

#[derive(Serialize)]
struct OpenTracking {
    enable: bool,
}











//************************************************************************
pub async fn send_email(account_fields: &Value, email_content: String, api_key: &String, client: &reqwest::Client) 
    -> Result<String, Box<dyn std::error::Error>> {

    // Get the fields for the email
    let email_to = &account_fields["email"].as_str().unwrap();
    let email_from = &account_fields["email_from"].as_str().unwrap();

    // TODO determine where the email subject is coming from in the final solution
    let email_subject = format!("A message from - {}", account_fields["client_name"].as_str().unwrap());

    // Create the body of the rest call
    let ps = vec!(Personalizations {
      to: vec!(Email {email: email_to.to_string()}),
      subject: email_subject,
    });

    let tracking_settings = TrackingSettings {
        click_tracking: ClickTracking {enable: true, enable_text: false},
        open_tracking: OpenTracking {enable: true},
    };

    let msg = MessageBody {
      personalizations: ps,
      from: Email {email: email_from.to_string()},
      content: vec!(Content {r#type: "text/html".to_string(), value: email_content}),
      tracking_settings: tracking_settings,
    };


    let json_msg = serde_json::to_string(&msg).expect("Failed to convert msg to json_msg");

    let url = "https://api.sendgrid.com/v3/mail/send";

    // Only make the call to the vendor solution if env var set correctly. This allows testing volume without making the calls
    if *SEND_TO_VENDOR {

        let mut res = client
            .post(url)
            .bearer_auth(api_key)
            .header("Content-Type", "application/json")
            .body(json_msg)
            .send()
//            .await
            .unwrap();

        // TODO if the response status is not 200 then an error needs to be generated
        // TODO need to deal with the errors that could come back from reqwest
        if !res.status().is_success() {
            error!("Response from send_email reqwest was failure. Status: {}, Text: {}", res.status(), res.text().unwrap());
        } else {

            // ch_channel_sender.1.3xvnh4xapb95@docker04    | [2020-04-10T19:06:17Z DEBUG channel_sender::email] Email header: ("x-message-id", "2if_CfKkSr65zEsp_SnAdA")
            
            // for header in res.headers() {
            //     debug!("Email header: {:?}", header);
            // }
            // let x_msg_id = "x-message-id";

            debug!("Response from send_email reqwest was success. Body: {}", res.text().unwrap());
            
            // Get the message id from the header using the "x-message-id"  attribute
//            let msg_id = resp.headers().get::<X-Message-Id>() match {
            let msg_id = res.headers().get("x-message-id").unwrap().to_str().unwrap().into();
            return Ok(msg_id)
            //  match {
            //     Some(id) => return Ok(id),
            //     None => return Ok("NO MESSAGE ID RETURNED FROM API CALL".to_string()),
            // };
        }
    } else {
        return Ok(Uuid::new_v4().to_string())
    }
    
    Ok("NO MESSAGE ID RETURNED FROM API CALL".to_string())
}


