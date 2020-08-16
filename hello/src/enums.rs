//Enums are types which have a few definitive values
enum Movement {
    //Variants
    UP ,DOWN, LEFT, RIGHT
}

fn move_avatar(m: Movement){
    //Perform action depending on info
    match m {
        Movement::UP => println!("Avatar moving UP"),
        Movement::DOWN => println!("Avatar moving DOWN"),
        Movement::LEFT => println!("Avatar moving LEFT"),
        Movement::RIGHT => println!("Avatar moving RIGHT")
    }
}

pub fn run(){
    let avatar1 = Movement::UP;
    let avatar2 = Movement::DOWN;
    let avatar3 = Movement::LEFT;
    let avatar4 = Movement::RIGHT;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}