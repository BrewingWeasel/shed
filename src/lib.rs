use regex::{Regex, Replacer};

use std::{
    borrow::Cow,
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

    match mode {
        's' => substitute(&mut split_up_conts, conts, selection),
        'd' => delete(conts, selection),
        e => panic!("invalid input, {}, {:?}", e, chars.next()),
    }
}

fn substitute(args: &mut Split<'_, char>, conts: String, range: Selection) -> String {
    let initial = Regex::new(args.next().unwrap()).unwrap();
    let replace = args.next().unwrap();

    let replacement_func = if args.next() == Some("g") {
        Regex::replace_all
    } else {
        Regex::replace
    };

    run_replacement(initial, replacement_func, replace, conts, range)
}

fn run_replacement<F, R>(
    initial: Regex,
    operation: F,
    replace: R,
    conts: String,
    range: Selection,
) -> String
where
    F: for<'h> Fn(&Regex, &'h str, R) -> Cow<'h, str>,
    R: Replacer + Copy,
{
    conts.lines().enumerate().fold("".to_string(), |i, (n, l)| {
        if range.in_selection(&n, l) {
            i + operation(&initial, l, replace).as_ref() + "\n"
        } else {
            i + l + "\n"
        }
    })
}

fn delete(conts: String, range: Selection) -> String {
    conts
        .lines()
        .enumerate()
        .filter(|(l, n)| !range.in_selection(l, n))
        .map(|(_, v)| v)
        .collect::<String>()
}

fn handle_ranges(input: &mut Chars<'_>, conts: &str) -> (Selection, char) {
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
            Some(c) if c == 's' || c == 'd' => {
                if can_add_chars {
                    cur_numbers.last_mut().unwrap().push(c);
                } else {
                    return (handle_numbers(cur_numbers), c);
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
