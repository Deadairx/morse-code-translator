use cli::cli;

mod cli;

fn main() {
    let matches = cli();

    let input = matches.get_one::<String>("INPUT").unwrap();

    let mut morse_code_output = String::new();

    for c in input.to_lowercase().chars() {
        morse_code_output.push_str(get_morse_for_char(&c));
    }

    println!("{}", morse_code_output);
}

fn get_morse_for_char(character: &char) -> &str {
    match character {
        'a' => "•-",
        'b' => "-•••",
        'c' => "-•-•",
        'd' => "-••",
        'e' => "•",
        'f' => "••-•",
        'g' => "--•",
        'h' => "••••",
        'i' => "••",
        'j' => "•---",
        'k' => "-•-",
        'l' => "•-••",
        'm' => "--",
        'n' => "-•",
        'o' => "---",
        'p' => "•--•",
        'q' => "--•-",
        'r' => "•-•",
        's' => "•••",
        't' => "-",
        'u' => "••-",
        'v' => "•••-",
        'w' => "•--",
        'x' => "-••-",
        'y' => "-•--",
        'z' => "--••",
        ' ' => " ",
        _ => "",
    }
}
