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
            if args.content == None {
                panic!("No content provided")
            }

            let request = ureq::post(&args.url)
                .send_string(&args.content.unwrap())
                .unwrap();

            println!("{}", request.into_string().unwrap())
            // Yeah that's really about it
        }
        false => {
            if args.content != None {
                println!("Ignoring provided content");
            }
            let request = ureq::get(&args.url);
            // Please fix this godawful thing
            println!("{}", request.call().unwrap().into_string().unwrap())
        }
    }
}
