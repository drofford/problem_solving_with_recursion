pub fn fibonacci(n: i64) -> i64 {
    if n > 1i64 {
        return fibonacci(n - 1) + fibonacci(n - 2);
    } else {
        return n;
    }
}
