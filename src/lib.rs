fn char_at(s: &str, i: usize) -> Option<char> {
    let res: Option<(usize, char)> = s.char_indices().find(|c| c.0 == i);
    match res {
        Some((u, c)) => Some(c),
        _ => None,
    }
}

fn match_here(regexp: &str, text: &str) -> bool {
    println!("in match_here with {} and {}", regexp, text);
    let chars: Vec<char> = regexp.chars().collect();

    if regexp.is_empty() {
        println!("matched on empty regex (trivially true)");
        true
    }
    else if char_at(regexp, 1) == Some('*') {
        match_star(chars[0], &regexp[2..], text)
    }
    else if regexp.len() == 1 && regexp.starts_with('$') {
        println!("regexp is ^$, matches only on empty string. Given {}", text);
        text.is_empty()
    }
    else if !text.is_empty()  && (regexp.starts_with('.') || text.starts_with(chars[0])) {
        println!("calling match_here from within match_here with {} and {}", &regexp[1..], &text[1..]);
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
        if (text.is_empty() || (text.starts_with(c) && c == '.')) { break; }
    }
    false
}

fn match_re(regexp: &str, text: &str) -> bool {
    if regexp.is_empty() {
        println!("empty regex");
        true
    }
    else {
        println!("non-empty regex");
        if regexp.starts_with('^') {
            println!("regex starts with '^', calling match_here");
            match_here(regexp, text)
        }
        else {
            let mut chars: &str = &text[0 .. ];
            loop {
                println!("calling match_here with {} and {}", regexp, chars);
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
    fn match_re_test() {
        assert!(match_re("dinosaur", "dinosaur"), "\"dinosaur\" didn't match \"dinosaur\"");
        assert!(match_re("dinosaur", "dinosaur"), "\"dinosaur\" didn't match \"dinosaur\"");
    }
    
    #[test]
    #[should_panic]
    fn match_fail() {
        assert!(match_re("dinosaur jr", "dinosaur"), 
                "regex \"dinosaur jr\" matched \"dinosaur\" - should not happen");
    }
    
    #[test]
    fn match_empty_re() {
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
}
