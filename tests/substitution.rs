use shed;

#[test]
fn simple_one_line_substitution() {
    assert_eq!(
        "Labas everyone!\n".to_string(),
        shed::parse("s/Hello/Labas/".to_string(), "Hello everyone!".to_string(),)
    )
}

#[test]
fn simple_one_line_substitution_no_global() {
    assert_eq!(
        "Labas everyone! Hello!\n".to_string(),
        shed::parse(
            "s/Hello/Labas/".to_string(),
            "Hello everyone! Hello!".to_string(),
        )
    )
}

#[test]
fn simple_one_line_substitution_global() {
    assert_eq!(
        "Labas everyone! Labas!\n".to_string(),
        shed::parse(
            "s/Hello/Labas/g".to_string(),
            "Hello everyone! Hello!".to_string(),
        )
    )
}

#[test]
fn simple_one_line_substitution_deletion() {
    assert_eq!(
        " everyone!\n".to_string(),
        shed::parse("s/Hello//".to_string(), "Hello everyone!".to_string(),)
    )
}

#[test]
fn first_line_selection() {
    assert_eq!(
        "Labas everyone!\nHello world!\n".to_string(),
        shed::parse(
            "1s/Hello/Labas/".to_string(),
            "Hello everyone!\nHello world!".to_string(),
        )
    )
}

#[test]
fn matching_line_selection() {
    assert_eq!(
        "Hello everyone!\nLabas world!\n".to_string(),
        shed::parse(
            "/world/s/Hello/Labas/".to_string(),
            "Hello everyone!\nHello world!".to_string(),
        )
    )
}

#[test]
fn matching_line_selection_multiple_matches() {
    assert_eq!(
        "Hello everyone!\nLabas world!\nLabas, are you the world?\n".to_string(),
        shed::parse(
            "/world/s/Hello/Labas/".to_string(),
            "Hello everyone!\nHello world!\nHello, are you the world?".to_string(),
        )
    )
}

#[test]
fn matching_line_selection_multiple_matches_changeless() {
    assert_eq!(
        "Hello everyone!\nHello world!\nHello, are you the world?\n".to_string(),
        shed::parse(
            "/world/s/Labas/Hello/".to_string(),
            "Hello everyone!\nHello world!\nHello, are you the world?".to_string(),
        )
    )
}

#[test]
fn matching_line_selection_multiple_matches_some_changed() {
    assert_eq!(
        "Hello everyone!\nLabas world!\nHow are you, world?\n".to_string(),
        shed::parse(
            "/world/s/Hello/Labas/".to_string(),
            "Hello everyone!\nHello world!\nHow are you, world?".to_string(),
        )
    )
}

#[test]
fn regex_inital_global() {
    assert_eq!(
        "Labas! Labas! হ্যালো!\n".to_string(),
        shed::parse(
            "s/H.llo/Labas/g".to_string(),
            "Hello! Hallo! হ্যালো!".to_string(),
        )
    )
}

#[test]
fn regex_inital_multiline() {
    assert_eq!(
        "สวัสดี, Kaip sekasi?\nสวัสดี, How are you?\nياخشىمۇسىز؟\n".to_string(),
        shed::parse(
            "s/^.{5}!/สวัสดี,/g".to_string(),
            "Labas! Kaip sekasi?\nHello! How are you?\nياخشىمۇسىز؟".to_string(),
        )
    )
}
