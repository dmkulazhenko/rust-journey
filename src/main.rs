fn fib(n: usize, cache: &mut Vec<u128>) -> u128 {
    let n = n - 1;
    if cache.len() - 1 < n {
        for idx in cache.len()..n + 1 {
            cache.push(cache[idx - 1] + cache[idx - 2]);
        }
    }
    cache[n]
}

fn main() {
    let mut cache: Vec<u128> = vec![0, 1];

    println!("{}", fib(100, cache.as_mut()));
    println!("{}", fib(101, cache.as_mut()));
}
