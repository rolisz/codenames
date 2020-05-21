use core::fmt;
use crate::map::State::{Neutral, Red, Blue, Bomb};
use rand::prelude::*;
use crate::game::{CODENAME_WORDS, Game};


static BLANK: String = String::new();

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
    pub color: State,
    pub word: &'static String,
    pub visibility: bool
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

    pub fn get_remaining_words_of_color(&self, color: State) -> Vec<&'static String> {
        self.cells.iter().filter(|x| !x.visibility)
            .filter(|x| x.color == color).map(|x| x.word).collect()

    }

    pub fn get_cell(&mut self, word: &String) -> &mut Cell {
        //  The word should be guaranteed to be from the words on the map
        self.cells.iter_mut().find(|x| x.word == word).unwrap()
    }

    pub fn is_game_finished(&self) -> bool {
        if self.unturned_cells_of_color(Red) == 0 {
            return true
        }
        if self.unturned_cells_of_color(Blue) == 0 {
            return true
        }
        if self.unturned_cells_of_color(Bomb) == 0 {
            return true
        }
        return false
    }

    // Visible cells = cells that have been overturned.
    // When all red or blue cells are visible, it's game over
    fn count_visible_cells_of_color(&self, color: State) -> usize {
        self.cells.iter().filter(|x| x.visibility)
                         .filter(|x| x.color == color).count()
    }

    fn unturned_cells_of_color(&self, color: State) -> usize {
        self.cells.iter().filter(|x| !x.visibility)
                         .filter(|x| x.color == color).count()
    }

    pub fn show_censored_map(&self) {
        let max_len = self.cells.iter().map(|x| x.word.len()).max().unwrap();

        for i in 0..5 {
            for j in 0..5 {
                let cell = self.cells[i*5+j];
                if cell.visibility {
                    print!("{} {:width$} ", cell.color, "", width=max_len);
                } else {
                    print!("{} {:width$} ", "U", cell.word, width=max_len);
                }
            }
            println!()

        }

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
