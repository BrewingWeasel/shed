use regex::Regex;

use std::{
    str::{Chars, Split},
    usize,
};

// TODO: IDEAS FOR STUFF: + to add ranges, more commands, -p/pretty option, negative numbers (check if
// exists already)

enum Selection {
    Line(usize),
    Range(usize, usize),
    Matching(Regex),
}

impl Selection {
    fn in_selection(&self, num: &usize, conts: &str) -> bool {
        match self {
            Self::Line(line_num) => num == line_num,
            Self::Range(start, end) => num >= start && num <= end,
            Self::Matching(matching) => matching.is_match(conts),
        }
    }
}

pub fn parse(expression: String, conts: String) -> String {
    let mut chars = expression.chars();
    let (selection, mode) = handle_ranges(&mut chars, &conts);
    let mut split_up_conts = expression.split(chars.next().unwrap_or('/'));
    split_up_conts.next();

    if let Selection::Matching(v) = &selection {
        if !v.is_match("") {
            split_up_conts.nth(1); // EXTREMELY Hacky work around, should check for splitting
                                   // char, should use different method to check for empty regex
        }
    }

    let changed = match mode {
        's' => substitute(&mut split_up_conts, conts, selection),
        'd' => delete(conts, selection),
        e => panic!("invalid input, {}, {:?}", e, chars.next()),
    };
    changed
}

fn substitute(args: &mut Split<'_, char>, conts: String, range: Selection) -> String {
    let initial = Regex::new(args.next().unwrap()).unwrap();
    let replace = args.next().unwrap();
    let global = match args.next() {
        Some("g") => true,
        _ => false,
    };

    if global {
        conts.lines().enumerate().fold("".to_string(), |i, (n, l)| {
            if range.in_selection(&n, l) {
                i + &initial.replace_all(&l, replace).to_string() + "\n"
            } else {
                i + l + "\n"
            }
        }) // TODO: nekartok
    } else {
        conts.lines().enumerate().fold("".to_string(), |i, (n, l)| {
            if range.in_selection(&n, l) {
                i + &initial.replace(&l, replace).to_string() + "\n"
            } else {
                i + l + "\n"
            }
        })
    }
}

fn delete(conts: String, range: Selection) -> String {
    conts
        .lines()
        .enumerate()
        .filter(|(l, n)| !range.in_selection(l, n))
        .map(|(_, v)| v)
        .collect::<String>()
}

fn handle_ranges(input: &mut Chars<'_>, conts: &String) -> (Selection, char) {
    // TODO: make this a struct
    let mut cur_numbers: Vec<String> = vec!["".to_string()];
    let mut can_add_chars = false;

    let handle_numbers = |numbers: Vec<String>| match numbers.len() {
        1 => {
            if let Ok(num) = numbers[0].parse::<usize>() {
                Selection::Line(num - 1)
            } else {
                Selection::Matching(Regex::new(&numbers[0]).unwrap())
            }
        }
        2 => Selection::Range(
            numbers[0].parse::<usize>().unwrap() - 1,
            numbers[1].parse::<usize>().unwrap(),
        ),
        _ => {
            panic!("Invalid number of inputs");
        }
    };

    loop {
        match input.next() {
            Some(',') => cur_numbers.push("".to_string()),
            Some('s') => {
                if can_add_chars {
                    cur_numbers.last_mut().unwrap().push('s');
                } else {
                    return (handle_numbers(cur_numbers), 's');
                }
            } // TODO: CLEAN UP BAD CODE
            Some('d') => {
                if can_add_chars {
                    cur_numbers.last_mut().unwrap().push('d');
                } else {
                    return (handle_numbers(cur_numbers), 'd');
                }
            }
            Some('/') => {
                can_add_chars = !can_add_chars;
            }
            Some('$') => cur_numbers
                .last_mut()
                .unwrap()
                .push_str(&(conts.split('\n').collect::<Vec<&str>>().len() - 1).to_string()),
            Some(n) => cur_numbers.last_mut().unwrap().push(n),
            None => panic!("eeh"),
        }
    }
}
