fn main() {
    use std::{thread, time};

    println!("Hello Player 1!");
    let mut word = get_input("Provide word searched for:");
    word.make_ascii_uppercase();
    println!("Word search for is {}.", word);

    thread::sleep(time::Duration::from_secs(3));

    print!("\x1B[2J\x1B[1;1H");
    println!("Hello Player 2!");

    let mut guessed_word = String::new();
    for _ in word.chars() {
        guessed_word.push_str("_");
    }

    let mut wrong_chars: Vec<char> = Vec::new();

    loop {
        println!("Current word: {}", guessed_word);
        println!("Wrong letters guessed: {:?}", wrong_chars);

        let mut input = get_input("Provide one character:");
        input.make_ascii_uppercase();
        
        if input.len() > 1 {
            println!("Please provide only one character!");
            continue;
        }

        if input.len() < 1 {
            println!("Please provde a character!");
            continue;
        }

        let letter = &input[0..1];
        let character = letter.chars().next().expect("Should not occur.");

        let mut found_letter = false;

        for chr in word.char_indices() {
            if chr.1 == character {
                found_letter = true;
                guessed_word.replace_range(chr.0..=chr.0, letter);
            }
        }

        if !found_letter {
            wrong_chars.insert(0, character);
        }

        if !guessed_word.contains("_") {
            println!("You won!");
            break;
        }

        if wrong_chars.len() > 8 {
            println!("You lost!");
            break;
        }
    }
}

fn get_input(s: &str) -> String {
    use std::io::stdin;

    println!("{}", s);
    let mut word = String::new();
    stdin().read_line(&mut word).expect("Something went wrong.");

    if let Some('\n')=word.chars().next_back() {
        word.pop();
    }
    if let Some('\r')=word.chars().next_back() {
        word.pop();
    }

    word
}
