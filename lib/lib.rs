use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn nth_prime_number(n: i32) -> String {
    let mut primes: Vec<i64> = vec![2];
    let mut position: i64 = 3;
    if n <= 2 {
        return (n + 1).to_string();
    }
    let mut position: i64 = 3;
    loop {
        let mut size = primes.len();
        let mut is_position_prime: bool = true;
        for prime in primes.iter() {
            // No need to check numbers higher than the square root.
            if prime * prime > position {
                break;
            }
            // Only need to check if number is divisible by other primes (every number has
            // prime denominators).
            if position % prime == 0 {
                is_position_prime = false;
                break;
            }
        }
        if is_position_prime {
            if size == (n as usize) - 1 {
                // Don't bother pushing it to the stack of prime numbers, it's the one we want so
                // return it straight away.
                return position.to_string();
            } else {
                // Save for checking subsequent numbers.
                primes.push(position);
            }
        }
        // We started on an odd number, there's absolutely no point checking even numbers so
        // advance to the next odd number.
        position += 2;
    }
}