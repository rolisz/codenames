use rand::prelude::*;
use ndarray::prelude::*;

use crate::game::{CODENAME_WORDS, Game};
use std::io;
use std::io::BufReader;
use std::fs::File;

use finalfusion::prelude::*;
use crate::map::{Map, State};
use finalfusion::similarity::WordSimilarity;
use crate::map::State::{Blue, Red};
use ndarray::ArrayView1;
use itertools::Itertools;

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

pub struct SimpleWordVectorSpymaster<'a> {
    embeddings: &'a Embeddings<VocabWrap, StorageViewWrap>,
    color: State,
}

impl SimpleWordVectorSpymaster<'_> {
    pub fn new(embeddings: &Embeddings<VocabWrap, StorageViewWrap>, color: State) -> SimpleWordVectorSpymaster {
        SimpleWordVectorSpymaster{embeddings, color}
    }
}

impl Spymaster for SimpleWordVectorSpymaster<'_> {
    fn give_hint(&mut self, map: &Map) -> Hint {
        let enemy_color = match self.color {
            Red => Blue,
            Blue=> Red,
            _ => panic!("Invalid player color")
        };
        let remaining_words = map.get_remaining_words_of_color(enemy_color);
        let word = remaining_words.get(0).unwrap();
        let words = self.embeddings.word_similarity(word, 10).unwrap();
        println!("Similar words: {:?}", words);
        return Hint{count: 1, word: words.get(0).unwrap().word.to_string()};
    }
}

pub struct SimpleWordVectorFieldOperatives<'a> {
    embeddings: &'a Embeddings<VocabWrap, StorageViewWrap>,
}

impl SimpleWordVectorFieldOperatives<'_> {
    pub fn new(embeddings: &Embeddings<VocabWrap, StorageViewWrap>) -> SimpleWordVectorFieldOperatives {
        SimpleWordVectorFieldOperatives{embeddings}
    }
}

impl FieldOperatives for SimpleWordVectorFieldOperatives<'_> {
    fn choose_words<'a>(&mut self, hint: &Hint, words: &Vec<&'a String>) -> Vec<&'a String> {
        let count = hint.count;
        let hint_word = &hint.word;
        let hint_emb = self.embeddings.embedding(hint_word).unwrap();
        let hint_embedding: ArrayView1<f32> = hint_emb.view();

        let mut similarities: Vec<f32> = vec![];
        for w in words {
            let new_embed = self.embeddings.embedding(w).unwrap();
            let similarity = new_embed.view().dot(&hint_embedding);
            similarities.push(similarity);
        }
        let sorted_sims: Vec<(usize, &f32)> = similarities.iter().enumerate().sorted_by(|(_, elem), (_, elem2)| elem.partial_cmp(elem2).unwrap()).collect();
        return vec![words[sorted_sims.last().unwrap().0]]
    }
}