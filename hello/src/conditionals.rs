// Conditionals - Used to check the condition of something and act on the result

pub fn run(){
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    //If/Else
    if age>= 21 && check_id || knows_person_of_age{
        println!("Garçon: O que voce quer beber??");
    }else if age < 22 && check_id{
        println!("Garçon: Desculpe voce terá de ir, Menores de 22 nao podem beber!!!");
    }else{
        println!("Garçon: Eu preciso ver seus documentos (ID)");
    }

    //Shorthand If
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of Age: {}", is_of_age);

}