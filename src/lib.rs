use std::collections::HashMap;

pub struct SquishyID {
    length: usize,
    characters_to_positions: HashMap<char, usize>,
    positions_to_characters: Vec<char>
}

impl SquishyID {

    fn new (key: &str) -> Result<Self, &str> {

        let positions_to_characters: Vec<char> = key.chars().collect();

        let length: usize = positions_to_characters.len();
        if length < 2 {
            return Err( "Key must contain at least 2 characters." );
        }

        let mut characters_to_positions: HashMap<char, usize> = HashMap::new();
        for (position, &character) in positions_to_characters.iter().enumerate() {
            if characters_to_positions.insert( character, position ).is_some() {
                return Err( "Key must contain unique characters." );
            }

        }

        Ok(
            Self {
                length,
                characters_to_positions,
                positions_to_characters
            }
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_too_short() {
        assert!(matches!(SquishyID::new(""), Err("Key must contain at least 2 characters.")));
        assert!(matches!(SquishyID::new("a"), Err("Key must contain at least 2 characters.")));
    }


    #[test]
    fn key_not_unique() {
        assert!(matches!(SquishyID::new("aa"), Err("Key must contain unique characters.")));
        assert!(matches!(SquishyID::new("aba"), Err("Key must contain unique characters.")));
    }

    #[test]
    fn valid_key() {
        assert!(SquishyID::new( "ab" ).is_ok());
    }


}
