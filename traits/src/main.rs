struct Person {
    name: String,
    age: u8
}

trait HasVoiceBox {
    // speak
    fn speak(&self);

    //check if can speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn can_speak(&self) -> bool{
        if self.age > 0 {
            return true;
        } return false;
    }
}

fn main() {
    let person = Person {
        name: String::from("Bob"),
        age: 41
    };

    println!("Can {} speak? {}", person.name, person.can_speak());
}
