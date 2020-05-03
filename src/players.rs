use rand::prelude::*;


use crate::game::{CODENAME_WORDS, Game, Map};

#[derive(Debug)]
pub struct Hint {
    word: String,
    count: i32,
}

pub trait Player {
    fn give_hint(&mut self) -> Hint;
    fn choose_words<'a>(&mut self, hint: &Hint,
                    words: &Vec<&'a String>) -> Vec<&'a String>;
}

#[derive(Debug)]
pub struct RandomPlayer {
    rng: ThreadRng,
}

impl RandomPlayer {
    pub fn new() -> RandomPlayer {
        let rng = thread_rng();
        RandomPlayer{rng}
    }
}


impl Player for RandomPlayer {
    fn give_hint(&mut self) -> Hint {
        let word = CODENAME_WORDS.choose(&mut self.rng).unwrap().clone();
        let count = self.rng.gen_range(1, 5);
        Hint{word, count}
    }

    fn  choose_words<'a>(&mut self, hint: &Hint, words: &Vec<&'a String>) -> Vec<&'a String> {
        let nr_found_words = self.rng.gen_range(1, hint.count+1) as usize;

       words.choose_multiple(&mut self.rng, nr_found_words).map(|&x| x).collect()
    }
}


// #[derive(Debug)]
// pub struct HumanPlayer<'a> {
//     words: &'a Vec<Codename>
// }
//
// impl Player<'_> for HumanPlayer<'_> {
//     fn give_hint(&mut self) -> Hint {
//         unimplemented!()
//     }
//
//     fn choose_words(&mut self, hint: &Hint) -> Vec<Codename> {
//         unimplemented!()
//     }
// }
