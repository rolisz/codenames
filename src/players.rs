use rand::prelude::*;


use crate::game::{CODENAME_WORDS, Game};
use std::io;
use std::io::BufReader;
use std::fs::File;

use finalfusion::prelude::*;
use crate::map::Map;


#[derive(Debug)]
pub struct Hint {
    word: String,
    count: i32,
}

pub trait Spymaster {
    fn give_hint(&mut self, map: &Map) -> Hint;
}

pub trait FieldOperatives {
    fn choose_words<'a>(&mut self, hint: &Hint,
                    words: &Vec<&'a String>) -> Vec<&'a String>;
}

#[derive(Debug)]
pub struct RandomSpyMaster {
    rng: ThreadRng,
}

#[derive(Debug)]
pub struct RandomFieldOperatives {
    rng: ThreadRng,
}

impl RandomSpyMaster {
    pub fn new() -> RandomSpyMaster {
        let rng = thread_rng();
        RandomSpyMaster{rng}
    }
}

impl RandomFieldOperatives {
    pub fn new() -> RandomFieldOperatives {
        let rng = thread_rng();
        RandomFieldOperatives{rng}
    }
}


impl Spymaster for RandomSpyMaster {
    fn give_hint(&mut self, map: &Map) -> Hint {
        let word = CODENAME_WORDS.choose(&mut self.rng).unwrap().clone();
        let count = self.rng.gen_range(1, 5);
        Hint { word, count }
    }
}

impl FieldOperatives for RandomFieldOperatives {
    fn  choose_words<'a>(&mut self, hint: &Hint, words: &Vec<&'a String>) -> Vec<&'a String> {
        let nr_found_words = self.rng.gen_range(1, hint.count+1) as usize;

       words.choose_multiple(&mut self.rng, nr_found_words).map(|&x| x).collect()
    }
}


#[derive(Debug)]
pub struct HumanCliSpymaster {

}
impl HumanCliSpymaster {
    pub fn new() -> HumanCliSpymaster {
        HumanCliSpymaster{}
    }
}
#[derive(Debug)]
pub struct HumanCliFieldOperatives {

}

impl HumanCliFieldOperatives {
    pub fn new() -> HumanCliFieldOperatives {
        HumanCliFieldOperatives{}
    }
}
impl Spymaster for HumanCliSpymaster {
    fn give_hint(&mut self, map: &Map) -> Hint {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                let results = input.split_ascii_whitespace().collect::<Vec<&str>>();
                let count = results[0].parse::<i32>().unwrap();
                Hint{count, word: String::from(results[0])}
            }
            Err(error) => panic!("error: {}", error),
        }
    }
}

impl FieldOperatives for HumanCliFieldOperatives {
   fn choose_words<'a>(&mut self, hint: &Hint, words: &Vec<&'a String>) -> Vec<&'a String> {
       let mut chosen_words = vec![];
       println!("Choose {} words from {:?}", hint.count, words);
       let mut counts = hint.count;
       while counts > 0 {
          let mut input = String::new();
           match io::stdin().read_line(&mut input) {

                Ok(n) => {
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
                Err(error) => panic!("error: {}", error),
        }
       }
       return chosen_words;
   }
}

pub struct SimpleWordVectorSpymaster {
    embeddings: Embeddings<VocabWrap, StorageWrap>
}
impl SimpleWordVectorSpymaster {
    pub fn new() -> SimpleWordVectorSpymaster {
        let mut reader = BufReader::new(File::open("resources/english-skipgram-mincount-50-ctx-10-ns-5-dims-300.fifu").unwrap());

       // Read the embeddings.
       let embeddings: Embeddings<VocabWrap, StorageWrap> =
           Embeddings::read_embeddings(&mut reader)
           .unwrap();
        SimpleWordVectorSpymaster{embeddings}
    }
}

impl Spymaster for SimpleWordVectorSpymaster {
    fn give_hint(&mut self, map: &Map) -> Hint {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                let results = input.split_ascii_whitespace().collect::<Vec<&str>>();
                let count = results[0].parse::<i32>().unwrap();
                Hint{count, word: String::from(results[0])}
            }
            Err(error) => panic!("error: {}", error),
        }
    }
}

#[derive(Debug)]
pub struct SimpleWordVectorFieldOperatives {

}

impl SimpleWordVectorFieldOperatives {
    pub fn new() -> SimpleWordVectorFieldOperatives {
        SimpleWordVectorFieldOperatives{}
    }
}