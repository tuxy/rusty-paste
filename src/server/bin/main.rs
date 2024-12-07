use tiny_http::{Server, Response};
use nanoid::nanoid;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

mod config;

fn main() {
    // Opens configulation file
    let config = config::Config::open_config();
    let bind_address = config.bind_address;

    // Stores the id of the paste, as well as the content.
    // TODO: aes-gcm symetric encryption
    let mut paste_data: Vec<[String; 2]> = Vec::new();
    let server = Server::http(&bind_address).unwrap();

    for mut request in server.incoming_requests() {

        // Checks URL and reads post content
        let mut content = String::new();
        request
            .as_reader()
            .read_to_string(&mut content)
            .unwrap();

        match request.url() {
            // Handle case for paste POST and URL creation
            "/" => {
                // Set up encryption
                let password = nanoid!(8);
                let key = &generate_key(&password);
                let cipher = Aes256Gcm::new(&key);
                let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

                let ciphertext = cipher.encrypt(&nonce, content.as_bytes().as_ref()).unwrap();
                
                let id = nanoid!(8);
                // Places id of the paste as well as the paste data into the array
                paste_data.push([String::from(&id), String::from_utf8(ciphertext).unwrap()]);

                println!("{:#?}", paste_data);
                let response = 
                    Response::from_string(get_url(id, &bind_address, password));
                let _ = request.respond(response);
            }
            // Handle case for paste GET and decryption
            _ => {
                // TODO: Remove forst character (Which is the '/'), and split the request URL
                // After splitting into id and password, iterate through the array to find ID's content
                // Decrypt content, and send back to client. OK ignore all previous ideas and realise
                // that the cipher and nonce also needs to be stored.
            }
        }
    }
}

fn generate_key(password: &String) -> Key<Aes256Gcm> {
    let password_bytes = password.as_bytes();
    let key: &Key<Aes256Gcm> = password_bytes.into();
    *key
}

fn get_url(id: String, url: &String, password: String) -> String {
    format!("https://{url}/{id}{password}")
}