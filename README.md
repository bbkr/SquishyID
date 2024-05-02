# SquishyID

Shorten and obfuscate IDs in [Rust](https://www.rust-lang.org/) language.

Useful for:

* Hiding real database IDs in URLs or REST APIs.
* Saving space where it is limited, like in SMS or Push messages.

## Example

```rust
    use squishyid::SquishyID;

    let s = SquishyID::new(
        "2BjLhRduC6Tb8Q5cEk9oxnFaWUDpOlGAgwYzNre7tI4yqPvXm0KSV1fJs3ZiHM"
    ).unwrap();

    let encoded: String = s.encode(48888851145);
    assert_eq!(encoded, "1FN7Ab");

    let decoded: u64 = s.decode("1FN7Ab").unwrap();
    assert_eq!(decoded, 48888851145);
```

## Methods

### new(key: &str) -> Result<Self, &str>

Constructs new instance using given key.

* It must consist of at least two unique unicode characters.
* The **longer the key** - the **shorter encoded ID** will be.
* Encoded ID will be **made exclusively out of characters from the key**.

Choose your key characters wisely, for example:

* For SMS messages generate key from `a-z,A-Z,0-9` range. You will get excellent shortening like `1234567890` -> `380FQs`.
* For NTFS file names generate key from `a-z` range. You will get good shortening and avoid case insensitivity collisions, like `1234567890` -> `iszbmfx`.
* When trolling generate key from Emojis. So `1234567890` will be represented as `😣😄😹😧😋😳`.

Errors:

* `Key must contain at least 2 characters.`
* `Key must contain unique characters.`

### encode(&self, decoded: u64) -> String

Encodes number using characters from the key.

Note that this should not be considered a strong encryption.
It does not contain consistency checks.
And key is easy to reverse engineer with small amount of encoded/decoded samples given.
Treat it as really, really fast obfuscation only.

### decode(&self, encoded: &str) -> Result<u64, &str>

Decodes string using characters from the key.

Errors:

* `Encoded value must contain at least 1 character.`
* `Encoded value contains character not present in key.`
* `Encoded value too big to decode.` - when it would cause u64 overflow.


## Other implementations

* [Raku](https://github.com/bbkr/TinyID)
* [PHP](https://github.com/krowinski/tinyID)
* [Perl](http://search.cpan.org/~bbkr/Integer-Tiny-0.3/lib/Integer/Tiny.pm)
