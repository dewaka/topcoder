// https://community.topcoder.com/stat?c=problem_statement&pm=15138

fn is_positive(question: &str) -> bool {
    // If the count of "not" in question is even, then it should be positive. If
    // it is an odd number then it will be negative.
    let mut not_count = 0;

    for w in question.split(" ") {
        if w == "not" {
            not_count += 1;
        }
    }

    not_count % 2 == 0
}

pub fn answer(question: &str) -> String {
    if is_positive(question) {
        "True.".to_string()
    } else {
        "False.".to_string()
    }
}

#[test]
fn test_answer() {
    let example_cases = vec![
        ("Do you want to get accepted?", "True."),
        ("Do you not want to get accepted?", "False."),
        ("Do you want to not get accepted?", "False."),
        ("Do you want to get not not accepted?", "True."),
        ("Do you not want to not get not not not accepted?", "False."),
    ];

    for (q, a) in &example_cases {
        assert_eq!(answer(q), a.to_owned());
    }
}
