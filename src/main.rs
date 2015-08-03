fn is_prime(n: i32) -> bool {
    if n < 2 { return false; }
    let mut i:i32 = 2i32;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 1;
    }
    return true;
}

fn prime_list(n: i32) -> Vec<i32>{
    let mut dp = vec![false; (n+1) as usize];
    let mut prime = vec![];
    let mut i = 2;
    while i <= n {
        if dp[i as usize] == false {
            prime.push(i);
        }
        let mut j = i * 2;
        while j <= n {
            dp[j as usize] = true;
            j += i;
        }
        i += 1;
    }
    return prime;
}

fn main() {
    for n in 1..10 {
        println!("{} is prime = {}", n, is_prime(n));
    }

    let pl = prime_list(10);
    for p in pl {
        println!("{}", p);
    }
}
