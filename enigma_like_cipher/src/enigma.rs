#[allow(unused_variables)]
#[allow(unused_assignments)]

pub mod enigma {

    pub struct Enigma {
        setting: [(char, char); 94],
        num_of_rotors: usize,
        rotors: Vec<[(char, char); 94]>,
        rotor_positions: Vec<usize>,
    }

    impl Enigma {

        pub fn new(rotor_positions: Vec<usize>) -> Enigma {
            let mut enigma = Enigma {
                setting: [
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
                ],
                num_of_rotors: rotor_positions.len(),
                rotors: Vec::new(),
                rotor_positions: rotor_positions.clone()
            };
            for i in 0..enigma.setting.len() {
                let mut temp = enigma.setting[enigma.setting.len() - 1 - i];
                enigma.setting[i].1 = temp.0.clone();
                temp.1 = enigma.setting[i].0.clone();
            }
            for i in 0..(enigma.num_of_rotors + 1) {
                enigma.rotors.push(enigma.setting);
            }
            for i in 0..(enigma.rotors.len() - 1) {
                Self::rotate_rotors(
                    &enigma.setting,
                    &mut enigma.rotors[i],
                    enigma.rotor_positions[i].clone()
                );
            }
            enigma
        }

        pub fn rotate_rotors(
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

        pub fn process_text(&mut self, input_text: String) -> String {
            let input_text_len = input_text.len();
            let mut input_text_arr = vec!['a'; input_text_len];
            let mut output_text = String::from("");

            for (index, value) in input_text.chars().enumerate() {
                input_text_arr[index] = value;
            }

            for i in 0..input_text_arr.len() {
                for j in 0..self.rotor_positions.len() {
                    if j == 0 {
                        self.rotor_positions[j] += 1;
                    }
                    if self.rotor_positions[j] >= self.setting.len() {
                        self.rotor_positions[j] = 0;
                        if j + 1 < self.rotor_positions.len() {
                            self.rotor_positions[j + 1] += 1;
                        }
                    }
                }

                for j in 0..(self.rotors.len() - 1) {
                    Self::rotate_rotors(
                        &self.setting,
                        &mut self.rotors[j],
                        self.rotor_positions[j].clone()
                    );
                }

                let mut current_char = input_text_arr[i];

                for j in 0..self.setting.len() {
                    if current_char == self.setting[j].0 {
                        current_char = self.setting[j].1.clone();
                        break;
                    }
                }
                for j in 0..self.rotors.len() {
                    for k in 0..self.rotors[j].len() {
                        if current_char == self.rotors[j][k].0 {
                            current_char = self.rotors[j][k].1.clone();
                            break;
                        }
                    }
                }
                for j in (0..(self.rotors.len() - 1)).rev() {
                    for k in 0..self.rotors[j].len() {
                        if current_char == self.rotors[j][k].1 {
                            current_char = self.rotors[j][k].0.clone();
                            break;
                        }
                    }
                }
                for j in 0..self.setting.len() {
                    if current_char == self.setting[j].1 {
                        current_char = self.setting[j].0.clone();
                        break;
                    }
                }

                output_text.push(current_char);
            }

            output_text
        }

        pub fn get_setting_len() -> usize {
            94
        }
        
    }
}