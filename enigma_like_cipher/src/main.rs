#[allow(unused_variables)]
#[allow(unused_assignments)]

// live version
// https://repl.it/repls/FractalSolidJavabytecode

mod enigma;

use std::io::{
    stdin,
    stdout,
    Write
};
use enigma::enigma::Enigma;

fn main() {
    let num_of_rotors: usize = 3;
    let mut rotor_positions: Vec<usize> = Vec::new();

    for i in 0..num_of_rotors {
        let rotor_position: usize; 
        'a: loop {
            let str_input = String::from(
                take_input(
                    format!("Rotor {} Position (0 to {}): ",
                            i + 1, Enigma::get_setting_len() - 1)
                            .as_str())
            );
            if str_input.parse::<usize>().is_ok()
                    && str_input.parse::<usize>().unwrap() < Enigma::get_setting_len() {
                rotor_position = str_input.parse::<usize>().unwrap();
                break 'a;
            } else {
                continue 'a;
            }
        };
        rotor_positions.push(rotor_position);
    }

    let mut enigma = Enigma::new(
        rotor_positions
    );

    let output_text = enigma.process_text(String::from(
        take_input("\nInput Text: ")
    ));

    println!("\nOutput Text: {}", output_text);

    println!("");
    String::from(
        take_input("Press any key to exit ...")
    );
}

// snippet taken from internet
fn take_input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);

    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    return s;
}
