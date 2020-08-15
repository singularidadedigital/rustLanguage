//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run(){
    let name  = "Rodrigo Luciano";
    let mut age = 43;
    println!("My name is {} and i am {}", name, age);
    age = 42;
    println!("My name is {} and i am {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}",ID);

    //Assing multiple variables
    let (my_namme, my_age) = ("Rodrigo Luciano", 42);
    println!("{} is {}",my_namme,my_age);

}