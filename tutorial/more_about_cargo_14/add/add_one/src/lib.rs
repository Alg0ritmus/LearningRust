use rand;


// make sure that this is public function if u want
// to call it from external package
pub fn add_one_function(a: usize) -> usize{
    a+1
        
}

pub fn add_random_function(a: usize) -> usize{
    a+rand::random::<usize>()
        
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = add_one_function(2);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_random_add() {
        let result = add_random_function(2);
        assert!(result >= 2);
    }
}
