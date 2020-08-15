pub fn run(){
    //Print ro console
    println!("Olá voce está imprimindo via file print.rs");
    
    //Basic formating
    println!("{} is from {}","Rodrigo Luciano","Brasil");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Rodrigo Luciano","Brasil", "code");

    //Named Arguments
    println!("{name} likes to play {activity}",
    name = "Rodrigo",
    activity = "Soccer"
    );
    // Placeholder traists
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}",(12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}",10+10)


}