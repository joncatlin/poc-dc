#[macro_use]
extern crate log; 
extern crate env_logger;
extern crate chrono;
extern crate twilio;

use twilio::{Client,OutboundMessage};
use env_logger::Builder;
use std::fs;
use chrono::{Local};
use std::io::Write;
use log::LevelFilter;


// Constants
static ACCOUNT_ID: &str = "account_id";
static TOKEN: &str = "token";
static SECRET_PATH: &str = "/run/secrets/";
//static SECRET_PATH: &str = "./";

fn main() {

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

        let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
        let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

        // Send an SMS
//        let client = twilio::Client::new(account_id, token);

        let to = "+14805169974";
        let from = "+14804053433";
        let body = "Hello, World! from Rust";
        let client = Client::new(account_id, token);
        let msg = OutboundMessage::new(from,to,body);
        match client.send_message(msg) {
            Err(e) => println!("{:?}",e),
            Ok(m)  => println!("{:?}",m),
        }







        statusCallback: new Uri("https://postb.in/1581798165421-1482100901193")


//        client.send_message(OutboundMessage::new(from,to,"Hello, World!"));
        client.send_message(OutboundMessage::new(from,to,"Hello, World!"));
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
