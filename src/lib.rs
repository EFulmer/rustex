#![allow(dead_code)]

#[derive(Debug)]
enum RegexChar {
    Char(char),
    CharClass(Vec<char>),
    StartLine,
    EndLine,
    AnyChar,
    AnyNumber(char),
    OnePlus(char),
    ZeroOrOne(char),
}

struct Regex(Vec<RegexChar>);

impl Regex {
    // TODO this needs cleaner error checking. 
    // Also can possibly skip using peekable, or even char_indices
    pub fn new(regexp: &str) -> Regex {
        let mut result = Vec::new();
        let mut iter = regexp.char_indices().peekable();
    
        loop {
            if iter.peek().is_none() { break; }
    
            let (i, c) = iter.next().unwrap();
    
            match c {
                '[' => {
                    let next = iter.next();
                    let mut chars = Vec::new();
                    loop {
                        if next.is_none() {
                            panic!("found EOL in character class");
                        }
                        if next.unwrap().1 == ']' {
                            break;
                        }
                        chars.push(next.unwrap().1);
                    }
                    result.push(RegexChar::CharClass(chars))
                }
                '\\' => {
                    match iter.peek() {
                        Some(t) => result.push(RegexChar::Char(t.1)),
                        None => panic!("regex escape backslash found at EOL"),
                    }
                }
                '^' => result.push(RegexChar::StartLine),
                '$' => result.push(RegexChar::EndLine),
                '.' => result.push(RegexChar::AnyChar),
                '*' => {
                    match result.pop().unwrap() {
                        RegexChar::Char(c) => result.push(RegexChar::AnyNumber(c)),
                        RegexChar::AnyChar => result.push(RegexChar::AnyChar),
                        _ => panic!("expected character"), // could look nicer
                    }
                }
                '+' => {
                    match result.pop().unwrap() {
                        RegexChar::Char(c) => result.push(RegexChar::OnePlus(c)),
                        RegexChar::AnyChar => result.push(RegexChar::AnyChar),
                        _ => panic!("expected character"),
                    }
                }
                '?' => {
                    match result.pop().unwrap() {
                        RegexChar::Char(c) => result.push(RegexChar::ZeroOrOne(c)),
                        RegexChar::AnyChar => result.push(RegexChar::AnyChar),
                        _ => panic!("expected character"),
                    }
                }
                c => result.push(RegexChar::Char(c)),
            }
        }
    
        Regex(result)
    }

    fn re_match(&self, s: &str) -> bool {
        true
    }
}


pub fn char_at(s: &str, i: usize) -> Option<char> {
    let res: Option<(usize, char)> = s.char_indices().find(|c| c.0 == i);
    match res {
        Some((_, c)) => Some(c),
        _ => None,
    }
}

fn match_here(regexp: &str, text: &str) -> bool {
    let chars: Vec<char> = regexp.chars().collect();

    if regexp.is_empty() {
        true
    }
    else if char_at(regexp, 1) == Some('*') {
        match_star(chars[0], &regexp[2..], text)
    }
    else if regexp.len() == 1 && regexp.starts_with('$') {
        text.is_empty()
    }
    else if !text.is_empty()  && (regexp.starts_with('.') || text.starts_with(chars[0])) {
        match_here(&regexp[1..], &text[1..])
    }
    else {
        false
    }
}

fn match_star(c: char, regexp: &str, text: &str) -> bool {
    let mut text = &text[0..];
    loop {
        if match_here(regexp, text) {
            return true;
        }
        text = &text[1..];
        if text.is_empty() || (text.starts_with(c) && c == '.') { break; }
    }
    false
}

pub fn match_re(regexp: &str, text: &str) -> bool {
    if regexp.is_empty() {
        true
    }
    else {
        if regexp.starts_with('^') {
            match_here(&regexp[1..], text)
        }
        else {
            let mut chars: &str = &text[0 .. ];
            loop {
                if match_here(regexp, chars) {
                    return true;
                }
                if chars.len() == 0 { break; }
                chars = &chars[1 .. ];
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::match_re;

    #[test]
    fn test_match_re() {
        assert!(match_re("dinosaur", "dinosaur"), "\"dinosaur\" didn't match \"dinosaur\"");
    }
    
    #[test]
    #[should_panic]
    fn test_match_fail() {
        assert!(match_re("dinosaur jr", "dinosaur"), 
                "regex \"dinosaur jr\" matched \"dinosaur\" - should not happen");
    }
    
    #[test]
    fn test_match_re_empty() {
        assert!(match_re("", ""), "empty regex should've matched empty string");
        assert!(match_re("", "rust"), "empty regex should've matched \"rust\"");
    }
    
    #[test]
    fn match_empty_string() {
        assert_eq!(match_re("rust", ""), false);
    }
    
    #[test]
    fn match_dot_star() {
        assert!(match_re(".*", ""), ".* should match empty string");
        assert!(match_re(".*", "memes"), ".* should match \"memes\"");
    }

    #[test]
    fn match_star() {
        assert!(match_re("a*", "aaa"), "\"a*\" should match \"aaa\"");
    }

    #[test]
    fn some_dict_stuff() {
        assert!(match_re("^.a...x.$", "cachexy"), "\"^.a...x.$\" should match \"cachexy\"");
        assert!(match_re("^.a...x.$", "carboxy"), "\"^.a...x.$\" should match \"carboxy\"");
        assert!(match_re("^.a...x.$", "martext"), "\"^.a...x.$\" should match \"martext\"");
        assert!(match_re("^.a...x.$", "panmixy"), "\"^.a...x.$\" should match \"panmixy\"");
    }
    // Thank you, Jonny! :)
    // #[test]
    // fn match_email() {
    //     assert!(match_re(".*@.*", "me@example.com"), "what");
    // }
}
