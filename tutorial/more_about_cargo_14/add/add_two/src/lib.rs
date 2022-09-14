pub fn add_two_function(a: usize) -> usize{
    a+2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_add_two() {
        let result = add_two_function(2);
        assert_eq!(result, 4);
    }
}
