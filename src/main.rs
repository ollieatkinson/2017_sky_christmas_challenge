use std::env;
use std::time::Instant;

#[inline]
fn sum_of_divisors(n: i32) -> i32 {

    let sqrt: i32 = (n as f32).sqrt() as i32 + 1;
    
    (2..sqrt).filter(|i| 
        n % i == 0
    ).fold(1 + n, |sum, i| {
        let divisor = n / i;
        sum + i + ((divisor != i) as i32) * divisor
    })
    
}

fn find_sum_of_divisors_over(n: i32) -> (i32, i32) {
    
    (1..).map(|i| {
        (i, sum_of_divisors(i))
    }).find(|&(_i, sum)| {
        sum >= n
    }).unwrap()
    
}

fn main() {
    
    let start = Instant::now();
    
    if let Some(arg1) = env::args().nth(1) {
        let (index, sum) = find_sum_of_divisors_over(arg1.parse::<i32>().unwrap() / 10);
        println!("DESK={}\nPRESENTS={}", index, sum * 10);
    } else {
        println!("Error! please specify input.");
    }
    
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
    
}
