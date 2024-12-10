use tiny_http::Server;
use jasondb::Database;
use std::sync::Mutex;


mod config;
use config::Config;

mod methods;
use methods::*;

fn main() -> Result<(), std::io::Error> {
    // Opens configulation file
    let config = Config::open_config();

    let db: Database<PasteData> = match Database::new("database.jdb") {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Could not create/read database.jdb. Check permissions? Error: {err}");
            panic!();
        }
    };
    let mut db_mutex: Mutex<Database<PasteData>> = Mutex::new(db);

    // Stores the id of the paste, as well as the content.
    let server = match Server::http(&config.bind_address) {
        Ok(server) => server,
        Err(err) => {
            eprintln!("Could not bind to address: {}", err);
            panic!()
        }
    };

    // Creates a new thread and continuously loops through, checking the time limit of the pastes

    for mut request in server.incoming_requests() {

        // Checks for expired data. The data doesn't actually get deleted at the exact time, but instead
        // right now in order to improve performance (And multi tasking is hard). However, this may add 
        // A little extra overhead per request, and is not a good solution for long term. For example, 
        // if there were 0 requests, then it would not update, but if there were 10 simoultaneous requests,
        // Then the database would be scanned 10 times.
        match loop_and_check(&mut db_mutex) {
            Ok(_) => (), // Do nothing when no error is returned
            Err(_) => eprintln!("Could not read or write to database.jdb. Check permissions?")
        };

        // Checks URL and reads post content
        let mut content = String::new();
        request
            .as_reader()
            .read_to_string(&mut content)
            .expect("Could not read request"); // When the request cannot be read

        match request.url() {
            // Handle case for paste POST and URL creation
            "/" => {
                post_paste(request, &mut db_mutex, config.clone(), content, config.ident_length);
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