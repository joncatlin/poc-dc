use crate::SEND_TO_VENDOR;

use html2text;
use std::collections::HashMap;
use serde_json::{Value};
use log::Level::Trace;


//************************************************************************
pub async fn send_sms(account_fields: &Value, sms_content: String, vendor_acc_id: &String, vendor_token: &String) -> Result<(), Box<dyn std::error::Error>> {

    // Convert SMS content to text as it will come in HTML format
    let converted_sms_content = html2text::from_read(sms_content.as_bytes(), 80);

    // TODO ensure the to phone number is in the correct international format
    
    // Send an SMS
    let mut params = HashMap::new();
    params.insert("To", account_fields["phone_mobile"].to_string());
    params.insert("From", "+14806603651".to_string());
    params.insert("Body", converted_sms_content);
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/sms".to_string());
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", vendor_acc_id);

    let client = reqwest::Client::new();
    // Only make the call to the vendor solution if env var set correctly. This allows testing volume without making the calls
    if *SEND_TO_VENDOR {

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


