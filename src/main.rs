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
}

fn main() {
    let songs = vec![
        String::from("The Vampire"),
        String::from("Cinderella"),
        String::from("Ruma"),
    ];
    let miku = Vocaloid::new(String::from("Hatsune Miku"), 16, songs);

    println!("info: {:#?}", miku);
    miku.info();
}
