use convertisseur_rust::convertisseur;

#[test]
fn test_binary_to_octal() {
    assert_eq!(convertisseur::binary_to_octal(101101), 55);
    assert_eq!(convertisseur::binary_to_octal(111111111111111), 77777);
    assert_eq!(convertisseur::binary_to_octal(110010), 62);
    assert_eq!(convertisseur::binary_to_octal(101010), 52);
}

#[test]
fn test_binary_to_decimal() {
    assert_eq!(convertisseur::binary_to_decimal(10101010101), 1365);
    assert_eq!(convertisseur::binary_to_decimal(111111), 63);
    assert_eq!(convertisseur::binary_to_decimal(111111111111111), 32767);
}

#[test]
fn test_binary_to_hexadecimal() {
    assert_eq!(convertisseur::binary_to_hexadecimal(101101), "2D");
    assert_eq!(convertisseur::binary_to_hexadecimal(1111), "F");
    assert_eq!(convertisseur::binary_to_hexadecimal(1100100), "64");
}

#[test]
fn test_octal_to_binary() {
    assert_eq!(convertisseur::octal_to_binary(777777), "111111111111111111");
    assert_eq!(
        convertisseur::octal_to_binary(1122021),
        "1001010010000010001"
    );
    assert_eq!(convertisseur::octal_to_binary(55555), "101101101101101");
}

#[test]
fn test_octal_to_decimal() {
    assert_eq!(convertisseur::octal_to_decimal(777777), "262143");
    assert_eq!(convertisseur::octal_to_decimal(1122021), "304145");
    assert_eq!(convertisseur::octal_to_decimal(55555), "23405");
}

#[test]
fn test_octal_to_hexadecimal() {
    assert_eq!(convertisseur::octal_to_hexadecimal(777777), "3FFFF");
    assert_eq!(convertisseur::octal_to_hexadecimal(1122021), "4A411");
    assert_eq!(convertisseur::octal_to_hexadecimal(55555), "5B6D");
}

#[test]
fn test_decimal_to_binary() {
    assert_eq!(convertisseur::decimal_to_binary(1024), "10000000000");
    assert_eq!(
        convertisseur::decimal_to_binary(101010101010),
        "1011110000100101010111101001100010010"
    );
    assert_eq!(convertisseur::decimal_to_binary(9999), "10011100001111");
}

#[test]
fn test_decimal_to_octal() {
    assert_eq!(convertisseur::decimal_to_octal(1024), "2000");
    assert_eq!(
        convertisseur::decimal_to_octal(101010101010),
        "1360452751422"
    );
    assert_eq!(convertisseur::decimal_to_octal(9999), "23417");
}

#[test]
fn test_decimal_to_hexadecimal() {
    assert_eq!(convertisseur::decimal_to_hexadecimal(1024), "400");
    assert_eq!(
        convertisseur::decimal_to_hexadecimal(101010101010),
        "1784ABD312"
    );
    assert_eq!(convertisseur::decimal_to_hexadecimal(9999), "270F");
}

#[test]
fn test_hexadecimal_to_binary() {
    assert_eq!(
        convertisseur::hexadecimal_to_binary("FFFFFF"),
        "111111111111111111111111"
    );
    assert_eq!(
        convertisseur::hexadecimal_to_binary("ABCD"),
        "1010101111001101"
    );
    assert_eq!(
        convertisseur::hexadecimal_to_binary("A521"),
        "1010010100100001"
    );
}

#[test]
fn test_hexadecimal_to_octal() {
    assert_eq!(convertisseur::hexadecimal_to_octal("FFFFFF"), "77777777");
    assert_eq!(convertisseur::hexadecimal_to_octal("ABCD"), "125715");
    assert_eq!(convertisseur::hexadecimal_to_octal("A521"), "122441");
}

#[test]
fn test_hexadecimal_to_decimal() {
    assert_eq!(convertisseur::hexadecimal_to_decimal("FFFFFF"), "16777215");
    assert_eq!(convertisseur::hexadecimal_to_decimal("ABCD"), "43981");
    assert_eq!(convertisseur::hexadecimal_to_decimal("A521"), "42273");
}
