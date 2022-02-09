// use ansi_term::Style;
// use clap::Parser;
// use reqwest;
// use serde_json::json;
// use std::error::Error;
// use ansi_term::Colour;

// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     #[clap(short, long)]
//     name: String,

//     #[clap(short, long, default_value_t = 1)]
//     count: u8,

//     #[clap(long = "json")]
//     json: bool,
// }

// fn main() {
//     let args = Args::parse();

//     for _ in 0..args.count {
//         println!("Hello {}!", args.name)
//     }
//     println!("{} and this is not",
//              Style::new().bold().paint("This is Bold"));
//     println!("{}, {} and {}",
//              Colour::Yellow.paint("This is colored"),
//              Style::new().bold().paint("this is bold"),
//              Colour::Yellow.bold().paint("this is bold and colored"));
//     println!("This is {} in color, {} in color and {} in color",
//              Colour::Red.paint("red"),
//              Colour::Blue.paint("blue"),
//              Colour::Green.paint("green"));
//     if args.json {
//         println!(
//             "{}",
//             json!({
//                 "type": "message",
//                 "content": "Hello world",
//             })
//         );
//     } else {
//         println!("Hello world");
//     }
// }

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
