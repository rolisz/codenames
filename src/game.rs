use core::fmt;
use self::State::{Neutral, Red, Blue, Bomb};
use rand::prelude::*;
use crate::players::Player;
use std::fs::File;
use std::io::Read;

//pub type Codename = ;

lazy_static! {
    pub static ref CODENAME_WORDS: Vec<String> = {
        let mut file = File::open("resources/wordlist").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        contents.lines().map(String::from).collect::<Vec<String>>()
    };
}

pub struct Game<'a> {
    pub map: &'a mut Map,
    pub red_player: &'a mut Player,
    pub blue_player: &'a mut Player,
    pub is_over:  bool,
    pub current_player: State,
}

impl<'a> Game<'a> {

    pub fn new(map : &'a mut Map, red_player: &'a mut Player, blue_player: &'a mut Player) -> Game<'a> {
        Game{map, red_player, blue_player, is_over: false, current_player: Red}
    }
    pub fn tick(&'a mut  self)  {
        // ??? why can't use curr_player like var?
        let hint = match self.current_player {
            Red => self.red_player.give_hint(),
            Blue => self.blue_player.give_hint(),
            _ => panic!("Unexpected state"),
        };

        let words = match self.current_player {
            Red => self.red_player.choose_words(&hint, &self.map.words),
            Blue => self.blue_player.choose_words(&hint, &self.map.words),
            _ => panic!("Unexpected stated"),
        };
        println!("Hint: {:?}", &hint);
        println!("Words: {:?}", words);
        for word in words {
            //  The word should be guaranteed to be from the words on the map
            let i = self.map.words.iter().position(|&x| x == word).unwrap() ;
            self.map.visibility[i] = true;
            let state = &self.map.states[i];
            println!("Found {} at {} of type {}", word, i, self.map.states[i]);
            match state {
                Bomb => {
                    self.is_over = true;
                    break;
                },
                Neutral => {
                    break;
                },
                Red => {
                    if self.current_player == Blue {
                        ();
                    } else{
                        println!("Missed!");
                        break;
                    }
                },
                Blue => {
                    if self.current_player == Red {
                        ();
                    } else{
                        println!("Missed!");
                        break;
                    }
                }
            }
        }
        // check how many are leftfr
    }
}
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum State {
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
pub struct Map {
    states: [State; 25],
    pub words: Vec<&'static String>,
    pub visibility: [bool; 25],
}

impl Map {
    pub fn new() -> Map {
        let mut map: [State; 25] = [Neutral, Neutral, Neutral, Neutral, Neutral, Neutral, Neutral,
        Red, Red, Red, Red, Red, Red, Red, Red, Red,  Blue, Blue, Blue, Blue, Blue, Blue, Blue,
                                   Blue, Bomb];
        let mut rng = thread_rng();
        map.shuffle(&mut rng);
        let words : Vec<&'static String> = CODENAME_WORDS.choose_multiple(&mut rng, 25).collect::<Vec<&'static String>>();

        return Map{ states: map, words, visibility: [false; 25] };
    }

}

impl fmt::Display for Map {
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

