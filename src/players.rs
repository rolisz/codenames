use rand::prelude::*;
use crate::map::{Map, Color};
use std::io;
use finalfusion::prelude::*;
use finalfusion::similarity::WordSimilarity;
use crate::game::opposite_player;
use ndarray::ArrayView1;
use itertools::Itertools;

#[derive(Debug)]
pub struct Hint {
    pub word: String,
    pub count: usize,
}

pub trait Spymaster {
    fn give_hint(&mut self, map: &Map) -> Hint;
}

pub trait FieldOperative {
    fn choose_words<'a>(&mut self, hint: &Hint, words: &[&'a str]) -> Vec<&'a str>;
}


pub struct RandomSpyMaster<'a> {
    rng: ThreadRng,
    clues: &'a [&'a str]
}

pub struct RandomFieldOperative {
    rng: ThreadRng,
}

impl RandomSpyMaster<'_> {
    pub fn new<'a>(words: &'a [&'a str]) -> RandomSpyMaster<'a> {
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
        let word = (*self.clues.choose(&mut self.rng).unwrap()).to_string();
        let count = self.rng.gen_range(1, 5);
        Hint { word, count }
    }
}

impl FieldOperative for RandomFieldOperative {
    fn  choose_words<'a>(&mut self, hint: &Hint, words: &[&'a str]) -> Vec<&'a str> {
        let nr_found_words = self.rng.gen_range(1, hint.count+1) as usize;
        words.choose_multiple(&mut self.rng, nr_found_words).copied().collect()
    }
}

pub struct HumanCliSpymaster {}

pub struct HumanCliFieldOperative {}

impl Spymaster for HumanCliSpymaster {
    fn give_hint(&mut self, map: &Map) -> Hint {
        println!("Give a hint for this map in count, word format: \n{} ", map);
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let results = input.split_ascii_whitespace().collect::<Vec<&str>>();
            match results[0].parse::<usize>() {
                Ok(count) => return Hint { count, word: results[0].to_string() },
                Err(_e) => println!("Give hint in count, word format!"),
            };
        }
    }
}


impl FieldOperative for HumanCliFieldOperative {
   fn choose_words<'a>(&mut self, hint: &Hint, words: &[&'a str]) -> Vec<&'a str> {
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

pub struct SimpleWordVectorSpymaster<'a> {
    embeddings: &'a Embeddings<VocabWrap, StorageViewWrap>,
    color: Color,
}

impl SimpleWordVectorSpymaster<'_> {
    pub fn new(embeddings: &Embeddings<VocabWrap, StorageViewWrap>, color: Color) -> SimpleWordVectorSpymaster {
        SimpleWordVectorSpymaster{embeddings, color}
    }
}

impl Spymaster for SimpleWordVectorSpymaster<'_> {
    fn give_hint(&mut self, map: &Map) -> Hint {
        let enemy_color = opposite_player(self.color);
        let remaining_words = map.remaining_words_of_color(enemy_color);
        let word = remaining_words.get(0).unwrap();
        let words = self.embeddings.word_similarity(word, 10).unwrap();
        println!("Similar words: {:?}", words);
        return Hint{count: 1, word: words.get(0).unwrap().word.to_string()};
    }
}


pub struct SimpleWordVectorFieldOperative<'a> {
    embeddings: &'a Embeddings<VocabWrap, StorageViewWrap>,
}

impl SimpleWordVectorFieldOperative<'_> {
    pub fn new(embeddings: &Embeddings<VocabWrap, StorageViewWrap>) -> SimpleWordVectorFieldOperative {
        SimpleWordVectorFieldOperative{embeddings}
    }
}

impl FieldOperative for SimpleWordVectorFieldOperative<'_> {
    fn choose_words<'a>(&mut self, hint: &Hint, words: &[&'a str]) -> Vec<&'a str> {
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
        let sorted_sims: Vec<(usize, &f32)> = similarities.iter().enumerate().sorted_by(|(_, elem), (_, elem2)| elem.partial_cmp(elem2).unwrap()).rev().collect();
        let mut results = vec![];
        for sims in sorted_sims.iter().take(hint.count) {
            results.push(words[sims.0]);
        }
        results
    }
}