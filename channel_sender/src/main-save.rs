extern crate twilio;
use twilio::{Client,OutboundMessage};
fn main() {
    let to = "<to-number>";
    let from = "<from-number>";
    let body = "Hello, World! ";
    let app_id = "<app-id>";
    let auth_token = "<auth-token>";
    let client = Client::new(app_id,auth_token);
    let msg = OutboundMessage::new(from,to,body);
    match client.send_message(msg) {
        Err(e) => println!("{:?}",e),
        Ok(m)  => println!("{:?}",m),
    }
}










#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(())
}



available_number=`curl -X GET \
    "https://api.twilio.com/2010-04-01/Accounts/${account_sid}/AvailablePhoneNumbers/US/Local"  \
    -u "${account_sid}:${auth_token}" | \
    sed -n "/PhoneNumber/{s/.*<PhoneNumber>//;s/<\/PhoneNumber.*//;p;}"` \
    && echo $available_number 

    