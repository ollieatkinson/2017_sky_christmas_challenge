use std::env;
use std::time::Instant;

fn main() {
    
    let start = Instant::now();

    if let Some(arg1) = env::args().nth(1) {
        let (index, sum) = find_sum_of_divisors_over(arg1.parse::<u32>().unwrap() / 10);
        println!("DESK={}\nPRESENTS={}", index, sum * 10);
    } else {
        println!("Error! please specify input.");
    }

    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
    
}

// searching

fn find_sum_of_divisors_over(n: u32) -> (u32, u32) {
    
    (1..).map(|i| {
        (i, sum_of_divisors(i))
    }).find(|&(_i, sum)| {
        sum >= n
    }).unwrap()
    
}

fn prime_factors(n: u32) -> PrimeIterator {
    PrimeIterator { 
        next: n
    }
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
        
                if i == 2 {
                    i += 1;
                } else {
                    i += 2;
                }
        
        	}
    
        	return (Prime { prime: n, count: 1 }, 1);
        
        }
        
        if self.next == 1 {
            return None;
        }

        let (prime, remainder) = calculate_factor(self.next);
    
        self.next = remainder;
    
        return Some(prime);  

    }
    
}

// Test Cases - prime_factors

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
    assert_eq!(sum_of_divisors(9000), 30420);
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
    assert_eq!(find_sum_of_divisors_over(12345), (3600, 12493));
}

#[test]
fn find_sum_of_divisors_over_5000000() {
    assert_eq!(find_sum_of_divisors_over(5000000), (1164240, 5088960));
}