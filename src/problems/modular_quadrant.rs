// ModularQuadrant problem - https://community.topcoder.com/stat?c=problem_statement&pm=15236

use std::cmp::max;

pub fn sum(r1: i32, r2: i32, c1: i32, c2: i32) -> i32 {
    let mut result: i32 = 0;

    for r in r1..r2 + 1 {
        for c in c1..c2 + 1 {
            result += max(r, c) % 3;
        }
    }

    result
}

#[test]
fn test_sum() {
    assert_eq!(sum(0, 2, 1, 4), 13);
    assert_eq!(sum(2, 2, 0, 7), 10);
    assert_eq!(sum(4, 8, 0, 5), 37);
}
