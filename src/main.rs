mod map;
mod players;
mod game;

use crate::map::{Color, Cell, Map};
use crate::players::*;
use crate::game::game;
use std::fs::File;
use std::io::{Read, BufReader};
use finalfusion::prelude::*;
use finalfusion::vocab::Vocab;
use rand::prelude::*;


fn main() {
    // let mut reader = BufReader::new(File::open("resources/english-skipgram-mincount-50-ctx-10-ns-5-dims-300.fifu").unwrap());
    // let mut reader = BufReader::new(File::open("resources/cc.en.300.fifu").unwrap());
    // let mut reader = BufReader::new(File::open("resources/wiki.en.fifu").unwrap());
    // //
    // // Read the embeddings.
    // let embeddings: Embeddings<VocabWrap, StorageViewWrap> =
    //     Embeddings::read_embeddings(&mut reader)
    //         .unwrap();
    //
    let mut reader = BufReader::new(File::open("resources/GoogleNews-vectors-negative300.bin").unwrap());

    // Read the embeddings.
    let embeddings = Embeddings::read_word2vec_binary(&mut reader)
        .unwrap();

    let mut rng = thread_rng();

    let words = embeddings.vocab().words();
   for i in 0..100 {
       let j = rng.gen_range(0, words.len());
       println!("{}: {}", j,  words[j]);
   }
    let mut file = File::open("resources/wordlist").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let words = contents.lines().collect::<Vec<&str>>();

    let mut map = Map::new(&words);
    println!("{:?}", map);

    // let mut sp = BestWordVectorSpymaster::new(&embeddings, Color::Red);
    // //let mut sp = HumanCliSpymaster{};
    // let mut fo = SimpleWordVectorFieldOperative::new(&embeddings);
    // //let mut fo = HumanCliFieldOperative{};
    //
    // let hint = sp.give_hint(&map);
    // println!("{:?}", &hint);
    // println!("{:?}", fo.choose_words(&hint, &words));
    //
    // let result = game(&mut sp, &mut fo,
    //        &mut RandomSpyMaster::new(&words), &mut RandomFieldOperative::new(), &mut map);
    //
    // println!("The winner is {}", result);


}
