use std::collections::HashMap;

pub struct SquishyID {
    length: usize,
    characters_to_positions: HashMap<char, usize>,
    positions_to_characters: Vec<char>
}

impl SquishyID {

    pub fn new ( key: &str ) -> Result<Self, &str> {

        let positions_to_characters: Vec<char> = key.chars( ).collect( );

        let length: usize = positions_to_characters.len( );
        if length < 2 {
            return Err( "Key must contain at least 2 characters." );
        }

        let mut characters_to_positions: HashMap<char, usize> = HashMap::new( );
        for ( position, &character ) in positions_to_characters.iter( ).enumerate( ) {
            if characters_to_positions.insert( character, position ).is_some( ) {
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

    pub fn encode ( &self, mut decoded: u64 ) -> String {

        let mut encoded: Vec<char> = Vec::new( );

        loop {
            let position: u64 = decoded % ( self.length as u64 );
            encoded.push( self.positions_to_characters[ position as usize ] );
            decoded /= self.length as u64;

            if decoded == 0 {
                break;
            }
        }

        encoded.iter( ).rev( ).collect( )
    }

    pub fn decode ( &self, encoded: &str ) -> Result<u64, &str> {

        let mut decoded: u64 = 0;

        for ( position, character ) in encoded.chars().rev().enumerate() {
            match self.characters_to_positions.get( &character ) {
                None => return Err( "Encoded value contains character not present in key." ),
                Some( &factor ) => { decoded += factor as u64 * (self.length as u64).pow( position as u32 ) }
            }
        }

        Ok( decoded )
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_too_short ( ) {
        assert!( matches!( SquishyID::new( "" ), Err( "Key must contain at least 2 characters." ) ) );
        assert!( matches!( SquishyID::new( "a" ), Err( "Key must contain at least 2 characters." ) ) );
    }

    #[test]
    fn key_not_unique ( ) {
        assert!( matches!( SquishyID::new( "aa" ), Err( "Key must contain unique characters." ) ) );
        assert!( matches!( SquishyID::new( "aba" ), Err( "Key must contain unique characters." ) ) );
    }

    #[test]
    fn key_valid ( ) {
        assert!( SquishyID::new( "ab" ).is_ok( ) );
    }

    #[test]
    fn transcode_0_value ( ) {
        let s = SquishyID::new( "ab" ).unwrap( );
        assert_eq!( s.encode( 0 ), "a" );
        assert_eq!( s.decode( "a" ).unwrap(), 0 );
    }

    #[test]
    fn transcode_u64_value ( ) {
        let s = SquishyID::new( "FujSBZHkPMincNQr6pq0mgxw2tXAsyb8DWV534EC1RUIlYoGOJhed9afKT7vzL" ).unwrap( );
        assert_eq!( s.encode( u64::MAX ), "gzUp3uHipVr" );
        assert_eq!( s.decode( "gzUp3uHipVr" ).unwrap(), u64::MAX );
    }

    #[test]
    fn transcode_non_ascii ( ) {
        let s = SquishyID::new( "äą" ).unwrap( );
        assert_eq!( s.encode( 8 ), "ąäää" );
        assert_eq!( s.decode( "ąäää" ).unwrap(), 8 );

        let s = SquishyID::new( "😀😁😂😃😄😅😆😇😈😉😊😋😌😍😎😏😐😑😒😓😔😕😖😗😘😙😚😛😜😝😞😟😠😡😢😣😤😥😦😧😨😩😪😫😬😭😮😯😰😱😲😳😴😵😶😷" ).unwrap( );
        assert_eq!( s.encode( 48888851145 ), "😁😠😫😈😵😇😁" );
        assert_eq!( s.decode( "😁😠😫😈😵😇😁" ).unwrap(), 48888851145 );
    }

}
