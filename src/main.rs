use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echoer")
        .version("0.1.0")
        .about("A simple command line tool to echo your input back to you.")
        .author("Highlander Paiva <[email protected]>")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("The text to echo")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print the trailing newline character"),
        )
        .get_matches();

    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();
    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    println!("{}{ending}", text.join(" "));
}
