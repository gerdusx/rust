#[derive(Debug)] // Derive the Debug trait for the struct -> to use in println! macro
struct VideoGameCharacter { // Define a struct
    name: String,
    health: u32,
    experience: u32,
    level: u32
}

// Implementation block -> to define methods on the struct
impl VideoGameCharacter {
    // Associated function -> no self parameter
    fn new(name: String) -> VideoGameCharacter {
        VideoGameCharacter {
            name, // Field init shorthand -> same as name: name
            health: 100,
            experience: 0,
            level: 1
        }
    }

    // Method -> takes self as parameter
    fn level_up(&mut self) {
        self.level += 1;
    }

    // Method -> takes self as parameter
    fn increase_experience(&mut self, experience: u32) {
        self.experience += experience;
        if self.experience >= 100 {
            self.level_up();
            self.experience = 0;
        }
    }
}

fn main() {

    let mut fireball = VideoGameCharacter::new(String::from("Fireball")); // call associated function new

    println!("Fireball before exp: {:?}", fireball); 
    //Console output -> Fireball before exp: VideoGameCharacter { name: "Fireball", health: 100, experience: 0, level: 1 }

    fireball.increase_experience(120); // call increase_experience method with 120 experience

    println!("Fireball after exp: {:?}", fireball); 
    //Console output -> Fireball after exp: VideoGameCharacter { name: "Fireball", health: 100, experience: 0, level: 2 }


    let icebrick = VideoGameCharacter {
        name: String::from("Icebrick"),
        ..fireball // Struct update syntax -> copy all fields from fireball except name
    };

    println!("Icebrick {:?}", icebrick);
    //Console output -> icebrick VideoGameCharacter { name: "Icebrick", health: 100, experience: 0, level: 2 }
}
