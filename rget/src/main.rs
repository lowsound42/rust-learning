extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Omar Khan <o.khandxb@gmail.com>")
        .about("wget clone written in Rust")
        .arg(
            Arg::with_name("URL")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("url to download"),
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    println!("{}", url);
}
