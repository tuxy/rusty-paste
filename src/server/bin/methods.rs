use tiny_http::{Request, Response};
use jasondb::Database;
use humphrey_json::prelude::*;
use nanoid::nanoid;
use std::{sync::Mutex, time::{SystemTime, UNIX_EPOCH}};

use crate::config::Config;

#[derive(Clone, FromJson, IntoJson)]
pub struct PasteData {
    pub time_added: u64,
    pub time_limit: u64,
    pub content: Vec<u8>,
}

// Takes in the content, encrypts it and then adds it to the JasonDB 'database'. 
pub fn post_paste(request: Request, db: &mut Mutex<Database<PasteData>>, config: Config, content: String) {
    // Set up encryption for URL
    let password = nanoid!(8);
    let bind_address = config.bind_address;

    let crypt = simplestcrypt::encrypt_and_serialize(password.as_bytes(), content.as_bytes());
    match crypt {
        Ok(ciphertext) => {              
            let id = nanoid!(8);

            // Appends to database
            db.lock().unwrap().set(&id, PasteData {
                // wtf is this ahhhh
                time_added: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs(),
                time_limit: config.time_limit,
                content: ciphertext
            }).unwrap();

            let response = 
                Response::from_string(
                    format!("https://{bind_address}/{id}!{password}")
                );
            let _ = request.respond(response);
        },
        Err(err) => {
            let response = 
            Response::from_string(
                format!("Could not encrypt data: {:?}", err)
            );
            let _ = request.respond(response);
        }
    };
}

// Parses the URL in the GET request and splits it into its id and password, creates the key from the password,
// decrypts the data and then sends it back. Technically not end to end encryption, since the work is done on the 
// server and not the client to encrypt, and so technically the server has knowledge of the content keys.  
pub fn get_paste(request: Request, db: &mut Mutex<Database<PasteData>>) { // _config is currently unused
    // Removes the first character of the url, which is the '/'
    let mut url = request.url().chars();
    url.next();
    let url = url.as_str();

    // Splits the url with the '#' and collects into Vec. Then assigns var id and password
    let parts = url.split("!").collect::<Vec<_>>();

    // TODO, this could be empty
    let (id, password) = (parts[0], parts[1]); 

    // TODO: Implement loop?
    let encrypted_data = db.lock()
        .expect("Could not get lock on database")
        .get(id)
        .expect("Could not find paste")
        .content;

    let crypt = simplestcrypt::deserialize_and_decrypt(password.as_bytes(), &encrypted_data);
    match crypt {
        // Handle decryption errors
        Ok(val) => {
            let response = 
                Response::from_string(String::from_utf8(val).expect("Couud not encode contents"));
            let _ = request.respond(response);
        },
        // Return the decryption error to the client if decryption was to fail
        Err(err) => {
            let response = 
                Response::from_string(
                    format!("Could not decrypt data: {:?}", err)
                );
            let _ = request.respond(response);
        }
    };
}

// Loops through the database and checks whether the paste has expired, and deletes it if it has. 
pub fn loop_and_check(db_unlock: &mut Mutex<Database<PasteData>>) -> Result<(), std::io::Error> {
    // This is where the time limits of each paste is monitored and deleted accordingly.
    // Q: How to delete safely while the request loop accesses it?

    // TODO: Implement loop?
    let mut db=  db_unlock.lock().expect("Could not get lock on database");

    let time_now_in_sec = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    

    for i in db.iter() {

        let paste_data = i.unwrap();
        if time_now_in_sec - paste_data.1.time_added >= paste_data.1.time_limit {
            // TODO: Implement loop
            db_unlock
                .lock()
                .expect("Could not get lock on database")
                .delete(paste_data.0)
                .expect("Could not delete key from database. Check permissions?");
        }

    };

    Ok(())
}