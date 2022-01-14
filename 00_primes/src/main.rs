fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }
    
    let mut i: i64 = 2;
    while i < x {
        if x % i == 0 {
            return false;
        }
        
        i += 1;
    }
    
    return true;
}

fn main() {
    const VAL: i64 = 5;
    println!("{} is prime? {}", VAL, is_prime(VAL));

    const MAX: i64 = 100;
    println!("Printing primes from 1 to {}", MAX);

    let mut i: i64 = 1;
    while i <= MAX {
        println!("{} is prime ? {}", i, is_prime(i));
        i += 1;
    }
}
