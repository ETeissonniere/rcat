extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("rcat")
        .version("1.0")
        .author("Eliott Teissonniere <eliott.teissonniere.org>")
        .about("Tiny netcat like program in rust")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .default_value("4242")
            .help("Specify port to use when listening or connecting")
            .value_name("PORT")
            .takes_value(true))
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .default_value("127.0.0.1")
            .help("Specify IP address to use when listening or connecting")
            .value_name("ADDR")
            .takes_value(true))
        .subcommand(SubCommand::with_name("listen")
            .about("start listening server"))
        .subcommand(SubCommand::with_name("connect")
            .about("connect to server"))
        .get_matches();
}
