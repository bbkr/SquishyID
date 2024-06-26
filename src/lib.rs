//! Shortens and obfuscates your IDs at the same time.
//!
//! Useful for:
//! - Hiding real database IDs in URLs or REST APIs.
//! - Saving space where it is limited, like in SMS or Push messages.
//!
//! # Examples
//! ```
//! use squishyid::SquishyID;
//!
//! let s = SquishyID::new(
//!     "2BjLhRduC6Tb8Q5cEk9oxnFaWUDpOlGAgwYzNre7tI4yqPvXm0KSV1fJs3ZiHM"
//! ).unwrap();
//!
//! let encoded: String = s.encode(48888851145);
//! assert_eq!(encoded, "1FN7Ab");
//!
//! let decoded: u64 = s.decode("1FN7Ab").unwrap();
//! assert_eq!(decoded, 48888851145);
//! ```
//!
//! Check out [SquishyID] for detailed methods description.
//!
//! # Other implementations
//! - [Raku](https://github.com/bbkr/TinyID)
//! - [PHP](https://github.com/krowinski/tinyID)
//! - [Perl](http://search.cpan.org/~bbkr/Integer-Tiny-0.3/lib/Integer/Tiny.pm)

use std::collections::HashMap;

pub struct SquishyID {
    length: usize,
    characters_to_positions: HashMap<char, usize>,
    positions_to_characters: Vec<char>,
}

impl SquishyID {

    /// Constructs new instance using given key.
    ///
    /// - It must consist of at least two unique unicode characters.
    /// - The **longer the key** - the **shorter encoded ID** will be.
    /// - Encoded ID will be **made exclusively out of characters from the key**.
    ///
    /// Choose your key characters wisely, for example:
    /// - For SMS messages generate key from `a-z,A-Z,0-9` range.
    ///   You will get excellent shortening like `1234567890` -> `380FQs`.
    /// - For NTFS file names generate key from `a-z` range.
    ///   You will get good shortening and avoid case insensitivity collisions, like `1234567890` -> `iszbmfx`.
    /// - When trolling generate key from Emojis.
    ///   So `1234567890` will be represented as `😣😄😹😧😋😳`.
    ///
    /// # Errors
    /// - `Key must contain at least 2 characters.`
    /// - `Key must contain unique characters.`

    pub fn new(key: &str) -> Result<Self, &str> {
        let positions_to_characters: Vec<char> = key.chars().collect();

        let length: usize = positions_to_characters.len();
        if length < 2 {
            return Err("Key must contain at least 2 characters.");
        }

        let mut characters_to_positions: HashMap<char, usize> = HashMap::new();
        for (position, &character) in positions_to_characters.iter().enumerate() {
            if characters_to_positions
                .insert(character, position)
                .is_some()
            {
                return Err("Key must contain unique characters.");
            }
        }

        Ok(Self {
            length,
            characters_to_positions,
            positions_to_characters,
        })
    }

    /// Encodes number using characters from the key.
    ///
    /// Note that **this should not be considered a strong encryption**.
    /// It does not contain consistency checks.
    /// And key is easy to reverse engineer with small amount of encoded/decoded samples given.
    /// Treat it as really, really fast obfuscation only.

    pub fn encode(&self, mut decoded: u64) -> String {
        let mut encoded: Vec<char> = Vec::new();

        loop {
            let position: u64 = decoded % (self.length as u64);
            encoded.push(self.positions_to_characters[position as usize]);
            decoded /= self.length as u64;

            if decoded == 0 {
                break;
            }
        }

        encoded.iter().rev().collect()
    }

    /// Decodes string using characters from the key.
    ///
    /// # Errors
    /// - `Encoded value must contain at least 1 character.`
    /// - `Encoded value contains character not present in key.`
    /// - `Encoded value too big to decode.` - when it would cause `u64` overflow.

    pub fn decode(&self, encoded: &str) -> Result<u64, &str> {
        if encoded.len() == 0 {
            return Err("Encoded value must contain at least 1 character.");
        }

        let mut decoded: u64 = 0;

        for (position, character) in encoded.chars().rev().enumerate() {
            let factor: u64 = match self.characters_to_positions.get(&character) {
                None => return Err("Encoded value contains character not present in key."),
                Some(&factor) => factor as u64,
            };

            match (self.length as u64)
                .checked_pow(position as u32)
                .and_then(|a| a.checked_mul(factor))
                .and_then(|a| a.checked_add(decoded))
            {
                None => return Err("Encoded value too big to decode."),
                Some(bigger_decoded) => decoded = bigger_decoded,
            }
        }

        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_too_short() {
        assert!(matches!(
            SquishyID::new(""),
            Err("Key must contain at least 2 characters.")
        ));
        assert!(matches!(
            SquishyID::new("a"),
            Err("Key must contain at least 2 characters.")
        ));
    }

    #[test]
    fn key_not_unique() {
        assert!(matches!(
            SquishyID::new("aa"),
            Err("Key must contain unique characters.")
        ));
        assert!(matches!(
            SquishyID::new("aba"),
            Err("Key must contain unique characters.")
        ));
    }

    #[test]
    fn key_valid() {
        assert!(SquishyID::new("ab").is_ok());
    }

    #[test]
    fn transcode_0_value() {
        let s = SquishyID::new("ab").unwrap();
        assert_eq!(s.encode(0), "a");
        assert_eq!(s.decode("a").unwrap(), 0);
    }

    #[test]
    fn transcode_u64_value() {
        let s = SquishyID::new("FujSBZHkPMincNQr6pq0mgxw2tXAsyb8DWV534EC1RUIlYoGOJhed9afKT7vzL")
            .unwrap();
        assert_eq!(s.encode(u64::MAX), "gzUp3uHipVr");
        assert_eq!(s.decode("gzUp3uHipVr").unwrap(), u64::MAX);
    }

    #[test]
    fn transcode_non_ascii() {
        let s = SquishyID::new("äą").unwrap();
        assert_eq!(s.encode(8), "ąäää");
        assert_eq!(s.decode("ąäää").unwrap(), 8);

        let s = SquishyID::new("😀😁😂😃😄😅😆😇😈😉😊😋😌😍😎😏😐😑😒😓😔😕😖😗😘😙😚😛😜😝😞😟😠😡😢😣😤😥😦😧😨😩😪😫😬😭😮😯😰😱😲😳😴😵😶😷").unwrap();
        assert_eq!(s.encode(48888851145), "😁😠😫😈😵😇😁");
        assert_eq!(s.decode("😁😠😫😈😵😇😁").unwrap(), 48888851145);
    }

    #[test]
    fn decode_empty_string() {
        let s = SquishyID::new("ab").unwrap();
        assert!(matches!(
            s.decode(""),
            Err("Encoded value must contain at least 1 character.")
        ));
    }

    #[test]
    fn decode_character_not_in_key() {
        let s = SquishyID::new("ab").unwrap();
        assert!(matches!(
            s.decode("x"),
            Err("Encoded value contains character not present in key.")
        ));
    }

    #[test]
    fn decode_overflow() {
        let s = SquishyID::new("0123456789ABCDEF").unwrap();
        assert!(matches!(
            s.decode("10000000000000000"),
            Err("Encoded value too big to decode.")
        ));
    }
}
