# literal-enum

This macro can automatically implement the TryFrom<Literal> trait and Into<Literal> trait where the `literal` must be only one type

## Usage Example

```rust
use literal_enum::LiteralEnum;

#[derive(LiteralEnum)]
enum Command {
    /// increment pointer
    #[lit = b'>']
    IncrementPointer, // >
    /// decrement pointer
    #[lit = b'<']
    DecrementPointer, // <
}

let b = b'>';
let cmd = Command::try_from(b).unwrap();
assert!(matches!(cmd, Command::IncrementPointer));

let b: u8 = cmd.into();
assert_eq!(b, b'>');
```


