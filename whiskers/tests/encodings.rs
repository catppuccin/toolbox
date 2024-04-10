//! tests that ensure the special encoding fixtures are left untouched
#[test]
fn utf8() {
    let bytes = &include_bytes!("fixtures/encodings/utf8.tera")[..3];
    assert_eq!(
        bytes, b"---",
        "fixtures/encodings/utf8.tera needs to be re-encoded to UTF-8"
    );
}

#[test]
fn utf8bom() {
    let bytes = &include_bytes!("fixtures/encodings/utf8bom.tera")[..6];
    assert_eq!(
        bytes, b"\xEF\xBB\xBF---",
        "fixtures/encodings/utf8bom.tera needs to be re-encoded to UTF-8 with BOM"
    );
}

#[test]
fn utf16be() {
    let bytes = &include_bytes!("fixtures/encodings/utf16be.tera")[..2];
    assert_eq!(
        bytes, b"\xFE\xFF",
        "fixtures/encodings/utf16be.tera needs to be re-encoded to UTF-16 BE"
    );
}

#[test]
fn utf16le() {
    let bytes = &include_bytes!("fixtures/encodings/utf16le.tera")[..2];
    assert_eq!(
        bytes, b"\xFF\xFE",
        "fixtures/encodings/utf16le.tera needs to be re-encoded to UTF-16 LE"
    );
}
