// https://primesieve.org/segmented_sieve.html

#[derive(Debug)]
pub struct Sieve {
    pub primes: Vec<i64>,
}

const L1D_CACHE_SIZE: i64 = 32768;

impl Sieve {
    pub fn new(limit: i64) -> Self {
        Sieve::segmented_sieve(limit)
    }

    fn segmented_sieve(limit: i64) -> Self {
        let sqrt = (limit as f64).sqrt() as i64;
        let segment_size = sqrt.max(L1D_CACHE_SIZE);
        let mut count = if limit < 2 { 0 } else { 1 };

        // we sieve primes >= 3
        let mut i: i64 = 3;
        let mut n: i64 = 3;
        let mut s: i64 = 3;

        let mut sieve: Vec<bool> = vec![true; segment_size as usize];
        let mut is_prime: Vec<bool> = vec![true; (sqrt + 1) as usize];
        let mut primes: Vec<i64> = vec![];
        let mut multiples: Vec<i64> = vec![];

        for low in (0..limit + 1).step_by(segment_size as usize) {
            sieve.iter_mut().for_each(|x| *x = true);

            // current segment = [low, high]
            let mut high = low + segment_size - 1;
            high = high.min(limit);

            // generate sieving primes using simple sieve of Eratosthenes
            while i * i <= high {
                if is_prime[i as usize] {
                    let mut j = i * i;
                    while j <= sqrt {
                        is_prime[j as usize] = false;
                        j += i;
                    }
                }

                i += 2;
            }

            // initialise sieving primes for segmented sieve
            while s * s <= high {
                if is_prime[s as usize] {
                    // println!("Pushing prime: {}", s);
                    primes.push(s);
                    multiples.push(s * s - low);
                }
                s += 2;
            }

            // sieve the current segment
            for k in 0..primes.len() {
                let mut j = multiples[k];

                let t = primes[k as usize] * 2;
                while j < segment_size {
                    sieve[j as usize] = false;
                    j += t;
                }
            }

            while n <= high {
                if sieve[(n - low) as usize] {
                    count += 1;
                }
                n += 2;
            }
        }

        println!("{} primes found.", count);

        Sieve { primes }
    }
}
