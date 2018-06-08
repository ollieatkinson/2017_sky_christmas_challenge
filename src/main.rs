use std::time::Instant;

fn main() {
    
    let start = Instant::now();
    
    let input = env!("PRESENTS");

    let (index, _sum) = find_sum_of_divisors_over(input.parse::<u32>().unwrap() / 10);
    println!("{}", index);

    let elapsed = start.elapsed();
    println!("{}μs", elapsed.subsec_nanos() / 1000);
    
}

// searching

fn find_sum_of_divisors_over(n: u32) -> (u32, u32) {
    
    let step  = calculate_step(n);
    let start = calculate_lower_bound(n, step);
        
    OpenRangeStepIterator { 
        start: start, 
        step: step 
    }.map(|i| {
        (i, sum_of_divisors(i))
    }).find(|&(_i, sum)| {
        sum >= n
    }).unwrap()
    
}

struct Step {
    n : u32,
    by : u32
}

fn calculate_step(n: u32) -> u32 {
    [ 
        Step { n: 4,       by: 2 },
        Step { n: 42,      by: 6 },
        Step { n: 1872,    by: 12 },
        Step { n: 9920,    by: 60 },
        Step { n: 47520,   by: 420 },
        Step { n: 2257920, by: 840 } 
    ]
    .iter()
    .rev()
    .find(|&step| {
        step.n < n
    }).unwrap_or(&Step { n: 0, by: 1 }).by
}

fn calculate_lower_bound(n: u32, step: u32) -> u32 {
    let counter = n / 5;
    let lower_bound = counter - (counter % step);
    
    if lower_bound > 0 {
        return lower_bound;
    } else {
        return 1;
    }
    
}

struct OpenRangeStepIterator {
    start : u32,
    step : u32
}

impl Iterator for OpenRangeStepIterator {
    
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.start;
        self.start = current + self.step;
        Some(current)
    }
    
}

#[inline]
fn prime_factors(n: u32) -> PrimeIterator {
    PrimeIterator { next: n }
}

#[inline]
fn sum_of_divisors(n: u32) -> u32 {

    prime_factors(n).fold(1, |sum, factor| {
        sum * (factor.prime.pow(factor.count as u32 + 1) - 1) / (factor.prime - 1)
    })

}

// Primes

#[derive(Debug)]
#[derive(PartialEq)]
struct Prime {
    prime : u32,
    count : u8
}

struct PrimeIterator {
    next: u32
}

impl Iterator for PrimeIterator {
    
    type Item = Prime;
    
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        
        fn calculate_factor(mut n: u32) -> (Prime, u32) {
        
             let sqrt = (n as f64).sqrt() as u32;

             let mut i = 2;
        
             while n > 1 && i <= sqrt {
                
                 if n % i == 0 {
            
                     let mut current = Prime { 
                         prime: i, 
                         count: 1 
                     };
            
                     n /= i;
            
                     while n % i == 0  {
                         current.count += 1;
                         n /= i;
                     }
            
                     return (current, n);
            
                 }
                
                 i += if i == 2 { 1 } else { 2 }
        
             }
    
             return (Prime { prime: n, count: 1 }, 1);
        
         }
        
        if self.next == 1 {
            return None;
        }

        let (prime, remainder) = calculate_factor(self.next);
    
        self.next = remainder;
    
        Some(prime)

    }
    
}

// Test Cases - prime_factors

#[test]
fn calculate_step_10() {
    assert_eq!(calculate_step(10), 2);
}

#[test]
fn calculate_step_100() {
    assert_eq!(calculate_step(100), 6);
}

#[test]
fn calculate_step_5000() {
    assert_eq!(calculate_step(5000), 12);
}

#[test]
fn calculate_step_10000() {
    assert_eq!(calculate_step(10000), 60);
}

#[test]
fn calculate_step_2259999() {
    assert_eq!(calculate_step(2259999), 120);
}

#[test]
fn prime_factors_of_two() {
    assert_eq!(prime_factors(2).collect::<Vec<Prime>>(), [ Prime { prime: 2, count: 1 } ]);
}

#[test]
fn prime_factors_of_three() {
    assert_eq!(prime_factors(3).collect::<Vec<Prime>>(), [ Prime { prime: 3, count: 1 } ]);
}

#[test]
fn prime_factors_of_four() {
    assert_eq!(prime_factors(4).collect::<Vec<Prime>>(), [ Prime { prime: 2, count: 2 } ]);
}

#[test]
fn prime_factors_of_five() {
    assert_eq!(prime_factors(5).collect::<Vec<Prime>>(), [ Prime { prime: 5, count: 1 } ]);
}

#[test]
fn prime_factors_of_six() {
    assert_eq!(prime_factors(6).collect::<Vec<Prime>>(), [ Prime { prime: 2, count: 1 }, Prime { prime: 3, count: 1 } ]);
}

#[test]
fn prime_factors_of_seven() {
    assert_eq!(prime_factors(7).collect::<Vec<Prime>>(), [ Prime { prime: 7, count: 1 } ]);
}

#[test]
fn prime_factors_of_eight() {
    assert_eq!(prime_factors(8).collect::<Vec<Prime>>(), [ Prime { prime: 2, count: 3 } ]);
}

#[test]
fn prime_factors_of_nine() {
    assert_eq!(prime_factors(9).collect::<Vec<Prime>>(), [ Prime { prime: 3, count: 2 } ]);
}

#[test]
fn prime_factors_of_28() {
    assert_eq!(prime_factors(48).collect::<Vec<Prime>>(), [ Prime { prime: 2, count: 4 }, Prime { prime: 3, count: 1 } ]);
}

#[test]
fn prime_factors_of_191() {
    assert_eq!(prime_factors(191).collect::<Vec<Prime>>(), [ Prime { prime: 191, count: 1 } ]);
}

#[test]
fn prime_factors_of_200() {
    assert_eq!(prime_factors(200).collect::<Vec<Prime>>(), [ Prime { prime: 2, count: 3 }, Prime { prime: 5, count: 2 } ]);
}

#[test]
fn prime_factors_of_9000() {
    assert_eq!(prime_factors(9000).collect::<Vec<Prime>>(), [ Prime { prime: 2, count: 3 }, Prime { prime: 3, count: 2 }, Prime { prime: 5, count: 3 } ]);
}

// Test Cases - sum_of_divisors

#[test]
fn sum_of_divisors_of_18() {
    assert_eq!(sum_of_divisors(18), 39);
}

#[test]
fn sum_of_divisors_of_28() {
    assert_eq!(sum_of_divisors(28), 56);
}

#[test]
fn sum_of_divisors_of_200() {
    assert_eq!(sum_of_divisors(200), 465);
}

#[test]
fn sum_of_divisors_of_9000() {
    assert_eq!(sum_of_divisors(9_000), 30_420);
}

// Test Cases - find_sum_of_divisors_over

#[test]
fn find_sum_of_divisors_over_100() {
    assert_eq!(find_sum_of_divisors_over(100), (48, 124));
}

#[test]
fn find_sum_of_divisors_over_500() {
    assert_eq!(find_sum_of_divisors_over(500), (180, 546));
}

#[test]
fn find_sum_of_divisors_over_12345() {
    assert_eq!(find_sum_of_divisors_over(12_345), (3_600, 12_493));
}

#[test]
fn find_sum_of_divisors_over_5000000() {
    assert_eq!(find_sum_of_divisors_over(5_000_000), (1_164_240, 5_088_960));
}
