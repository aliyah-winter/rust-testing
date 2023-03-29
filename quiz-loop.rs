fn main() {
    let arr1: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arr2: [u32; 3] = [1, 2, 3];
    let mut count: u32 = 0;

    // outer loop
    for outer_element in arr1 {
        println!("Loop count: {}", outer_element);
        
        // inner loop
            for _inner_element in arr2 {
                if count < 15 {
                    count += 1;
                    println!("{}", count);
                }
                
                else if count >= 15 {
                    break;
                }
            }   
    }
}