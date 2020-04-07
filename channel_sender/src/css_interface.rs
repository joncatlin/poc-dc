use rand::Rng;
use serde_json::{Value};
use std::sync::Mutex;
// use std::path::Path;
// use std::error::Error;
// use std::fs::File;
// use std::io::{Read, Write};

use std::fs;


lazy_static! {
    //            static ref DATA: Mutex<Vec<Value>> = Mutex::new(vec![v]);
    static ref DATA: Mutex<Vec<Value>> = Mutex::new(vec![]);
}
    
    
fn main() {

    let account = "Account1".to_string();
    let fields = Vec::<String>::new();
    let account_fields = get_account_fields (&account, &fields);
    println!("value = {:?}", account_fields);

    let email_to = &account_fields["email"].as_str().unwrap();
    let email_from = &account_fields["email_from"].as_str().unwrap();
    println!("email_to = {:?}", email_to);
    println!("email_from = {:?}", email_from);

}

//static mut data_initialized: bool = false;

//************************************************************************
pub fn get_account_fields (account: &String, template_fields: &Vec<String>) -> Value {

    // Call CSS to get a list of fields for a given account. In the future this would be a call to CSS
    // but for the POC this is a local call to get dummy data.
    let fields = css_get_account_fields (account, template_fields);
    fields
}


//************************************************************************
pub fn css_get_account_fields (_account: &String, _template_fields: &Vec<String>) -> Value {

    // IGNORE THE FIELDS PASSED IN


    if DATA.lock().unwrap().to_vec().len() == 0 {

        // Read data from file for all the dummy CSS values
        let file_contents = fs::read_to_string("./mock_data.json").expect("error on reading json data from file");

        let mut array: Vec<Value> = serde_json::from_str(&file_contents).expect("");

        DATA.lock().unwrap().append(&mut array);

//        let v: Value = serde_json::from_str(&*fields).unwrap();

//         // Save the date in the global reference
//         lazy_static! {
// //            static ref DATA: Mutex<Vec<Value>> = Mutex::new(vec![v]);
//             static ref DATA: Mutex<Vec<Value>> = Mutex::new(array);
//         }

//        data_initialized = true;
    }

    // Randomly pick one of the accounts and pass it back
    let mut rng = rand::thread_rng();
    let length = DATA.lock().unwrap().to_vec().len()-1;
    let index = rng.gen_range(0, length);

    debug!("Index generated is: {} and length is: {}", index, length);

    DATA.lock().unwrap().to_vec()[index].to_owned()
}



