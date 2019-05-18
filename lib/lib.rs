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
            if prime * prime > position {
                break;
            }
            if position % prime == 0 {
                is_position_prime = false;
                break;
            }
        }

        if is_position_prime {
            if size == (n as usize) - 1 {
                return position.to_string();
            } else {
                primes.push(position);
            }
        }

        position += 2;
    }
}