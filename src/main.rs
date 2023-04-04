use cli::cli;

mod cli;

fn main() {
    let matches = cli();

    let input = matches.get_one::<String>("INPUT").unwrap();

    let morse_code_output = input
        .to_lowercase()
        .chars()
        .map(|c| get_morse_for_char(&c).to_string())
        .collect::<Vec<_>>()
        .join(" ");

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
        '1' => "•----",
        '2' => "••---",
        '3' => "•••--",
        '4' => "••••-",
        '5' => "•••••",
        '6' => "-••••",
        '7' => "--•••",
        '8' => "---••",
        '9' => "----•",
        '0' => "-----",
        ' ' => " ",
        _ => "",
    }
}
