use rand::prelude::*;


use crate::game::{CODENAME_WORDS, Game, Map};

#[derive(Debug)]
pub struct Hint<'a> {
    word: &'a str,
    count: i32,
}

pub trait Player {
    fn give_hint(&mut self) -> Hint;
    fn choose_words(&mut self, hint: &Hint) -> Vec<&'static String>;
}

#[derive(Debug)]
pub struct RandomPlayer<'a> {
    rng: ThreadRng,
    words: &'a Vec<&'static String>,
}

impl RandomPlayer<'_> {
    pub fn new<'s>(words: &'s Vec<&'static String>) -> RandomPlayer<'s> {
        let rng = thread_rng();
        RandomPlayer{rng, words}
    }
}


impl Player for RandomPlayer<'_> {
    fn give_hint(&mut self) -> Hint {
        let word = CODENAME_WORDS.choose(&mut self.rng).unwrap();
        let count = self.rng.gen_range(1, 5);
        Hint{word, count}
    }

    fn choose_words(&mut self, hint: &Hint) -> Vec<&'static String> {
        let nr_found_words = self.rng.gen_range(1, hint.count+1) as usize;

       // self.words.choose_multiple(&mut self.rng, nr_found_words).map(|&x| x).collect()
        vec![]
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
