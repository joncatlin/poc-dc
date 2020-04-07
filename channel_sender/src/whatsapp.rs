use std::collections::HashMap;
use reqwest::Client;
use serde_json::{Value};
use log::Level::Trace;
use log::{debug, error, warn, trace, info, log_enabled};



//************************************************************************
pub async fn send_whatsapp(account_fields: &Value, whatsapp_content: String, vendor_acc_id: &String, vendor_token: &String) -> 
    Result<(), Box<dyn std::error::Error>> {

    // TODO ensure the to phone number is in the correct international format


    // Send a Whatsapp msg
    let mut params = HashMap::new();
    let whatsapp_to = format!("whatsapp:{}", account_fields["phone_mobile"]);
    params.insert("To", whatsapp_to);
    params.insert("From", "whatsapp:+14155238886".to_string());
    params.insert("Body", whatsapp_content);
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/whatsapp".to_string());

    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", vendor_acc_id);

    let client = reqwest::Client::new();

    // Only make the call to the vendor solution if we are not tracing. This allows testing volume without making the calls
    if !log_enabled!(Trace) {

        let res = client
            .post(&url)
            .basic_auth(vendor_acc_id, Some(vendor_token))
            .form(&params)
            .send()
            .await
            .unwrap();

        let response_text = res.text().await.unwrap();
        debug!("Response from send_sms reqwest: {}", response_text);
    }
    // TODO if the response status is not 200 then an error needs to be generated
    // TODO need to deal with the errors that could come back from reqwest

    Ok(())
}
    
    
    