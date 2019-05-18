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
pub fn nth_prime_number(n: i32) -> i32 {
    let mut primes: Vec<i32> = vec![2];
    let mut position: i32 = 3;

    if n <= 2 {
        return (n as i32) + 1;
    }

    let mut position: i32 = 3;

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
                return position;
            } else {
                primes.push(position);
            }
        }

        position += 2;
    }
}