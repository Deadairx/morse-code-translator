use clap::{ArgMatches, Command, arg};

pub fn cli() -> ArgMatches {
    Command::new("morse-code-translator")
        .args(&[
            arg!(<INPUT> "Text to be translated to morse code")
        ])
        .get_matches()
}
