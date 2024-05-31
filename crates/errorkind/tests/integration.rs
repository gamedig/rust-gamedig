extern crate errorkind;

use errorkind::{generate_error_kind, ErrorKind};

#[derive(Debug, ErrorKind)]
struct TestErrorCaseOne;

#[derive(Debug, ErrorKind)]
struct TestErrorCaseTwo;

#[derive(Debug, ErrorKind)]
struct TestErrorCaseThree;

// Generate the ErrorKind enum
generate_error_kind!();

#[test]
fn test_error_kind_enum_generation() {
    // Ensure that the ErrorKind enum contains the expected variants
    let case_one: ErrorKind = TestErrorCaseOne.into();
    let case_two: ErrorKind = TestErrorCaseTwo.into();
    let case_three: ErrorKind = TestErrorCaseThree.into();

    match case_one {
        ErrorKind::TestErrorCaseOne => println!("Case One variant is present."),
        _ => panic!("Expected Case One variant."),
    }

    match case_two {
        ErrorKind::TestErrorCaseTwo => println!("Case Two variant is present."),
        _ => panic!("Expected Case Two variant."),
    }

    match case_three {
        ErrorKind::TestErrorCaseThree => println!("Case Three variant is present."),
        _ => panic!("Expected Case Three variant."),
    }
}
