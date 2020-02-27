#[macro_use]
extern crate log; 
extern crate env_logger;
extern crate chrono;
extern crate reqwest;

use env_logger::Builder;
use std::fs;
use chrono::{Local};
use std::io::Write;
use log::LevelFilter;
use reqwest::Client;
use std::collections::HashMap;

// Constants
static ACCOUNT_ID: &str = "account_id";
static TOKEN: &str = "token";
static SECRET_PATH: &str = "/run/secrets/";

#[tokio::main]
async fn main() {

        // Initialize the logger for stdout
        Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

        // Get the secrets
        // let account_id = get_secret(ACCOUNT_ID);
        // info!("The contents of account_id: {}", account_id);
        // let token = get_secret(TOKEN);
        // info!("The contents of token: {}", token);

        send_whatsapp().await;
}


async fn send_whatsapp() {
    
    let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
    let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

    // Send a Whatsapp msg
    let mut params = HashMap::new();
    params.insert("To", "whatsapp:+14805169974");
    params.insert("From", "whatsapp:+14155238886");
    params.insert("Body", "Hello, World! from Rust");
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/whatsapp");
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_id);
    let client = Client::new();
    let res = client
        .post(&url)
        .basic_auth(account_id, Some(token))
        .form(&params)
        .send()
        .await
        .unwrap();

    let mytext = res.text().await.unwrap();
    println!("{}", mytext);
}

async fn send_sms() {
    let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
    let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

    // Send an SMS
    let mut params = HashMap::new();
    params.insert("To", "+14805169974");
    params.insert("From", "+14804053433");
    params.insert("Body", "Hello, World! from Rust");
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/sms");
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_id);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .basic_auth(account_id, Some(token))
        .form(&params)
        .send()
        .await
        .unwrap();

    let mytext = res.text().await.unwrap();
    println!("{}", mytext);
}









// static async Task Execute()
// {
//     var apiKey = "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";
// //            var apiKey = Environment.GetEnvironmentVariable("SENDGRID_APIKEY");
//     var client = new SendGridClient(apiKey);
//     var from = new EmailAddress("jonc@destini.com", "Jon Catlin");
//     var subject = "Testing sending with SendGrid";
//     var to = new EmailAddress("jon.catlin@destini.com", "Jon T Catlin");
//     var plainTextContent = "This is an easy api to use";
//     var htmlContent = "<strong>Just figuring out how to use this api</strong>";
//     var msg = MailHelper.CreateSingleEmail(from, to, subject, plainTextContent, htmlContent);
//     var response = await client.SendEmailAsync(msg);
// }


//format!("https://api.sendgrid.com/api/{}.{}.{}?api_user={}&api_key={}", module, action, format);






async fn send_email() {
    let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
    let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

    // Send an SMS
    let mut params = HashMap::new();
    params.insert("To", "+14805169974");
    params.insert("From", "+14804053433");
    params.insert("Body", "Hello, World! from Rust");
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/sms");
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_id);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .basic_auth(account_id, Some(token))
        .form(&params)
        .send()
        .await
        .unwrap();

    let mytext = res.text().await.unwrap();
    println!("{}", mytext);
}


fn get_secret(name: &str) -> String {
    let file_name = format!("{}{}", SECRET_PATH, name);
    let secret = match fs::read_to_string(&file_name) {
        Ok(s) => s,
        Err(e) => {
            error!("Error occurred trying to find a secret named:{}. The error was:{}", &file_name, e);
            panic!("secret not found, cannot continue")
        },
    };
    secret
}
