#[derive(PartialEq, Debug)]
struct Shoe{
    size: u32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>,size: u32)->Vec<Shoe>{
    
    

    // into_iter takes ownership of iterator
    shoes.into_iter().filter(|x| x.size == size).collect()
    //filter is iterator adapter like "map()" and it only
    //returns values that satisfy a codition

}


#[cfg(test)]
mod test{
    #[test]
    fn iterator_sum(){
        let v1 = vec![1,2,3];
        let v1_iter = v1.iter();

        // sum takes ownership of v1_iter iterator
        let total: i32 = v1_iter.sum();
        
        assert_eq!(total,6);
    }

    // use super to access 

    use super::*;
    #[test]
    fn filter_by_size(){
        let my_vec_of_shoes = vec![
            Shoe{
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 8,
                style: String::from("sandal"),
            },
            Shoe{
                size: 10,
                style: String::from("flip-flops"),
            },
        ];

        let matched_selection_of_shoes = shoe_in_size(my_vec_of_shoes,10);

        assert_eq!(
            vec![

            Shoe{
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 10,
                style: String::from("flip-flops"),
            },
            ],
            matched_selection_of_shoes
            );

    }
}
