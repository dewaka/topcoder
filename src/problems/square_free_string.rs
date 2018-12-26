// SquareFreeString - https://community.topcoder.com/stat?c=problem_statement&pm=14428
use std::collections::HashSet;

// Using checked set for memoisation
fn has_square_mem(s: &str, checked: &mut HashSet<String>) -> bool {
    if s.is_empty() {
        false
    } else {
        let n = s.len() / 2;
        if s[0..n] == s[n..] {
            true
        } else {
            let left_side = &s[1..];
            let right_side = &s[0..s.len() - 1];

            if !checked.contains(left_side) && has_square_mem(left_side, checked) {
                return true;
            } else {
                checked.insert(left_side.to_string());
            }

            if !checked.contains(right_side) && has_square_mem(right_side, checked) {
                return true;
            } else {
                checked.insert(right_side.to_string());
            }

            false
        }
    }
}

fn has_square(s: &str) -> bool {
    let mut checked = HashSet::new();
    has_square_mem(s, &mut checked)
}

pub fn is_square_free(s: &str) -> bool {
    !has_square(s)
}

#[test]
fn test_has_square() {
    assert!(!has_square(""));
    assert!(has_square("aa"));
    assert!(!has_square("aba"));

    assert!(has_square("bobo"));
}

#[test]
fn test_is_square_free() {
    assert!(!is_square_free("bobo"));
    assert!(!is_square_free("apple"));
    assert!(!is_square_free("aydyamrbnauhftmphyrooyq"));

    assert!(is_square_free("pen"));
    assert!(is_square_free("qwertyuiopasdfghjklzxcvbnm"));
}
