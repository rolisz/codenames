use rand::prelude::*;
use crate::State::{Neutral, Red, Blue, Bomb};
use core::fmt;
use std::fs::File;
use std::io::{BufReader, Read};

use finalfusion::prelude::*;
use finalfusion::vocab::Vocab;
use finalfusion::similarity::{EmbeddingSimilarity, WordSimilarity};
/*
1. Generate random map
2. Generate random words
3. Word vectors
*/


#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum State {
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

struct Map<'a> {
    states: [State; 25],
    words: Vec<&'a str>,
}

impl Map<'_> {
    fn new(words: Vec<&str>) -> Map {
        let mut map: [State; 25] = [Neutral, Neutral, Neutral, Neutral, Neutral, Neutral, Neutral,
        Red, Red, Red, Red, Red, Red, Red, Red, Red,  Blue, Blue, Blue, Blue, Blue, Blue, Blue,
                                   Blue, Bomb];
        let mut rng = thread_rng();
        map.shuffle(&mut rng);
        let words = words.choose_multiple(&mut rng, 25).cloned().collect::<Vec<&str>>();
        return Map{ states: map, words };
    }

}

impl fmt::Display for Map<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..5 {
            write!(f, "{} {} {} {} {}\n", self.states[i*5], self.states[i*5+1], self.states[i*5+2], self.states[i*5+3], self.states[i*5+4]);
        }
        let max_len = self.words.iter().map(|x| x.len()).max().unwrap();
        for i in 0..5 {
            write!(f, "{:width$} {:width$} {:width$} {:width$} {:width$}\n", self.words[i*5], self.words[i*5+1], self.words[i*5+2], self.words[i*5+3], self.words[i*5+4], width=max_len);
        }
        Ok(())
    }
}

fn main() {
        let mut file = File::open("resources/wordlist").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let words = contents.lines().collect::<Vec<&str>>();
        let map = Map::new(words);
        println!("Map: \n{}", map);


//    let mut reader = BufReader::new(File::open("resources/english-skipgram-mincount-50-ctx-10-ns-5-dims-300.fifu").unwrap());
//
//    // Read the embeddings.
//    let embeddings: Embeddings<VocabWrap, StorageWrap> =
//        Embeddings::read_embeddings(&mut reader)
//        .unwrap();
//
//
//
//    // Look up an embedding.
//    let mut rng = thread_rng();
//    for i in 0..50 {
//        let j = rng.gen_range(0, words.len());
//        println!("{}: {}, {:?}", j,  words[j], embeddings.embedding(words[j]))
//    }
//    let word = "CAT";
//    println!("{}, {:?}",  word, embeddings.word_similarity(word, 10));
//    let word = "Cat";
//    println!("{}, {:?}",  word, embeddings.embedding(word));
//
//        let word = "School";
//    println!("{}, {:?}",   word, embeddings.embedding(word));
//    let word = "SCHOOL";
//    println!("{}, {:?}",  word, embeddings.embedding(word));
//
//        let word = "PLATE";
//    println!("{}, {:?}",  word, embeddings.embedding(word));
//    let word = "Plate";
//    println!("{}, {:?}",  word, embeddings.embedding(word));
}
