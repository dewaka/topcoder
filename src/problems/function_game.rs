// https://community.topcoder.com/stat?c=problem_statement&pm=15051
fn gcd_rec(a: i32, b: i32) -> i32 {
    if a == b {
        a
    } else if a > b {
        gcd(a - b, b)
    } else {
        gcd(a, b - a)
    }
}

fn gcd_iter(mut a: i32, mut b: i32) -> i32 {
    loop {
        if a == b {
            return a;
        } else if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    gcd_iter(a, b)
}

fn euler_totient_fast(mut n: i32) -> i32 {
    let mut result = n;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            result -= result / i;
        }

        while n % i == 0 {
            n /= i;
        }

        i += 1;
    }

    if n > 1 {
        result -= result / n;
    }

    result
}

fn euler_totient_slow(n: i32) -> i32 {
    let mut count = 0;

    for i in 1..n + 1 {
        if gcd(n, i) == 1 {
            count += 1;
        }
    }

    count
}

fn euler_totient(n: i32) -> i32 {
    euler_totient_fast(n)
}

const N: i32 = 1_000_000_007;

fn find_value_naive(m: i32, n: i32) -> i32 {
    let mut sum = 0;
    for x in 1..m + 1 {
        for r in 1..n + 1 {
            sum += euler_totient(x.pow(r as u32));
        }
    }

    sum % N
}

pub fn find_value(m: i32, n: i32) -> i32 {
    find_value_naive(m, n)
}

#[test]
fn test_euler_totient() {
    assert_eq!(euler_totient(3), 2);
    assert_eq!(euler_totient(9), 6);
    assert_eq!(euler_totient(18), 6);
}

#[test]
fn test_find_value() {
    assert_eq!(find_value(1, 3), 3);
    assert_eq!(find_value(3, 1), 4);
    assert_eq!(find_value(3, 2), 13);
}
