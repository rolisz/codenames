use rand::prelude::*;
use crate::map::{Map, Color};
use std::io;
use finalfusion::prelude::*;
use finalfusion::similarity::{WordSimilarity, EmbeddingSimilarity};
use crate::game::opposite_player;
use ndarray::{ArrayView1, ArrayView2, ViewRepr, Array1, ArrayViewMut1, ShapeBuilder};
use itertools::Itertools;
use ordered_float::NotNan;
use inflector::string::pluralize::to_plural;
use std::collections::HashSet;
use std::hash::Hash;
use finalfusion::similarity::WordSimilarityResult;
use std::collections::HashMap;
use ordered_float::NotNaN;

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

type Embedding = Embeddings<VocabWrap, StorageViewWrap>;

fn find_similar_words<'a>(word: &str, embeddings: &'a Embedding, limit: usize) -> Vec<WordSimilarityResult<'a>> {
    let embed = embeddings.embedding(&word).unwrap();
    let mut skip: HashSet<&str> = HashSet::new();
    skip.insert(&word);
    let pluralized = to_plural(&word);
    skip.insert(&pluralized);
    embeddings.embedding_similarity_masked(embed.view(), limit, &skip).unwrap()
}


pub struct BestWordVectorSpymaster<'a> {
    pub embeddings: &'a Embedding,
    pub color: Color,
}

impl Spymaster for BestWordVectorSpymaster<'_> {
    // This spymaster computes a hint for every word and suggests the one with the best similarity
    fn give_hint(&mut self, map: &Map) -> Hint {
        let enemy_color = opposite_player(self.color);
        let remaining_words = map.remaining_words_of_color(enemy_color);
        let mut best_sim = NotNan::new(-1f32).unwrap();
        let mut best_word = "";
        for word in remaining_words {
            let words = find_similar_words(&word, self.embeddings, 1);
            let hint = words.get(0).unwrap();
            if hint.similarity > best_sim {
                best_sim = hint.similarity;
                best_word = hint.word;
            }
        }
        return Hint{count: 1, word: best_word.to_string()};
    }
}

pub struct DoubleHintVectorSpymaster<'a> {
    pub embeddings: &'a Embedding,
    pub color: Color,
}


impl Spymaster for DoubleHintVectorSpymaster<'_> {
    // This spymaster tries to find a clue that matches two words
    fn give_hint(&mut self, map: &Map) -> Hint {
        let enemy_color = opposite_player(self.color);
        let remaining_words = map.remaining_words_of_color(enemy_color);

        let mut sim_words: HashMap<&str, Vec<&str>> = HashMap::new();
        for word in remaining_words {
            let words = find_similar_words(&word, self.embeddings, 300);
            for w in words {
                if !sim_words.contains_key(w.word) {
                    let mut lst = vec![word];
                    sim_words.insert(w.word, lst);
                } else {
                    let mut lst = sim_words.get_mut(w.word).unwrap();
                    lst.push(word);
                }
                // let count = sim_words.entry(w.word).or_insert(0);
                // *count +=1;
            }
        }
        let mut best_word = sim_words.iter().max_by_key(|(_, y)| y.len()).unwrap();
        println!("{:?}", best_word);

        return Hint{count: best_word.1.len() as usize, word: best_word.0.to_string()};
    }
}

// Build out sets of 3-4-5 words, add them up and then find closest word to that embedding

pub struct SummedVectorSpymaster<'a> {
    pub embeddings: &'a Embedding,
    pub color: Color,
    pub cnt: usize,
}


impl Spymaster for SummedVectorSpymaster<'_> {
    // This spymaster tries to find a clue that matches two words
    fn give_hint(&mut self, map: &Map) -> Hint {
        let enemy_color = opposite_player(self.color);
        let remaining_words = map.remaining_words_of_color(enemy_color);
        let mut local_embeddings = HashMap::new();
        for &word in &remaining_words {
            let emb = self.embeddings.embedding(&word.to_lowercase()).unwrap();
            local_embeddings.insert(word.clone(), emb);
        }
        let mut best_sim= NotNan::new(-1f32).unwrap();
        let mut best_word = "";

        let it = remaining_words.iter().combinations(self.cnt);
        for word_combs in it {
            let emb_sum = word_combs.iter().map(|&x| local_embeddings.get(x).unwrap()).fold(Array1::zeros((300).f()), |x, y|{
                x+y
            });
            let mut skip_set: HashSet<&str> = HashSet::new();
            let plurals = word_combs.iter().map(|x| to_plural(x)).collect::<Vec<String>>();
            for w in &word_combs {
                skip_set.insert(w);
            }
            for w in &plurals {
                skip_set.insert(&w);
            }
            let ws = self.embeddings.embedding_similarity_masked(emb_sum.view(), 20, &skip_set).unwrap();
            // println!("{:?}", word_combs);
            // println!("{:?}", ws);
            let hint = ws.get(0).unwrap();
            if hint.similarity > best_sim {
                best_sim = hint.similarity;
                best_word = hint.word;
                println!("Best sim so far: {:?}", word_combs);
            }
        }

        return Hint{count: self.cnt, word: best_word.to_string()};
    }
}

pub struct SimpleWordVectorFieldOperative<'a> {
    embeddings: &'a Embedding,
}

impl SimpleWordVectorFieldOperative<'_> {
    pub fn new(embeddings: &Embedding) -> SimpleWordVectorFieldOperative {
        SimpleWordVectorFieldOperative{embeddings}
    }
}

impl FieldOperative for SimpleWordVectorFieldOperative<'_> {
    fn choose_words<'a>(&mut self, hint: &Hint, words: &[&'a str]) -> Vec<&'a str> {
        let hint_emb = self.embeddings.embedding(&hint.word).unwrap();
        let hint_embedding = hint_emb.view();
        let mut similarities = vec![];
        for w in words {
            let new_embed = self.embeddings.embedding(&w).unwrap();
            let similarity: f32 = new_embed.view().dot(&hint_embedding);
            similarities.push((w, similarity));
        }
        similarities.iter()
            .sorted_by(|(_, e), (_, e2)| e.partial_cmp(e2).unwrap())
            .rev().take(hint.count).map(|x| *x.0).collect()
    }
}