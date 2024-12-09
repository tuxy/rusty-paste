use tiny_http::{Request, Response, Server};
use jasondb::Database;
use humphrey_json::prelude::*;
use nanoid::nanoid;
use std::{sync::Mutex, thread, time::{SystemTime, UNIX_EPOCH}};


mod config;
use config::Config;

#[derive(Clone, FromJson, IntoJson)]
struct PasteData {
    time_added: u64,
    time_limit: u64,
    content: Vec<u8>,
}

fn main() -> Result<(), std::io::Error> {
    // Opens configulation file
    let config = Config::open_config();

    let db: Database<PasteData> = Database::new("database.jdb").unwrap();
    let mut db_mutex: Mutex<Database<PasteData>> = Mutex::new(db);

    // Stores the id of the paste, as well as the content.
    let server = match Server::http(&config.bind_address) {
        Ok(server) => server,
        Err(err) => {
            eprintln!("Could bind to address: {}", err);
            panic!()
        }
    };

    // Creates a new thread and continuously loops through, checking the time limit of the pastes
    loop_and_check(&mut db_mutex).unwrap();

    for mut request in server.incoming_requests() {

        // Checks URL and reads post content
        let mut content = String::new();
        request
            .as_reader()
            .read_to_string(&mut content)
            .expect("Could not read to string");

        match request.url() {
            // Handle case for paste POST and URL creation
            "/" => {
                post_paste(request, &mut db_mutex, config.clone(), content);
            }
            // Handle case for paste GET and decryption
            _ => {
                // For now, pass on a placeholder
                get_paste(request, &mut db_mutex);
            }
        }
    };
    Ok(())
}

fn post_paste(request: Request, db: &mut Mutex<Database<PasteData>>, config: Config, content: String) {
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
                time_added: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
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

fn get_paste(request: Request, db: &mut Mutex<Database<PasteData>>) { // _config is currently unused
    // Removes the first character of the url, which is the '/'
    let mut url = request.url().chars();
    url.next();
    let url = url.as_str();

    // Splits the url with the '#' and collects into Vec. Then assigns var id and password
    let parts = url.split("!").collect::<Vec<_>>();

    let (id, password) = (parts[0], parts[1]); 

    // TODO
    let encrypted_data = db.lock().unwrap().get(id).unwrap().content;

    let crypt = simplestcrypt::deserialize_and_decrypt(password.as_bytes(), &encrypted_data);
    match crypt {
        // Handle decryption errors
        Ok(val) => {
            let response = 
                Response::from_string(String::from_utf8(val).unwrap());
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

fn loop_and_check(db_unlock: &mut Mutex<Database<PasteData>>) -> Result<(), std::io::Error> {
    thread::scope(|s| {
        loop {
            s.spawn(|| {
                // This is where the time limits of each paste is monitored and deleted accordingly.
                // Q: How to delete safely while the request loop accesses it?

                // TODO
                let mut db=  db_unlock.lock().unwrap();

                let time_now_in_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                

                for i in db.iter() {

                    let paste_data = i.unwrap();
                    if time_now_in_sec - paste_data.1.time_added >= paste_data.1.time_limit {
                        // God please fix this. What kind of monster have I created. TODO
                        db_unlock.lock().unwrap().delete(paste_data.0).unwrap();
                    }

                };
        }).join().unwrap();
       }
    });

    Ok(())
}