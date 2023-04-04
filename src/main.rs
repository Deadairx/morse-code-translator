use cli::cli;

mod cli;

fn main() {
    let matches = cli();

    let input = matches.get_one::<String>("INPUT").unwrap();

    let mut morse_code_output = String::new();

    for c in input.chars() {
        morse_code_output.push_str(get_morse_for_char(&c));
    }

    println!("{}", morse_code_output);
}

fn get_morse_for_char(character: &char) -> &str {
    match character {
        'a' => "•-",
        _ => "",
    }
}
