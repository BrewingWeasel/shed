mod common;
use shed::{parse, Config};

#[test]
fn simple_change_single_line() {
    run_test!("Hello everyone!", "1c hi", "hi\n");
}

#[test]
fn simple_change_single_line_multiple_spaces_before_arg() {
    run_test!("Hello everyone!", "1c     hi", "hi\n");
}

#[test]
fn change_second_line() {
    run_test!(
        "Hello everyone!\nHi!\nLabas!",
        "2c     Sveiks!",
        "Hello everyone!\nSveiks!\nLabas!\n"
    );
}

#[test]
fn change_matching_lines() {
    run_test!(
        "Hello everyone!\nHi!\nLabas!\nd",
        "/!/c Sveiks!",
        "Sveiks!\nSveiks!\nSveiks!\nd\n"
    );
}

#[test]
fn change_matching_lines_non_matching_in_middle() {
    run_test!(
        "Hello everyone!\nHi!\ne\nLabas!\nd",
        "/!/c Sveiks!",
        "Sveiks!\nSveiks!\ne\nSveiks!\nd\n"
    );
}
