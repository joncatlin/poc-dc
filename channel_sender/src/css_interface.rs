use serde_json::{Value};
//use std::boxed;
//use std::collections::HashMap;
// use std::fs;
// use std::error::Error;

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

