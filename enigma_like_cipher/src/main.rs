#[allow(unused_variables)]
#[allow(unused_assignments)]

// live version
// https://repl.it/repls/FractalSolidJavabytecode

use std::io::{
    stdin,
    stdout,
    Write
};

fn main() {
    let mut setting = [
        ('a', 'a'),
        ('b', 'b'),
        ('c', 'c'),
        ('d', 'd'),
        ('e', 'e'),
        ('f', 'f'),
        ('g', 'g'),
        ('h', 'h'),
        ('i', 'i'),
        ('j', 'j'),
        ('k', 'k'),
        ('l', 'l'),
        ('m', 'm'),
        ('n', 'n'),
        ('o', 'o'),
        ('p', 'p'),
        ('q', 'q'),
        ('r', 'r'),
        ('s', 's'),
        ('t', 't'),
        ('u', 'u'),
        ('v', 'v'),
        ('w', 'w'),
        ('x', 'x'),
        ('y', 'y'),
        ('z', 'z'),
        ('A', 'A'),
        ('B', 'B'),
        ('C', 'C'),
        ('D', 'D'),
        ('E', 'E'),
        ('F', 'F'),
        ('G', 'G'),
        ('H', 'H'),
        ('I', 'I'),
        ('J', 'J'),
        ('K', 'K'),
        ('L', 'L'),
        ('M', 'M'),
        ('N', 'N'),
        ('O', 'O'),
        ('P', 'P'),
        ('Q', 'Q'),
        ('R', 'R'),
        ('S', 'S'),
        ('T', 'T'),
        ('U', 'U'),
        ('V', 'V'),
        ('W', 'W'),
        ('X', 'X'),
        ('Y', 'Y'),
        ('Z', 'Z'),
        ('1', '1'),
        ('2', '2'),
        ('3', '3'),
        ('4', '4'),
        ('5', '5'),
        ('6', '6'),
        ('7', '7'),
        ('8', '8'),
        ('9', '9'),
        ('~', '~'),
        ('`', '`'),
        ('!', '!'),
        ('@', '@'),
        ('#', '#'),
        ('$', '$'),
        ('%', '%'),
        ('^', '^'),
        ('&', '&'),
        ('(', '('),
        (')', ')'),
        ('-', '-'),
        ('_', '_'),
        ('+', '+'),
        ('=', '='),
        ('*', '*'),
        ('[', '['),
        (']', ']'),
        ('{', '{'),
        ('}', '}'),
        (';', ';'),
        (':', ':'),
        (',', ','),
        ('.', '.'),
        ('<', '<'),
        ('>', '>'),
        ('/', '/'),
        ('?', '?'),
        (' ', ' '),
        ('"', '"'),
        ('|', '|'),
        ('\'', '\''),
        ('\\', '\\'),
    ];

    let num_of_rotors: usize = 3;

    for i in 0..setting.len() {
        let mut temp = setting[setting.len() - 1 - i];
        setting[i].1 = temp.0.clone();
        temp.1 = setting[i].0.clone();
    }

    let mut rotors: Vec<[(char, char); 94]> = Vec::new();
    let mut rotor_positions: Vec<usize> = Vec::new();

    for i in 0..num_of_rotors+1 {
        rotors.push(setting);
        rotor_positions.push(i as usize);
    }

    for i in 0..num_of_rotors {
        let rotor_position: usize; 
        'a: loop {
            let str_input = String::from(
                take_input(format!("Rotor {} Position (0 to {}): ", i + 1, setting.len() - 1).as_str())
            );
            if str_input.parse::<usize>().is_ok()
                    && str_input.parse::<usize>().unwrap() < setting.len() {
                rotor_position = str_input.parse::<usize>().unwrap();
                break 'a;
            } else {
                continue 'a;
            }
        };
        rotor_positions[i] = rotor_position as usize;
    }

    for j in 0..rotors.len()-1 {
        rotate_rotors(&setting, &mut rotors[j], rotor_positions[j].clone());
    }

    let input_text = String::from(
        take_input("\nInput Text: ")
    );
    let input_text_len = input_text.len();
    let mut input_text_arr = vec!['a'; input_text_len];

    for (index, value) in input_text.chars().enumerate() {
        input_text_arr[index] = value;
    }

    let mut output_text = String::from("");

    for i in 0..input_text_arr.len() {
        for j in 0..rotor_positions.len() {
            if j == 0 {
                rotor_positions[j] += 1;
            }
            if rotor_positions[j] >= setting.len() {
                rotor_positions[j] = 0;
                if j + 1 < rotor_positions.len() {
                    rotor_positions[j + 1] += 1;
                }
            }
        }

        for j in 0..rotors.len()-1 {
            rotate_rotors(&setting, &mut rotors[j], rotor_positions[j].clone());
        }

        let mut current_char = input_text_arr[i];

        for j in 0..setting.len() {
            if current_char == setting[j].0 {
                current_char = setting[j].1.clone();
                break;
            }
        }
        for j in 0..rotors.len() {
            for k in 0..rotors[j].len() {
                if current_char == rotors[j][k].0 {
                    current_char = rotors[j][k].1.clone();
                    break;
                }
            }
        }
        for j in (0..(rotors.len() - 1)).rev() {
            for k in 0..rotors[j].len() {
                if current_char == rotors[j][k].1 {
                    current_char = rotors[j][k].0.clone();
                    break;
                }
            }
        }
        for j in 0..setting.len() {
            if current_char == setting[j].1 {
                current_char = setting[j].0.clone();
                break;
            }
        }

        output_text.push(current_char);
    }
    
    println!("\nOutput Text: {}", output_text);
}

fn rotate_rotors(
    setting: &[(char, char)],
    rotor: &mut [(char, char)],
    index: usize
) {
    let mut index = index;
    for index_a in 0..rotor.len() {
        if index >= setting.len() {
            index = 0;
        }
        rotor[index_a].1 = setting[index].1.clone();
        index += 1;
    }
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
