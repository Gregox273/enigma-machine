use std::io;

// Letters A-Z
const MIN_ASCII_CHAR: u8 = 65;
const MAX_ASCII_CHAR: u8 = 90;

const EXIT_STR: &str = "exit";

// Convert single character from ascii to rotor position notation
fn char_to_rotor_position(c: char) -> u8 {
    c as u8 - MIN_ASCII_CHAR
}

// Convert from ascii to zero indexed rotor position notation
fn ascii_to_rotor_position(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    let mut rtn_vec: Vec<u8> = Vec::with_capacity(bytes.len());

    for &item in bytes.iter() {
        rtn_vec.push(item - MIN_ASCII_CHAR);
    }

    rtn_vec
}

fn main() {
    let rotors: Vec<enigma::Rotor> = vec![
        enigma::Rotor::new(char_to_rotor_position('A'),
            ascii_to_rotor_position("V"),
            char_to_rotor_position('A'),
            enigma::Wiring::new(ascii_to_rotor_position("BDFHJLCPRTXVZNYEIWGAKMUSQO"))),
        enigma::Rotor::new(char_to_rotor_position('A'),
            ascii_to_rotor_position("E"),
            char_to_rotor_position('A'),
            enigma::Wiring::new(ascii_to_rotor_position("AJDKSIRUXBLHWTMCQGZNPYFVOE"))),
        enigma::Rotor::new(char_to_rotor_position('A'),
            ascii_to_rotor_position("Q"),
            char_to_rotor_position('A'),
            enigma::Wiring::new(ascii_to_rotor_position("EKMFLGDQVZNTOWYHXUSPAIBRCJ"))),
    ];

    let etw_wiring: enigma::Wiring =
        enigma::Wiring::new(ascii_to_rotor_position("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));

    // UKW-B
    let reflector_wiring: enigma::Wiring =
        enigma::Wiring::new(ascii_to_rotor_position("YRUHQSLDPXNGOKMIEBFZCWVJAT"));

    let stecker_wiring: enigma::Wiring =
        enigma::Wiring::new(ascii_to_rotor_position("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));

    let mut enigma_1: enigma::Enigma =
        enigma::Enigma::new(rotors, etw_wiring, reflector_wiring, stecker_wiring);

    loop {
        println!("Enter input text (\"exit\" to quit):");

        let mut input_string = String::new();

        // Read input
        io::stdin()
            .read_line(&mut input_string)
            .expect("ERROR: Failed to read input");
        let trimmed_input_str = input_string.trim();

        // Quit program
        if trimmed_input_str == EXIT_STR {
            break;
        }

        // Process message
        input_string = trimmed_input_str.to_string().to_uppercase();
        input_string = input_string.replace(" ", "");  // Remove spaces

        // Check for invalid characters
        let message_bytes = input_string.as_bytes();
        let mut message_valid = true;

        for (i, &character) in message_bytes.iter().enumerate() {
            if character < MIN_ASCII_CHAR || character > MAX_ASCII_CHAR {
                // Invalid input
                println!("ERROR: Invalid character \"{}\" at position {}", character as char, i);
                message_valid = false;
                break;
            }
        }

        if message_valid {
            // ASCII output
            let mut output: Vec<u8> = Vec::with_capacity(message_bytes.len());

            for character in message_bytes.iter() {
                output.push(enigma_1.translate(character - MIN_ASCII_CHAR) + MIN_ASCII_CHAR);
            }

            println!("{}", std::str::from_utf8(&output[..]).unwrap());
        }

        println!("");
    }
}
