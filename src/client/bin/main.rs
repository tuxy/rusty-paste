use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    content: Option<String>,

    #[arg(short, long)]
    post: bool,
}

fn main() {
    let args = Args::parse();

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
                    println!("{}", 
                        val.into_string().expect("Could not format reply content"));
                },
                Err(err) => {
                    eprintln!("Could not make request: {}", err);
                    panic!();
                },
            };
        }
    }
}
