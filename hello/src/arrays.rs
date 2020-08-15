use std::mem;

pub fn run(){
    let mut numbers: [i32; 4] = [1,2,3,4];

    //Re-assingn Value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single val
    println!("Single Value: {}", numbers[01]);

    //Get array length
    println!("Array Length {}",numbers.len());

    //array are stack allocated
    println!("Array occupied {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}