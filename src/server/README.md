# rusty-paste server

This is where most of the encryption and storage occurs, with the only content stored being the encrypted data of the paste, and its id to be able to pick it out later. The encryption key is stored in the URL itself, with a configurable length.

The ID and password can either be:
 - Long, secure but harder to type out
Or: 
 - Shorter, easier to remember but harder to type out

 # Usage
 Just run the server with a `config.toml` file seen the the root of the project:
 `cargo run --bin server`

 or after compilation:

 `./server`