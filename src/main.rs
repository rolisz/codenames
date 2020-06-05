mod map;
mod players;

use crate::map::{Color, Cell, Map};
use crate::players::{RandomFieldOperative, RandomSpyMaster, Spymaster, FieldOperative, HumanCliSpymaster, HumanCliFieldOperative};

fn main() {
    println!("{}", Color::Black);
    println!("{}", Color::Blue);
    let cell = Cell{word: "WASHINGTON", color: Color::Red, revealed: true};
    println!("{}", cell);
    let cell = Cell{word: "WASHINGTON", color: Color::Gray, revealed: false};
    println!("{}", cell);

    let words = vec!["RAY", "REVOLUTION", "RING", "ROBIN", "ROBOT", "ROCK",
"ROME", "ROOT", "ROSE", "ROULETTE", "ROUND", "ROW", "RULER", "SATELLITE", "SATURN",
"SCALE", "SCHOOL", "SCIENTIST", "SCORPION", "SCREEN", "SCUBA DIVER", "SEAL",
"SERVER", "SHADOW", "SHAKESPEARE", "SHARK", "SHIP", "SHOE", "SHOP", "SHOT", "SINK"];
    let map = Map::new(&words);
    println!("{}", map);

    let mut sp = RandomSpyMaster::new(&words);
    let mut sp = HumanCliSpymaster{};
    let mut fo = RandomFieldOperative::new();
    let mut fo = HumanCliFieldOperative{};

    let hint = sp.give_hint(&map);
    println!("{:?}", &hint);
    println!("{:?}", fo.choose_words(&hint, &words));

}
