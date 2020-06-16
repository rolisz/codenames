mod map;
mod players;
mod game;

use crate::map::{Color, Cell, Map};
use crate::players::{RandomFieldOperative, RandomSpyMaster, Spymaster, FieldOperative, HumanCliSpymaster, HumanCliFieldOperative};
use crate::game::game;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("{}", Color::Black);
    println!("{}", Color::Blue);
    let cell = Cell{word: "WASHINGTON", color: Color::Red, revealed: true};
    println!("{}", cell);
    let cell = Cell{word: "WASHINGTON", color: Color::Gray, revealed: false};
    println!("{}", cell);

    let mut file = File::open("resources/wordlist").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let words = contents.lines().collect::<Vec<&str>>();

    let mut map = Map::new(&words);
    println!("{:?}", map);

    let mut sp = RandomSpyMaster::new(&words);
    //let mut sp = HumanCliSpymaster{};
    let mut fo = RandomFieldOperative::new();
    //let mut fo = HumanCliFieldOperative{};

    let hint = sp.give_hint(&map);
    println!("{:?}", &hint);
    println!("{:?}", fo.choose_words(&hint, &words));

    let result = game(&mut RandomSpyMaster::new(&words), &mut RandomFieldOperative::new(),
           &mut RandomSpyMaster::new(&words), &mut RandomFieldOperative::new(), &mut map);

    println!("The winner is {}", result);


}
