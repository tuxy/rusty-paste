use tiny_http::{Server, Response};
use nanoid::nanoid;

mod config;

struct PasteData {
    id: String,
    content: Vec<u8>,
}

fn main() {
    // Opens configulation file
    let config = config::Config::open_config();
    let bind_address = config.bind_address;

    // Stores the id of the paste, as well as the content.
    // TODO: aes-gcm symetric encryption
    let mut paste_data: Vec<PasteData> = Vec::new();
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

                let ciphertext = 
                    simplestcrypt::encrypt_and_serialize(password.as_bytes(), content.as_bytes())
                    .unwrap();

                let id = nanoid!(8);
                // Places id of the paste as well as the paste data into the array
                paste_data.push(PasteData {
                    id: id.clone(),
                    content: ciphertext,
                });

                let response = 
                    Response::from_string(get_url(id, &bind_address, password));
                let _ = request.respond(response);
            }
            // Handle case for paste GET and decryption
            _ => {
                // Removes the first character of the url, which is the '/' TODO: Performance???
                let mut url = request.url().chars();
                url.next();
                let url = url.as_str();

                // Splits the url with the '#' and collects into Vec. Then assigns var id and password
                let parts = url.split("!").collect::<Vec<_>>();

                let (id, password) = (parts[0], parts[1]); 

                let mut encrypted_data = vec![];
                for element in &paste_data {
                    if element.id.as_str() == id {
                        encrypted_data = element.content.clone();
                    } 
                }

                let decrypted_data = simplestcrypt::deserialize_and_decrypt(password.as_bytes(), &encrypted_data).unwrap();
                let response = 
                    Response::from_string(String::from_utf8(decrypted_data).unwrap());
                let _ = request.respond(response);
            }
        }
    }
}

fn get_url(id: String, url: &String, password: String) -> String {
    format!("https://{url}/{id}!{password}")
}   