use core::fmt;
use crate::map::State::{Neutral, Red, Blue, Bomb};
use std::fs::File;
use std::io::Read;
use crate::players::Spymaster;
use crate::players::FieldOperatives;
use crate::map::Map;
use crate::map::State;
use crate::map::Cell;
use crate::game::RoundResult::FoundBomb;
use crate::game::RoundResult::FoundEnemyAgents;


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
    pub red_spy_master: &'a mut Spymaster,
    pub red_field_operatives: &'a mut FieldOperatives,
    pub blue_spy_master: &'a mut Spymaster,
    pub blue_field_operatives: &'a mut FieldOperatives,
    pub is_over:  bool,
    pub current_player: State,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RoundResult {
    FoundEnemyAgents(u8),
    FoundBomb
}

impl<'a> Game<'a> {

    pub fn new(map : &'a mut Map, red_spy_master: &'a mut Spymaster,
               red_fo: &'a mut FieldOperatives,
               blue_spy_master: &'a mut Spymaster, blue_fo: &'a mut FieldOperatives) -> Game<'a> {
        Game{map, red_spy_master, red_field_operatives: red_fo, blue_spy_master,
             blue_field_operatives: blue_fo, is_over: false, current_player: Red}
    }
    pub fn tick(&mut  self) -> RoundResult  {
        // ??? why can't use curr_player like var?
        let hint = match self.current_player {
            Red => self.red_spy_master.give_hint(self.map),
            Blue => self.blue_spy_master.give_hint(self.map),
            _ => panic!("Unexpected state"),
        };

        let words = match self.current_player {
            Red => self.red_field_operatives.choose_words(&hint, &self.map.get_remaining_words()),
            Blue => self.blue_field_operatives.choose_words(&hint, &self.map.get_remaining_words()),
            _ => panic!("Unexpected state"),
        };
        println!("Hint: {:?}", &hint);
        println!("Words: {:?}", words);
        let mut nr_found: u8 = 0;
        for word in words {
            let cell: &mut Cell = self.map.get_cell(word);
            cell.visibility = true;
            let state = cell.color;
            println!("Found {} of type {}", word, state);
            match state {
                Bomb => {
                    return FoundBomb;
                    break;
                },
                Neutral => {
                    break;
                },
                Red => {
                    if self.current_player == Blue {
                        println!("Correct! You caught an agent of the enemy");
                        nr_found += 1;
                    } else{
                        break;
                    }
                },
                Blue => {
                    if self.current_player == Red {
                        println!("Correct! You caught an agent of the enemy");
                        nr_found += 1;
                    } else{
                        break;
                    }
                }
            }
        }
        return FoundEnemyAgents(nr_found);
    }

    pub fn swap_player(& mut self) {
        match self.current_player {
            Red => self.current_player = Blue,
            Blue => self.current_player = Red,
            _ => panic!("Wrong player state")
        };
    }
}


