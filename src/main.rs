mod cli;
mod models;
use cli::Client;

use clap::{App, AppSettings, Arg};

#[tokio::main]
async fn main() {
    let matches = App::new("xkcd_query")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .about("show xkcd titles")
        .author("fp")
        .arg(
            Arg::new("xkcd_number")
                .short('n')
                .long("number")
                .value_name("XKCD NUMBER")
                .about("Sets the xkcd to fetch")
                .takes_value(true),
        )
        .get_matches();

    let xkcd_number = matches.value_of("xkcd_number").expect("number not valid");
    let client = Client::new();
    let comic = client.fetch(&xkcd_number, false).await;

    match comic {
        Ok(comic) => println!("{}", comic),
        Err(e) => panic!(e),
    }
}
