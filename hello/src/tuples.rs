//Tuples group together values of diferent types
//Max 12 elements

pub fn run(){
    let person:(&str, &str, i8) = ("Rodrigo Luciano", "Brasil", 42);
    println!("{} is from {} and is {} age", person.0, person.1, person.2);

}