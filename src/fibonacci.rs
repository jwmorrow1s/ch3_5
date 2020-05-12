#[allow(dead_code)]
fn fibo(n: usize) -> u64 {
    let mut current = 0;
    let mut next = 1;

    for _ in 1..n {
        let tmp = current;
        current = next;
        next = tmp + next;
    }

    current
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 0);
        assert_eq!(fibo(2), 1);
        assert_eq!(fibo(3), 1);
        assert_eq!(fibo(4), 2);
        assert_eq!(fibo(5), 3);
        assert_eq!(fibo(6), 5);
        assert_eq!(fibo(7), 8);
        assert_eq!(fibo(8), 13);
        assert_eq!(fibo(30), 514229);
    }
}
