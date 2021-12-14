// iterators4.rs


pub fn factorial(num: u64) -> u64 {
    let mut n = num;
    let mut f = n.clone();

    loop {
        if n == 1 {
            break;
        }
        println!("N is {}", n);
        n -= 1;
        f *= n;
        println!("f is {}", f);
    }
    println!("final f is {}", f);
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
