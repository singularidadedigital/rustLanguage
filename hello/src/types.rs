/*
Primitive tyoes
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 
(number of bits they take in memomy)
floats: f32, f64
Booblean (bool)
Characters (char)
Tuples
Arrays
*/

/*Rust is statical typed language, which means that it must know the types of all
variables at complile time, however, the compiler can usualy infer what type we want
to use based on the value and how we use it.*/

pub fn run(){
    //Degault is "i32"
    let _x = 1;

    //Default is "g64"
    let _y = 2.5;

    //Add explicit type
    let _z: i64 = 454545454554;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active:bool = true;

    //get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = "a";
    let face = '\u{2622}';
    println!("{:?}", (_x, _y, _z, is_active, is_greater, a1, face));


}