struct ProductOfNumbers {
    num_list: Vec<i32>,
}

// TODO: seems like there are some marjor gains to be made here still

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        return ProductOfNumbers{num_list: Vec::new()}
    }
    
    fn add(&mut self, num: i32) {
       self.num_list.push(num); 
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let mut count = k;
       self.num_list.iter().rev().filter(|p| {
           if count > 0 {
                count -= 1;
               true
           } else {
               false
           }
       }).product() 
    }
}
