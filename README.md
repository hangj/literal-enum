# literal-enum

Automatically implements the `TryFrom<Literal>` trait and `Into<Literal>` trait for an `enum` where the `literal`s must be the same type(one of [`&'static str`, `&'static [u8]`, `u8`, `char`, `u32`, `bool`])

If you need to convert other interger type(i8, i16, u16, i32, i64, u64, i128, u128), you can use [intype-enum](https://crates.io/crates/inttype-enum)

## Usage Example

```rust
use literal_enum::LiteralEnum;

#[derive(LiteralEnum)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Command {
    /// increment pointer
    #[lit = b'>']
    IncrementPointer,
    /// decrement pointer
    #[lit = b'<']
    DecrementPointer,
}

assert_eq!(Command::try_from(b'>').unwrap(), Command::IncrementPointer);
let b: u8 = Command::IncrementPointer.into();
assert_eq!(b, b'>');
```

```rust
#[derive(LiteralEnum)]
enum E {
    A,
    AAAA,
}

let s: &'static str = E::A.into();
assert_eq!(s, "A");
```
