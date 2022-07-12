use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};
use colored::*;

mod detect_server;

fn main() -> Result<()> {
    let matches = App::new("h4")
        .version("0.1.0")
        .author("seestem")
        .about("h4cking")
        .subcommand(
            SubCommand::with_name("sniff").about("Sniff stuff").arg(
                Arg::with_name("server")
                    .long("server")
                    .short("s")
                    .value_name("ADDRESS")
                    .takes_value(true)
                    .help("Detect the server being used to serve the address (eg Apache)"),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("sniff") {
        if matches.is_present("server") {
            let address = matches
                .value_of("server")
                .expect("Server address to sniff is not provided");

            let is_apache = detect_server::is_apache(address)?;

            if is_apache {
                println!(
                    "{}",
                    "The server is propably running Apache!".green().bold()
                );
            } else {
                println!("{}", "Unknown server!".red().bold());
            }
        }
    }

    Ok(())
}
