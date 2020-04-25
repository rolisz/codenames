use core::fmt;
use self::State::{Neutral, Red, Blue, Bomb};
use rand::prelude::*;
use crate::players::Player;
use std::fs::File;
use std::io::Read;


lazy_static! {
    pub static ref CODENAME_WORDS: Vec<String> = {
        let mut file = File::open("resources/wordlist").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        contents.lines().map(String::from).collect::<Vec<String>>()
    };
}

pub struct Game<'a> {
    pub map: Map<'a>,
    pub red_player: Box<dyn Player>,
    pub blue_player: Box<dyn Player>,
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum State {
    Neutral,
    Red,
    Blue,
    Bomb
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            Neutral => 'N',
            Red => 'R',
            Blue => 'B',
            Bomb => 'X'
        };
        write!(f, "{}", char)
    }
}

#[derive(Debug)]
pub struct Map<'a> {
    states: [State; 25],
    words: Vec<&'a String>,
}

impl<'a> Map<'a> {
    pub fn new() -> Map<'a> {
        let mut map: [State; 25] = [Neutral, Neutral, Neutral, Neutral, Neutral, Neutral, Neutral,
        Red, Red, Red, Red, Red, Red, Red, Red, Red,  Blue, Blue, Blue, Blue, Blue, Blue, Blue,
                                   Blue, Bomb];
        let mut rng = thread_rng();
        map.shuffle(&mut rng);
        let words = CODENAME_WORDS.choose_multiple(&mut rng, 25).collect::<Vec<&String>>();
        return Map{ states: map, words };
    }

}

impl fmt::Display for Map<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..5 {
            write!(f, "{} {} {} {} {}\n", self.states[i*5], self.states[i*5+1], self.states[i*5+2], self.states[i*5+3], self.states[i*5+4]);
        }
        let max_len = self.words.iter().map(|x| x.len()).max().unwrap();
        for i in 0..5 {
            write!(f, "{:width$} {:width$} {:width$} {:width$} {:width$}\n", self.words[i*5], self.words[i*5+1], self.words[i*5+2], self.words[i*5+3], self.words[i*5+4], width=max_len);
        }
        Ok(())
    }
}

