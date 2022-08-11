use clap::{Command, SubCommand, Arg};

fn main() {
    let matches = Command::new("kvs")
        .version("0.1.0")
        .author("Nirdesh Bhattarai nbhattarai.work@gmail.com")
        .about("A simple in-memory key value store in Rust")
        .arg_required_else_help(true)
        .subcommand(
    SubCommand::with_name("get")
                .about("Get value for a specified key")
                .arg(
                    Arg::with_name("key")
                        .takes_value(true)
                        .value_name("key")
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("set")
                        .about("Set key:value")
                        .arg(
                            Arg::with_name("key")
                                .takes_value(true)
                                .value_name("key")
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("value")
                                .takes_value(true)
                                .value_name("value")
                                .required(true)
                        )
                )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .action(clap::ArgAction::Version)
        )
        .get_matches();  
    
    if let Some(c) = matches.subcommand_matches("get") {
        eprintln!("unimplemented");
    }

    if let Some(c) = matches.subcommand_matches("set") {
        eprintln!("unimplemented");
    }
}