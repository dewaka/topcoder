// https://community.topcoder.com/stat?c=problem_statement&pm=14783

fn is_beautiful(s: &str) -> bool {
    let cs = s.chars().collect::<Vec<char>>();

    for i in 1..cs.len() {
        if cs[i - 1] == cs[i] {
            return true;
        }
    }

    false
}

fn two_same<T>(a: T, b: T, c: T) -> bool
where
    T: PartialEq,
{
    (a == b) || (b == c) || (c == a)
}

fn can_make_beautiful(s: &str) -> bool {
    if s.len() < 3 {
        false
    } else {
        let cs = s.chars().collect::<Vec<char>>();

        for i in 2..cs.len() {
            let a = cs[i - 2];
            let b = cs[i - 1];
            let c = cs[i];

            if two_same(a, b, c) {
                return true;
            }
        }

        false
    }
}

#[test]
fn test_is_beautiful() {
    assert!(!is_beautiful("A"));
    assert!(!is_beautiful("GH"));
    assert!(!is_beautiful("ABCABCBX"));

    assert!(is_beautiful("KEEP"));
    assert!(is_beautiful("ZZZZ"));
}

#[test]
fn test_can_make_beautiful() {
    assert!(can_make_beautiful("VIKING"));
    assert!(can_make_beautiful("AABB"));

    assert!(!can_make_beautiful("XX"));
    assert!(!can_make_beautiful("A"));
    assert!(!can_make_beautiful("BCAB"));
}
