use crate::SEND_TO_VENDOR;

use uuid::Uuid;
use html2text;
use std::collections::HashMap;
use serde_json::{Value};
use serde::{Deserialize};

#[derive(Deserialize)]
struct MsgId {
    sid: String,
}


//************************************************************************
pub async fn send_sms(account_fields: &Value, sms_content: String, vendor_acc_id: &String, vendor_token: &String, client: &reqwest::Client) 
-> Result<String, Box<dyn std::error::Error>> {

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

//    let client = reqwest::Client::new();
    // Only make the call to the vendor solution if env var set correctly. This allows testing volume without making the calls
    if *SEND_TO_VENDOR {

        let mut res = client
            .post(&url)
            .basic_auth(vendor_acc_id, Some(vendor_token))
            .form(&params)
            .send()
//            .await
            .unwrap();

        if !res.status().is_success() {
            error!("Response from send_sms reqwest was failure. Status: {}, Text: {}", res.status(), res.text().unwrap());
        } else {


            // Response from send_sms reqwest was success. The response text is: <"{\"sid\": \"SM320fb3c756f945b7ac4308b68e25ec8c\", 
            //     \"date_created\": \"Sun, 12 Apr 2020 22:02:03 +0000\", \"date_updated\": \"Sun, 12 Apr 2020 22:02:03 +0000\", \"date_sent\": null, 
            //     \"account_sid\": \"ACf78ce29e3249a6ddd415f8ec27681c52\", \"to\": \"+14805169974\", \"from\": \"+14806603651\", 
            //     \"messaging_service_sid\": null, \"body\": 
            //         \"Sent from your Twilio trial account - This is the sms template\\n\\n\\n\\n[tt logo]\\n\\n\\nFrom address\\n\\n\\nAusten Dunbobin\\n2267 Maywood Parkway\\n\\n\\nFresno\\nCalifornia 93778\\n\\n\\nDear Mrs Dunbobin thank you for being a valued customer of ours.\\n\\nWe would like to extend this wonderful offer to you. Please go to our website at\\nwww.concordservicing.com to sign up.\", \"status\": \"queued\", \"num_segments\": \"3\", \"num_media\": \"0\", \"direction\": \"outbound-api\", \"api_version\": \"2010-04-01\", \"price\": null, \"price_unit\": \"USD\", \"error_code\": null, \"error_message\": null, \"uri\": \"/2010-04-01/Accounts/ACf78ce29e3249a6ddd415f8ec27681c52/Messages/SM320fb3c756f945b7ac4308b68e25ec8c.json\", \"subresource_uris\": {\"media\": \"/2010-04-01/Accounts/ACf78ce29e3249a6ddd415f8ec27681c52/Messages/SM320fb3c756f945b7ac4308b68e25ec8c/Media.json\"}}">








//            let mut response_json: Vec<Value> = serde_json::from_str(&res.text().unwrap()).unwrap();
            let res_text = res.text().unwrap();
            debug!("Response from send_sms reqwest was success. The response text is: <{}>", res_text);
//            return Ok("1".to_string());

            // Get the id of the message from the body of the response
            let msg_id_struct: MsgId = serde_json::from_str(&res_text)
                .expect("Failed to get sid from response to sending sms msg. Without the sid there is a fundamental problem with matching what was sent with the returning calls to the webhook");
            // let msg_id_struct: MsgId = serde_json::from_str(&res_text) match {
            //         Ok(s) =>    s,
            //     Err(e) =>   return Err(e),
            // };

            debug!("SMS msg id: {}", msg_id_struct.sid);
            return Ok(msg_id_struct.sid)
        }
    } else {
        return Ok(Uuid::new_v4().to_string())
    }
    // TODO if the response status is not 200 then an error needs to be generated
    // TODO need to deal with the errors that could come back from reqwest
    // TODO. Get the message id and pass it back to the caller
    Ok("".to_string())
}


