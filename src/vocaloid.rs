#[derive(Debug)]
struct Vocaloid {
    name: String,
    age: u8,
    songs: Vec<String>,
}

impl Vocaloid {
    fn new(name: String, age: u8, songs: Vec<String>) -> Self {
        Self { name, age, songs }
    }

    fn info(&self) {
        println!("vocaloid info: {:#?}", &self);
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn say_hello(&self) -> String {
        format!("Hello {}!", self.name)
    }
}

pub fn run() {
    let songs = vec![
        String::from("The Vampire"),
        String::from("Cinderella"),
        String::from("Ruma"),
    ];
    let miku = Vocaloid::new(String::from("Hatsune Miku"), 16, songs);
    miku.info();
    println!("{}", miku.is_adult());
    println!("{}", miku.say_hello());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_not_adult() {
        let miku = Vocaloid::new(String::from("Hatsune Miku"), 16, vec![String::from("Ruma")]);
        assert!(!miku.is_adult());
    }

    #[test]
    fn test_is_adult() {
        let kaito = Vocaloid::new(String::from("KAITO"), 26, vec![String::from("Ohed")]);
        assert!(kaito.is_adult());
    }

    #[test]
    fn test_say_hello_contains_name() {
        let miku = Vocaloid::new(String::from("Hatsune Miku"), 16, vec![String::from("Ruma")]);
        assert!(miku.say_hello().contains("Hatsune Miku"));
    }

    #[test]
    fn test_say_hello_does_not_contain_different_name() {
        let miku = Vocaloid::new(String::from("Hatsune Miku"), 16, vec![String::from("Ruma")]);
        assert!(!miku.say_hello().contains("KAITO"));
    }
}
