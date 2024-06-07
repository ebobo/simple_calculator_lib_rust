pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}

pub fn mul(x: i32, y: i32) -> i32 {
    x * y
}

pub fn div(x: i32, y: i32) -> i32 {
    x / y
}

#[cfg(test)] // this attribute tells the Rust compiler to only compiles when running tests
mod tests {
    use super::*;

    #[test] // this attribute that should be run by the test harness provided by Cargo and Rust’s standard library.
    fn add_test() {
        let result = add(2, -2);
        assert_eq!(result, 0);
    }
    #[test]
    fn sub_test() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn mul_test() {
        let result = mul(3, 5);
        assert_eq!(result, 15);
    }

    #[test]
    fn div_test() {
        let result = div(10, 2);
        assert_eq!(result, 5);
    }
}
