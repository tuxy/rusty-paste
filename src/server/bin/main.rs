use tiny_http::{Request, Response, Server};
use nanoid::nanoid;
use std::thread;

mod config;
use config::Config;

struct PasteData {
    id: String,
    content: Vec<u8>,
}

fn main() {
    // Opens configulation file
    let config = Config::open_config();

    // Stores the id of the paste, as well as the content.
    // TODO: aes-gcm symetric encryption
    let mut paste_data: Vec<PasteData> = Vec::new();
    let server = match Server::http(&config.bind_address) {
        Ok(server) => server,
        Err(err) => {
            eprintln!("Could bind to address: {}", err);
            panic!()
        }
    };

    thread::spawn(|| {
        loop {
            // This is where the time limits of each paste is monitored and deleted accordingly.
            // Q: How to delete safely while the request loop accesses it?
            todo!()
        }
    });

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
                post_paste(request, &mut paste_data, config.clone(), content);
            }
            // Handle case for paste GET and decryption
            _ => {
                // For now, pass on a placeholder
                get_paste(request, &paste_data, Config {bind_address: String::from("test")});
            }
        }
    }
}

fn post_paste(request: Request, paste_data: &mut Vec<PasteData>, config: Config, content: String) {
    // Set up encryption for URL
    let password = nanoid!(8);
    let bind_address = config.bind_address;

    let crypt = simplestcrypt::encrypt_and_serialize(password.as_bytes(), content.as_bytes());
    match crypt {
        Ok(ciphertext) => {              
            let id = nanoid!(8);
            // Places id of the paste as well as the paste data into the array
            paste_data.push(PasteData {
                id: id.clone(),
                content: ciphertext,
            });

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

fn get_paste(request: Request, paste_data: &Vec<PasteData>, _config: Config) { // _config is currently unused
    // Removes the first character of the url, which is the '/'
    let mut url = request.url().chars();
    url.next();
    let url = url.as_str();

    // Splits the url with the '#' and collects into Vec. Then assigns var id and password
    let parts = url.split("!").collect::<Vec<_>>();

    let (id, password) = (parts[0], parts[1]); 

    // TODO: Performance
    let mut encrypted_data = vec![];
    for element in paste_data {
        if element.id.as_str() == id {
            encrypted_data = element.content.clone();
        } 
    }

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