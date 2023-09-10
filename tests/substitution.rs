mod common;
use shed::{parse, Config};

#[test]
fn simple_one_line_substitution() {
    run_test!("Hello everyone!", "s/Hello/Labas/", "Labas everyone!\n");
}

#[test]
fn simple_one_line_substitution_no_global() {
    run_test!(
        "Hello everyone! Hello!",
        "s/Hello/Labas/",
        "Labas everyone! Hello!\n"
    )
}

#[test]
fn simple_one_line_substitution_global() {
    run_test!(
        "Hello everyone! Hello!",
        "s/Hello/Labas/g",
        "Labas everyone! Labas!\n"
    )
}

#[test]
fn simple_one_line_substitution_deletion() {
    run_test!("Hello everyone!", "s/Hello//", " everyone!\n")
}

#[test]
fn first_line_selection() {
    run_test!(
        "Hello everyone!\nHello world!",
        "1s/Hello/Labas/",
        "Labas everyone!\nHello world!\n"
    )
}

#[test]
fn matching_line_selection() {
    run_test!(
        "Hello everyone!\nHello world!",
        "/world/s/Hello/Labas/",
        "Hello everyone!\nLabas world!\n"
    );
}

#[test]
fn matching_line_selection_multiple_matches() {
    run_test!(
        "Hello everyone!\nHello world!\nHello, are you the world?",
        "/world/s/Hello/Labas/",
        "Hello everyone!\nLabas world!\nLabas, are you the world?\n"
    )
}

#[test]
fn matching_line_selection_multiple_matches_changeless() {
    run_test!(
        "Hello everyone!\nHello world!\nHello, are you the world?",
        "/world/s/Labas/Hello/",
        "Hello everyone!\nHello world!\nHello, are you the world?\n"
    )
}

#[test]
fn matching_line_selection_multiple_matches_some_changed() {
    run_test!(
        "Hello everyone!\nHello world!\nHow are you, world?",
        "/world/s/Hello/Labas/",
        "Hello everyone!\nLabas world!\nHow are you, world?\n"
    )
}

#[test]
fn regex_inital_global() {
    run_test!("Hello! Hallo!", "s/H.llo/Labas/g", "Labas! Labas!\n")
}

#[test]
fn regex_inital_multiline() {
    run_test!(
        "Labas! Kaip sekasi?\nHello! How are you?\nياخشىمۇسىز؟",
        "s/^.{5}!/สวัสดี,/g",
        "สวัสดี, Kaip sekasi?\nสวัสดี, How are you?\nياخشىمۇسىز؟\n"
    )
}
