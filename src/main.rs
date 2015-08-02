fn is_prime(n: i32) -> bool {
    if n < 2 { return false; }
    let mut i:i32 = 2i32;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 1;
    }
    return true;
}

fn main() {
    for i in 1..30 {
        println!("{} is prime {}", i, is_prime(i));
    }
}
