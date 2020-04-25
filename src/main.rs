use rand::prelude::*;
use core::fmt;
use std::fs::File;
use std::io::Read;

mod game;
mod players;

use crate::game::{Map, Game};
use crate::players::{RandomPlayer, HumanPlayer, Player};
#[macro_use]
extern crate lazy_static;
//use finalfusion::prelude::*;
//use finalfusion::vocab::Vocab;
//use finalfusion::similarity::{EmbeddingSimilarity, WordSimilarity};



/*
1- Generate random map
2- Generate random words
3- Separate code into separate files
4. Make game with random player and human player
5. Word vectors
*/



fn main() {

    let map = Map::new();
    println!("Map: \n{}", map);


    let blue_player = Box::new(HumanPlayer{});
    let red_player = Box::new(RandomPlayer::new());

    let mut game = Game{map, red_player, blue_player};

    let hint = game.red_player.give_hint();
    println!("Hint: {:?}", &hint);
    println!("Words: {:?}", game.red_player.choose_words(hint))
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
