use tiny_http::{Server, Response};
use serde::Deserialize;
use std::{fs, cmp};
use nanoid::nanoid;

#[derive(Deserialize)]
struct Config {
    bind_address: String,
    paste_limit: usize,
}

impl Config {
    fn open_config() -> Config {
        let config = fs::read_to_string("config.toml").unwrap();
        return toml::from_str(&config).unwrap();
    }
}

fn main() {
    let config = Config::open_config();
    let mut paste_data: Vec<[String; 2]> = Vec::with_capacity(config.paste_limit);
    let server = Server::http(config.bind_address).unwrap();

    for mut request in server.incoming_requests() {

        let mut content = String::new();
        request
            .as_reader()
            .read_to_string(&mut content)
            .unwrap();
        
        match  paste_data.len().cmp(&config.paste_limit) {
            cmp::Ordering::Less => {
                let id = nanoid!(8);
                paste_data.push([String::from(&id), String::from(content)]);

                println!("{:#?}", paste_data);
                let response = Response::from_string(id);
                let _ = request.respond(response);
            },
            _ => todo!()
        }
    }
}