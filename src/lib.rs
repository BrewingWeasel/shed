use regex::Regex;

use std::{
    borrow::{self, Cow},
    collections::HashMap,
    str::Chars,
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

struct Substitute {
    pub regex: Regex,
    pub replace: String,
    pub global: bool,
}

impl Substitute {
    pub fn generate(args: Vec<String>) -> Self {
        Substitute {
            regex: Regex::new(&args[0]).unwrap(),
            replace: args[1].to_owned(),
            global: args[2] == "g",
        }
    }
}

impl ShedOperation for Substitute {
    fn run(&self, conts: &mut borrow::Cow<'_, str>, _cur_string: &mut String) -> bool {
        let new = if self.global {
            Regex::replace_all(&self.regex, conts, &self.replace)
        } else {
            Regex::replace(&self.regex, conts, &self.replace)
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
    pub fn generate(args: Vec<String>) -> Self {
        Self {
            map: args[0].chars().zip(args[1].chars()).collect(),
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

struct Change {
    pub change: String,
}

impl Change {
    pub fn generate(to_change: String) -> Self {
        Change { change: to_change }
    }
}

impl ShedOperation for Change {
    fn run(&self, conts: &mut Cow<'_, str>, _cur_string: &mut String) -> bool {
        _ = std::mem::replace(conts, Cow::Owned(self.change.to_owned())); // TODO: remove
                                                                          // to_string call
        true
    }
}

struct Insert {
    pub to_insert: String,
}

impl Insert {
    pub fn generate(to_insert: String) -> Self {
        Self { to_insert }
    }
}

impl ShedOperation for Insert {
    fn run(&self, _conts: &mut Cow<'_, str>, cur_string: &mut String) -> bool {
        cur_string.push_str(&self.to_insert);
        cur_string.push('\n');
        true
    }
}

pub fn parse(expressions: Vec<String>, config: Config, conts: String) -> String {
    let mut final_string = String::new();
    let mut operations: Vec<(Box<dyn ShedOperation>, Selection)> = Vec::new();
    for expression in expressions.iter() {
        let mut chars = expression.chars();
        let (selection, mode) = handle_ranges(&mut chars, &conts);
        let operation = get_operation(chars, mode);

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

fn get_args(mut input: Chars) -> Vec<String> {
    let seperator = input.next().unwrap_or('/');
    input
        .collect::<String>()
        .split(seperator)
        .map(|v| v.to_string())
        .collect()
}

fn get_single_arg(mut input: Chars<'_>) -> String {
    let input = input.by_ref().skip_while(|c| c.is_whitespace());
    input.collect()
}

fn get_operation(input: Chars, operation: char) -> Box<dyn ShedOperation + '_> {
    match operation {
        's' => {
            let args = get_args(input);
            Box::new(Substitute::generate(args))
        }
        'y' => {
            let args = get_args(input);
            Box::new(Transliterate::generate(args))
        }
        'c' => Box::new(Change::generate(get_single_arg(input))),
        'i' => Box::new(Insert::generate(get_single_arg(input))),
        'd' => Box::new(Delete {}),
        'p' => Box::new(ShedPrint {}),
        _ => unreachable!(),
    }
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
            Some(c) if ['s', 'd', 'p', 'y', 'c', 'i'].contains(&c) => {
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
