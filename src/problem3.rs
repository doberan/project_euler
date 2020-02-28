//! https://projecteuler.net/problem=3
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! 
//! What is the largest prime factor of the number 600851475143 ?

pub struct Problem;

impl Problem {
    pub fn exec() -> i64 {
        let max = Problem::get_max_prime_facter(13195);
        println!("max prime facter is {}", max);
        max
    }

    fn get_max_prime_facter(x: i64) -> i64 {
        let max = 0;
        println!("x = {}", x);
        for i in (1..x).rev() {
            println!("i = {}", i);
            let max = if x % i == 0 { i } else { 0 };
            println!("max = {}", max);
            if max != 0 { break; }
        }
        max
    }
}