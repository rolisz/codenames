use rand::prelude::*;


use crate::game::CODENAME_WORDS;

#[derive(Debug)]
pub struct Hint {
    word: &'static str,
    count: i32,
}

pub trait Player {
    fn give_hint(&mut self) -> Hint;
    fn choose_words(&mut self, hint: Hint) -> Vec<&String>;
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

#[derive(Debug)]
pub struct HumanPlayer {

}

impl Player for HumanPlayer {
    fn give_hint(&mut self) -> Hint {
        unimplemented!()
    }

    fn choose_words(&mut self, hint: Hint) -> Vec<&String> {
        unimplemented!()
    }
}

impl Player for RandomPlayer {
    fn give_hint(&mut self) -> Hint {
        let word = CODENAME_WORDS.choose(&mut self.rng).unwrap();
        let count = self.rng.gen_range(1, 5);
        Hint{word, count}
    }

    fn choose_words(&mut self, hint: Hint) -> Vec<&String> {
        let nr_found_words = self.rng.gen_range(1, hint.count+1) as usize;

        CODENAME_WORDS.choose_multiple(&mut self.rng, nr_found_words).collect()
    }
}