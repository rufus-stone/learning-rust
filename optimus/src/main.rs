#[derive(Debug, Default)]
struct Optimus {
    counter: u64,
}

impl Optimus {
    /// Return the next prime number
    fn next_prime(&mut self) -> u64 {
        self.counter += 1;

        while !Self::is_prime(self.counter) {
            self.counter += 1;
        }

        self.counter
    }

    /// Use the 6k+1 method to check if a given number is prime
    fn is_prime(n: u64) -> bool {
        if n <= 3 {
            n > 1
        } else if n % 2 == 0 || n % 3 == 0 {
            false
        } else {
            let mut i = 5u64;
            while i.pow(2) <= n {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                } else {
                    i += 6
                }
            }
            true
        }
    }
}

impl Iterator for Optimus {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_prime())
    }
}

fn main() {
    let optimus = Optimus::default();

    let start = std::time::Instant::now();

    let _: Vec<u64> = optimus.into_iter().take(1_000_000).collect();

    let elapsed = std::time::Instant::now().duration_since(start);
    println!("Elapsed: {:?}", elapsed);
}
