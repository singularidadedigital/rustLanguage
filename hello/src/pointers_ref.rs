//Reference Pointers - Point to a resource in memory

pub fn run(){
    //Primitive Array
    let arr1 = [1,2,3];
    let _arr2 = arr1;
    //println!("Valuers: {:?}", (arr1, arr2));

    //With non-primitives, if you assing another variable to a piece of data
    //the first variable will no longer hold tha value. You'll use a reference
    //(&) to point to the resource.

    //Vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, vec2));
}