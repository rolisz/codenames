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

static BLANK: String = String::new();

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
            Red => self.red_player.choose_words(&hint, &self.map.get_remaining_words()),
            Blue => self.blue_player.choose_words(&hint, &self.map.get_remaining_words()),
            _ => panic!("Unexpected stated"),
        };
        println!("Hint: {:?}", &hint);
        println!("Words: {:?}", words);
        for word in words {
            let cell: &mut Cell = self.map.get_cell(word);
            cell.visibility = true;
            let state = cell.color;
            println!("Found {} of type {}", word, state);
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    cells: [Cell; 25]
    // states: [State; 25],
    // pub words: Vec<&'static String>,
    // pub visibility: [bool; 25],
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    color: State,
    word: &'static String,
    visibility: bool
}

impl Map {
    pub fn new() -> Map {
        let mut map: [State; 25] = [Neutral, Neutral, Neutral, Neutral, Neutral, Neutral, Neutral,
        Red, Red, Red, Red, Red, Red, Red, Red, Red,  Blue, Blue, Blue, Blue, Blue, Blue, Blue,
                                   Blue, Bomb];
        let mut rng = thread_rng();
        map.shuffle(&mut rng);
        let words : Vec<&'static String> = CODENAME_WORDS.choose_multiple(&mut rng, 25).collect::<Vec<&'static String>>();

        // Couldn't find a nicer way to initialize an array with structs in Rust
        let mut cells: [Cell; 25] = [Cell{
            color: State::Neutral,
            word: &BLANK,
            visibility: false
        }; 25];
        for i in 0..25 {
            cells[i] = Cell{color: map[i], word: words[i], visibility: false};;
        }
        return Map{ cells};
    }

    pub fn get_remaining_words(&self) -> Vec<&'static String> {
        self.cells.iter().filter(|x| !x.visibility).map(|x| x.word).collect()
    }

    fn get_cell(&mut self, word: &String) -> &mut Cell {
        //  The word should be guaranteed to be from the words on the map
        self.cells.iter_mut().find(|x| x.word == word).unwrap()

    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..5 {
            write!(f, "{} {} {} {} {}\n",
                   self.cells[i*5].color, self.cells[i*5+1].color, self.cells[i*5+2].color, self.cells[i*5+3].color, self.cells[i*5+4].color);
        }
        let max_len = self.cells.iter().map(|x| x.word.len()).max().unwrap();
        for i in 0..5 {
            write!(f, "{:width$} {:width$} {:width$} {:width$} {:width$}\n",
                   self.cells[i*5].word, self.cells[i*5+1].word, self.cells[i*5+2].word, self.cells[i*5+3].word, self.cells[i*5+4].word, width=max_len);
        }
        Ok(())
    }
}

