// https://community.topcoder.com/stat?c=problem_statement&pm=14836

fn replacements(input: &str, check: &str, sub: &str) -> Vec<String> {
    let mut rs: Vec<String> = vec![];

    if input.len() >= check.len() {
        let splits: Vec<&str> = input.split(check).collect();

        println!("splits len = {}", splits.len());

        for i in 1..splits.len() {
            let head = &splits[0..i];
            let tail = &splits[i..];

            let rep = format!("{}{}{}", head.join(check), sub, tail.join(check));
            rs.push(rep);
        }
    }

    rs
}

fn check_naive(t1: &str, t2: &str) -> bool {
    if t1 == t2 {
        true
    } else if t1.len() < t2.len() {
        false
    } else {
        let rs = replacements(t1, "()", "");
        for r in &rs {
            if check_naive(r, t2) {
                return true;
            }
        }

        false
    }
}

fn check(t1: &str, t2: &str) -> bool {
    check_naive(t1, t2)
}

#[test]
fn test_replacements() {
    let rs = replacements("foo bar foo baz", "foo", "fib");
    assert_eq!(rs, vec!["fib bar foo baz", "foo bar fib baz"]);
}

#[test]
fn test_check() {
    assert!(check("()", "()"));
    assert!(check("(())", "()"));
    assert!(check("((())((())())())", "(()(())())"));

    assert!(!check("(()()())", "((()))"));
    assert!(!check("((())((())())())", "((()()()()()))"));
}
