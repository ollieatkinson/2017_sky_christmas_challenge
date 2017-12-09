use std::env;
use std::vec::Vec;
use std::time::Instant;

#[derive(Debug)]
#[derive(PartialEq)]
struct Prime {
	prime : u32,
	count : u8
}

fn find_sum_of_divisors_over(n: u32) -> (u32, u32) {
    
    (1..).map(|i| {
        (i, sum_of_divisors(i))
    }).find(|&(_i, sum)| {
        sum >= n
    }).unwrap()
    
}

#[inline]
fn sum_of_divisors(n: u32) -> u32 {

    prime_factors(n).iter().fold(1, |sum, factor| {
        sum * (factor.prime.pow(factor.count as u32 + 1) - 1) / (factor.prime - 1)
    })

}

fn prime_factors(mut n: u32) -> Vec<Prime> {
    
	let mut primes: Vec<Prime> = vec![];
    
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
				current.count = current.count  +1;
				n /= i;
			}
            
			primes.push(current);
			
		}
        
		i += 1;
	}
    
	if n > 1 {
	    primes.push(Prime { prime: n, count: 1 });
	}
    
	return primes;
}

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

#[test]
fn prime_factors_of_two() {
    assert_eq!(prime_factors(2), [ Prime { prime: 2, count: 1 } ]);
}

#[test]
fn prime_factors_of_three() {
    assert_eq!(prime_factors(3), [ Prime { prime: 3, count: 1 } ]);
}

#[test]
fn prime_factors_of_four() {
    assert_eq!(prime_factors(4), [ Prime { prime: 2, count: 2 } ]);
}

#[test]
fn prime_factors_of_five() {
    assert_eq!(prime_factors(5), [ Prime { prime: 5, count: 1 } ]);
}

#[test]
fn prime_factors_of_six() {
    assert_eq!(prime_factors(6), [ Prime { prime: 2, count: 1 }, Prime { prime: 3, count: 1 } ]);
}

#[test]
fn prime_factors_of_seven() {
    assert_eq!(prime_factors(7), [ Prime { prime: 7, count: 1 } ]);
}

#[test]
fn prime_factors_of_eight() {
    assert_eq!(prime_factors(8), [ Prime { prime: 2, count: 3 } ]);
}

#[test]
fn prime_factors_of_nine() {
    assert_eq!(prime_factors(9), [ Prime { prime: 3, count: 2 } ]);
}

#[test]
fn prime_factors_of_28() {
    assert_eq!(prime_factors(48), [ Prime { prime: 2, count: 4 }, Prime { prime: 3, count: 1 } ]);
}

#[test]
fn prime_factors_of_191() {
    assert_eq!(prime_factors(191), [ Prime { prime: 191, count: 1 } ]);
}

#[test]
fn prime_factors_of_200() {
    assert_eq!(prime_factors(200), [ Prime { prime: 2, count: 3 }, Prime { prime: 5, count: 2 } ]);
}

#[test]
fn prime_factors_of_9000() {
    assert_eq!(prime_factors(9000), [ Prime { prime: 2, count: 3 }, Prime { prime: 3, count: 2 }, Prime { prime: 5, count: 3 } ]);
}