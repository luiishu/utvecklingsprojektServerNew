pub fn hello() {
    println!("Hello from new test module!");
}

pub fn is_even(number: i64) -> bool {
    number % 2 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    //#[test]
    fn even() {
        assert_eq!(true, is_even(4));
    }
    
    //#[test]
    fn odd() {
        assert_eq!(false, is_even(19));
    }
}