pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// we want to declare this module for tests within the same file as the code for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/*
    Test private functions
*/
pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

// our private function
fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
