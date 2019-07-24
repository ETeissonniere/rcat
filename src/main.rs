#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(rcat => 
        (version: "1.0")
        (author: "Eliott Teissonniere <eliott.teissonniere.org>")
        (about: "Tiny netcat like program in rust")
        (@arg PORT: -p --port +takes_value "Port to connect to or listen on")
        (@arg ADDR: -a --address +takes_value "Address to connect to or listen on")
        (@subcommand listen => 
            (about: "Listen for incoming connections on specified address and port")
        )
        (@subcommand connect =>
            (about: "Connect to specified address and port")
        )
    ).get_matches();
    
    if let Some(_) = matches.subcommand_matches("listen") {
        println!("Listening on {}:{}", matches.value_of("ADDR").unwrap_or("127.0.0.1"), matches.value_of("PORT").unwrap_or("4242"))
    } else if let Some(_) = matches.subcommand_matches("connect") {
        println!("Connecting to {}:{}", matches.value_of("ADDR").unwrap_or("127.0.0.1"), matches.value_of("PORT").unwrap_or("4242"))
    } else {
        println!("Unknown command")
    }
}
