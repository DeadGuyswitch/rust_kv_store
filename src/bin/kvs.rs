use clap::{Command, Arg};

fn main() {
    let _matches = Command::new("kvs")
        .version("0.1.0")
        .author("Nirdesh Bhattarai nbhattarai.work@gmail.com")
        .about("A simple in-memory key value store in Rust")
        .arg_required_else_help(true)
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .action(clap::ArgAction::Version)
        )
        // .arg(
        //     Arg::new("get")
        //         .short('g')
        //         .long("get")
        //         .action(action)

        // )
        .get_matches();
}