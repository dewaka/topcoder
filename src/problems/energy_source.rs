// TODO: EnergySource - https://community.topcoder.com/stat?c=problem_statement&pm=15107

pub fn count_different_sources(num: i64) -> (i64, i64) {
    unimplemented!();
}

#[test]
fn test_count_different_sources() {
    let test_cases = vec![
        (3, (2, 4)),
        (10, (9, 103)),
        (1, (1, 1)),
        (90, (98014, 45465390986863499)),
        (48, (6978, 9697161469)),
    ];

    for &(i, r) in &test_cases {
        assert_eq!(count_different_sources(i), r);
    }
}
