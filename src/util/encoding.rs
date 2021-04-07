use std::{
    collections::HashMap
};

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub struct Encoding {
    utf8_to_ascii: HashMap<char, Vec<char>>
}

impl Encoding {
    pub fn new() -> Encoding {
        // create dictionaries
        let utf8_ascii = hashmap![
            'a' => vec![ 'ã', 'á', 'à' ],
            'e' => vec![ 'é' ],
            'i' => vec![ 'í', 'ì' ],
            'o' => vec![ 'õ', 'ó', 'ò' ],
            'u' => vec![ 'ú', 'ù' ]
        ];

        Encoding {
            utf8_to_ascii: utf8_ascii
        }
    }

    pub fn from_utf8_to_ascii<S: AsRef<str>>(&self, string: S) -> String {
        let mut ascii_representation = String::new();

        let s = String::from(string.as_ref());
        for char in s.chars() {
            let ascii = match self.convert_char_utf8_to_ascii(&char) {
                Some(c) => c,
                None => char
            };

            ascii_representation.push(ascii);
        }

        ascii_representation
    }

    pub fn convert_char_utf8_to_ascii(&self, c: &char) -> Option<char> {
        if c.is_ascii() {
            return Some(*c);
        }

        for (ascii, utf8_chars) in &self.utf8_to_ascii {
            for utf8 in utf8_chars {
                if c.eq(utf8) {
                    return Some(*ascii);
                }
            }
        }

        None
    }
}
