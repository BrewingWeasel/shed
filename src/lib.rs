use regex::Regex;

use std::{
    borrow::{self, Cow},
    collections::HashMap,
    str::{Chars, Split},
};

// TODO: IDEAS FOR STUFF: + to add ranges, more commands, -p/pretty option, negative numbers (check if
// exists already)
//

pub struct Config {
    pub quiet: bool, // TODO: quiet should work for everything; add multiple streams
}

enum Selection {
    Line(usize),
    Range(usize, usize),
    Step(usize, usize),
    Matching(Regex),
    Any,
}

impl Selection {
    fn in_selection(&self, num: usize, conts: &str) -> bool {
        match self {
            Self::Line(line_num) => num == *line_num,
            Self::Range(start, end) => num >= *start && num <= *end,
            Self::Step(start, step) => (num.saturating_sub(*start) + 1) % step == 0,
            Self::Matching(matching) => matching.is_match(conts),
            Self::Any => true,
        }
    }
}

trait ShedOperation {
    fn run(&self, conts: &mut Cow<'_, str>, cur_string: &mut String) -> bool;
}

struct Substitute<'a> {
    pub regex: Regex,
    pub replace: &'a str,
    pub global: bool,
}

impl<'a> Substitute<'a> {
    pub fn generate(mut split_up_conts: Split<'a, char>) -> Self {
        Substitute {
            regex: Regex::new(split_up_conts.next().unwrap()).unwrap(),
            replace: split_up_conts.next().unwrap(),
            global: split_up_conts.next() == Some("g"),
        }
    }
}

impl ShedOperation for Substitute<'_> {
    fn run(&self, conts: &mut borrow::Cow<'_, str>, _cur_string: &mut String) -> bool {
        let new = if self.global {
            Regex::replace_all(&self.regex, conts, self.replace)
        } else {
            Regex::replace(&self.regex, conts, self.replace)
        }
        .into_owned();
        _ = std::mem::replace(conts, Cow::Owned(new));
        true
    }
}

struct Delete {}

impl ShedOperation for Delete {
    fn run(&self, _conts: &mut Cow<'_, str>, _cur_string: &mut String) -> bool {
        false
    }
}

struct ShedPrint {}

impl ShedOperation for ShedPrint {
    fn run(&self, conts: &mut Cow<'_, str>, cur_string: &mut String) -> bool {
        cur_string.push_str(conts);
        cur_string.push('\n');
        true
    }
}

// TODO: maybe better name? just stolen from gnu docs
struct Transliterate {
    pub map: HashMap<char, char>,
}

impl Transliterate {
    pub fn generate(mut split_up_conts: Split<'_, char>) -> Self {
        Transliterate {
            map: split_up_conts
                .next()
                .unwrap()
                .chars()
                .zip(split_up_conts.next().unwrap().chars())
                .collect(),
        }
    }
}

impl ShedOperation for Transliterate {
    fn run(&self, conts: &mut Cow<'_, str>, _cur_string: &mut String) -> bool {
        _ = std::mem::replace(
            conts,
            Cow::Owned(
                conts
                    .chars()
                    .map(|c| self.map.get(&c).copied().unwrap_or(c))
                    .collect(),
            ),
        );
        true
    }
}

pub fn parse(expressions: Vec<String>, config: Config, conts: String) -> String {
    let mut final_string = String::new();
    let mut operations: Vec<(Box<dyn ShedOperation>, Selection)> = Vec::new();
    for expression in expressions.iter() {
        let mut chars = expression.chars();
        let (selection, mode) = handle_ranges(&mut chars, &conts);
        let mut split_up_conts = expression.split(chars.next().unwrap_or('/'));
        split_up_conts.next();

        // HACK:     not sure how this works or why it's necessary anymore, but without it
        // everything breaks
        if let Selection::Matching(v) = &selection {
            if !v.is_match("") {
                split_up_conts.nth(1); // EXTREMELY Hacky work around, should check for splitting
                                       // char, should use different method to check for empty regex
            }
        }

        let operation: Box<dyn ShedOperation> = match mode {
            's' => Box::new(Substitute::generate(split_up_conts)),
            'd' => Box::new(Delete {}),
            'p' => Box::new(ShedPrint {}),
            'y' => Box::new(Transliterate::generate(split_up_conts)),
            _ => panic!("invalid operation"),
        };
        operations.push((operation, selection))
    }

    for (num, line) in conts.lines().enumerate() {
        let mut line = Cow::from(line);
        let mut print_final = true;
        for (operation, selection) in &operations {
            if selection.in_selection(num, line.as_ref()) {
                let should_print = operation.run(&mut line, &mut final_string);
                if print_final {
                    print_final = should_print;
                }
            }
        }
        if !config.quiet && print_final {
            final_string.push_str(line.as_ref());
            final_string.push('\n');
        }
    }
    final_string
}

fn handle_ranges(input: &mut Chars<'_>, conts: &str) -> (Selection, char) {
    enum SelectionType {
        MatchingPattern,
        Step,
        Default,
    }

    let mut cur_numbers: Vec<String> = vec![String::new()];
    let mut can_add_chars = false;
    let mut selection_type = SelectionType::Default;

    let handle_numbers = |numbers: Vec<String>, selec_type: SelectionType| match numbers.len() {
        1 => {
            if matches!(selec_type, SelectionType::MatchingPattern) {
                Selection::Matching(Regex::new(&numbers[0]).unwrap())
            } else if let Ok(num) = numbers[0].parse::<usize>() {
                Selection::Line(num - 1)
            } else {
                Selection::Any
            }
        }
        2 => {
            let first = numbers[0].parse::<usize>().unwrap();
            let second = numbers[1].parse::<usize>().unwrap();
            if matches!(selec_type, SelectionType::Step) {
                Selection::Step(first, second)
            } else {
                Selection::Range(first.saturating_sub(1), second.saturating_sub(1))
            }
        }
        _ => {
            panic!("Invalid number of inputs");
        }
    };

    loop {
        match input.next() {
            Some(',') => cur_numbers.push(String::new()),
            Some('~') => {
                selection_type = SelectionType::Step;
                cur_numbers.push(String::new());
            }
            Some(c) if c == 's' || c == 'd' || c == 'p' || c == 'y' => {
                if can_add_chars {
                    cur_numbers.last_mut().unwrap().push(c);
                } else {
                    return (handle_numbers(cur_numbers, selection_type), c);
                }
            }
            Some('/') => {
                selection_type = SelectionType::MatchingPattern;
                can_add_chars = !can_add_chars;
            }
            Some('$') => cur_numbers
                .last_mut()
                .unwrap()
                .push_str(&(conts.split('\n').count() - 1).to_string()),
            Some(n) => cur_numbers.last_mut().unwrap().push(n),
            None => panic!("eeh"),
        }
    }
}
