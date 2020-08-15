//Vectors are resizeble arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> =vec![1,2,3,4,12,90];

    //Re-assingn Value
    numbers[2] = 20;
    numbers[3] = 32;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Po off last value
    
    numbers.pop();

    println!("{:?}", numbers);

    //Get single val
    println!("Single Value: {}", numbers[01]);

    //Get array length
    println!("Vector Length {}",numbers.len());

    //array are stack allocated
    println!("Vector occupied {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}", slice);

    //Loop througth vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //Loop & mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}