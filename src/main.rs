mod game;
mod players;
mod map;

use crate::game::Game;
use crate::map::Map;
use crate::players::RandomSpyMaster;
use crate::players::RandomFieldOperatives;
use crate::players::HumanCliSpymaster;
use crate::players::HumanCliFieldOperatives;
use crate::players::SimpleWordVectorSpymaster;

use crate::game::RoundResult;
use std::io::{BufReader, Read};
use std::fs::File;

#[macro_use]
extern crate lazy_static;
use finalfusion::prelude::*;
use finalfusion::vocab::Vocab;
use finalfusion::similarity::{EmbeddingSimilarity, WordSimilarity};
use rand::thread_rng;
use rand::Rng;

/*
1- Generate random map
2- Generate random words
3- Separate code into separate files
4. Check words against map
5. Write an iterator for remaining words.
6. Check winning conditions
7. Do the loop
8. Check game winning logic
9. Separate player into captain/team
9. Tick function shouldn't print anything. It should return an enum/option
9. Look into the current player logic - might be wrong lifetime issue
4. Make game with random player and human player
5. Write some tests
5. Word vectors
*/



fn main() {

    // let mut map = Map::new();
    // println!("Map: \n{}", map);


    // let mut blue_sm = SimpleWordVectorSpymaster::new();
    // let mut red_fo = RandomFieldOperatives::new();
    // let mut red_sm = RandomSpyMaster::new();
    // let mut blue_fo = HumanCliFieldOperatives::new();
    // let mut game = Game::new(& mut map, &mut red_sm, &mut red_fo,
    //                          &mut blue_sm, &mut blue_fo);
    //
    // //let hint = game.red_player.give_hint();
    // while !game.is_over {
    //     game.map.show_censored_map();
    //     println!("Now is the turn of {} player", game.current_player);
    //     let found = game.tick();
    //     println!("Found {:?} agents", found);
    //     if let RoundResult::FoundBomb = found {
    //         print!("{} lost the game! They found da bomb!", game.current_player);
    //         break;
    //     }
    //     if game.map.is_game_finished() {
    //         println!("The game was won by: {}", game.current_player);
    //     }
    //     game.swap_player();
    //     println!();
    // }
   let mut reader = BufReader::new(File::open("resources/english-skipgram-mincount-50-ctx-10-ns-5-dims-300.fifu").unwrap());

   // Read the embeddings.
   let embeddings: Embeddings<VocabWrap, StorageViewWrap > =
       Embeddings::read_embeddings(&mut reader)
       .unwrap();



     let mut file = File::open("resources/wordlist").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
     let words=    contents.lines().map(String::from).collect::<Vec<String>>();
   // Look up an embedding.
   let mut rng = thread_rng();
   for i in 0..50 {
       let j = rng.gen_range(0, words.len());
       println!("{}: {}, {:?}", j,  words[j], embeddings.embedding(&words[j]))
   }
   let word = "CAT";
   println!("{}, {:?}",  word, embeddings.word_similarity(word, 10));
   let word = "Cat";
   println!("{}, {:?}",  word, embeddings.embedding(word));

       let word = "School";
   println!("{}, {:?}",   word, embeddings.embedding(word));
   let word = "SCHOOL";
   println!("{}, {:?}",  word, embeddings.embedding(word));

       let word = "PLATE";
   println!("{}, {:?}",  word, embeddings.embedding(word));
   let word = "Plate";
   println!("{}, {:?}",  word, embeddings.embedding(word));
}
