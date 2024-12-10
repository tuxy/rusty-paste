use clap::Parser;
use arboard::{Clipboard, Error};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    url: String,

    #[arg(short, long)]
    content: Option<String>,

    #[arg(short, long)]
    post: bool,
}

fn main() {
    let args = Args::parse();

    // Initialises clipboard
    let mut clipboard_supported = false;
    let clipboard = match Clipboard::new() {
        Ok(clipboard) => Ok(clipboard),
        Err(err) => {
            eprintln!("Could not initialise clipboard: {err}");
            clipboard_supported = false;
            Err(Error::ClipboardNotSupported)
        }
    };

    match args.post {
        true => {

            let content = match args.content {
                Some(val) => val,
                None => panic!("No content provided")
            };
            
            let request = ureq::post(&args.url)
            .send_string(&content);

            match request {
                // Handle error for not being able to reach server
                Ok(val) => {
                    println!("{}", val.into_string().expect("Could not format reply content"));
                }
                Err(err) => {
                    eprintln!("{err}");
                    panic!();
                }
            }
        }
        false => {
            if args.content.is_some() {
                println!("Ignoring provided content");
            }
            let request = ureq::get(&args.url);

            match request.call() {
                // Handle error for not being able to reach server
                Ok(val) => {
                    let content = val.into_string().expect("Could not format reply content");
                    println!("{}", content);
                    if clipboard_supported {
                        // TODO
                        clipboard.unwrap().set_text(content).unwrap();
                    }
                },
                Err(err) => {
                    eprintln!("Could not make request: {}", err);
                    panic!();
                },
            };
        }
    }
}
