use literal_enum::LiteralEnum;


#[derive(LiteralEnum)]
enum StrDefault {
    Hello,
    World,
}

#[test]
fn test_str_default() {
    let s: &str = StrDefault::Hello.into();
    assert_eq!(s, "Hello");

    let s: &str = StrDefault::World.into();
    assert_eq!(s, "World");
}

#[derive(LiteralEnum)]
enum StrExplicit {
    #[lit = "Hello"]
    Hello,
    World,
}

#[test]
fn test_str_explicit() {
    let s: &str = StrExplicit::Hello.into();
    assert_eq!(s, "Hello");

    let s: &str = StrExplicit::World.into();
    assert_eq!(s, "World");
}

#[derive(LiteralEnum)]
enum ByteStr {
    #[lit = b"Hello"]
    Hello,
    World,
}

#[test]
fn test_byte_str() {
    let s: &[u8] = ByteStr::Hello.into();
    assert_eq!(s, b"Hello");

    let s: &[u8] = ByteStr::World.into();
    assert_eq!(s, b"World");
}

#[derive(LiteralEnum)]
enum CStr {
    #[lit = c"Hello"]
    Hello,
    World,
}

#[test]
fn test_c_str() {
    let s: &std::ffi::CStr = CStr::Hello.into();
    assert_eq!(s, c"Hello");

    let s: &std::ffi::CStr = CStr::World.into();
    assert_eq!(s, c"World");
}

#[derive(LiteralEnum)]
enum Byte {
    #[lit = b'h']
    Hello,
    #[lit = b'w']
    World,
    X,
}

#[test]
fn test_byte() {
    let b: u8 = Byte::Hello.into();
    assert_eq!(b, b'h');

    let b: u8 = Byte::World.into();
    assert_eq!(b, b'w');

    let b: u8 = Byte::X.into();
    assert_eq!(b, b'X');
}

#[derive(LiteralEnum)]
enum Char {
    #[lit = 'h']
    Hello,
    #[lit = 'w']
    World,
    X,
}

#[test]
fn test_char() {
    let c: char = Char::Hello.into();
    assert_eq!(c, 'h');

    let c: char = Char::World.into();
    assert_eq!(c, 'w');

    let c: char = Char::X.into();
    assert_eq!(c, 'X');
}

#[derive(LiteralEnum)]
enum U32 {
    #[lit = 1]
    One,
    #[lit = 2]
    Two,
}

#[test]
fn test_u32() {
    let i: u32 = U32::One.into();
    assert_eq!(i, 1);

    let i: u32 = U32::Two.into();
    assert_eq!(i, 2);
}

#[derive(LiteralEnum)]
enum Bool {
    #[lit = true]
    True,
    #[lit = false]
    False,
}

#[test]
fn test_bool() {
    let b: bool = Bool::True.into();
    assert_eq!(b, true);

    let b: bool = Bool::False.into();
    assert_eq!(b, false);
}

