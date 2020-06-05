use rand::prelude::*;
use crate::map::Map;
use std::io;


#[derive(Debug)]
pub struct Hint {
    word: String,
    count: i32,
}

pub trait Spymaster {
    fn give_hint(&mut self, map: &Map) -> Hint;
}

pub trait FieldOperative {
    fn choose_words<'a>(&mut self, hint: &Hint, words: &Vec<&'a str>) -> Vec<&'a str>;
}


#[derive(Debug)]
pub struct RandomSpyMaster<'a> {
    rng: ThreadRng,
    clues: &'a Vec<&'a str>
}

#[derive(Debug)]
pub struct RandomFieldOperative {
    rng: ThreadRng,
}

impl RandomSpyMaster<'_> {
    pub fn new<'a>(words: &'a Vec<&'a str>) -> RandomSpyMaster<'a> {
        let rng = thread_rng();
        RandomSpyMaster{rng, clues: words}
    }
}

impl RandomFieldOperative {
    pub fn new() -> RandomFieldOperative {
        let rng = thread_rng();
        RandomFieldOperative{rng}
    }
}


impl Spymaster for RandomSpyMaster<'_> {
    fn give_hint(&mut self, _map: &Map) -> Hint {
        let word = self.clues.choose(&mut self.rng).unwrap().to_string();
        let count = self.rng.gen_range(1, 5);
        Hint { word, count }
    }
}

impl FieldOperative for RandomFieldOperative {
    fn  choose_words<'a>(&mut self, hint: &Hint, words: &Vec<&'a str>) -> Vec<&'a str> {
        let nr_found_words = self.rng.gen_range(1, hint.count+1) as usize;
        words.choose_multiple(&mut self.rng, nr_found_words).map(|&x| x).collect()
    }
}

#[derive(Debug)]
pub struct HumanCliSpymaster {}

#[derive(Debug)]
pub struct HumanCliFieldOperative {}

impl Spymaster for HumanCliSpymaster {
    fn give_hint(&mut self, map: &Map) -> Hint {
        println!("Give a hint for this map in count, word format: \n{} ", map);
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let results = input.split_ascii_whitespace().collect::<Vec<&str>>();
            match results[0].parse::<i32>() {
                Ok(count) => return Hint { count, word: results[0].to_string() },
                Err(_e) => println!("Give hint in count, word format!"),
            };
        }
    }
}


impl FieldOperative for HumanCliFieldOperative {
   fn choose_words<'a>(&mut self, hint: &Hint, words: &Vec<&'a str>) -> Vec<&'a str> {
       let mut chosen_words = vec![];
       println!("Choose {} words from {:?}", hint.count, words);
       let mut counts = hint.count;
       while counts > 0 {
           let mut input = String::new();
           io::stdin().read_line(&mut input).unwrap();
           if input.trim() == "" {
               return chosen_words;
           }
           match words.iter().position(|&x| {
               x.to_lowercase() == input.trim()
           }) {
               Some(c) => {
                   chosen_words.push(words[c]);
                   counts -= 1;
               },
               None => println!("Choose a word from the given list")
           }
       }
       chosen_words
   }
}