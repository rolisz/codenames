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
use crate::map::Color::{Red, Blue};


fn main() {
    // let mut reader = BufReader::new(File::open("resources/english-skipgram-mincount-50-ctx-10-ns-5-dims-300.fifu").unwrap());
    // let mut reader = BufReader::new(File::open("resources/cc.en.300.fifu").unwrap());


    let mut reader = BufReader::new(File::open("resources/smaller_embs.fifu").unwrap());
    //
    // Read the embeddings.
    let embeddings: Embeddings<VocabWrap, StorageViewWrap> =
        Embeddings::read_embeddings(&mut reader)
            .unwrap();
    //
    // let mut reader = BufReader::new(File::open("resources/GoogleNews-vectors-negative300.bin").unwrap());
    //
    // // Read the embeddings.
    // let embeddings = Embeddings::read_word2vec_binary(&mut reader)
    //     .unwrap();

    let mut rng = thread_rng();

    let words = embeddings.vocab().words();

    let mut file = File::open("resources/wordlist").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let words = contents.lines().collect::<Vec<&str>>();

    let mut map = Map::new_fixed1(&words);
    //println!("{:?}", map);
    println!("{:?}", map.remaining_words_of_color(Blue));
    let mut svsp2 = SummedVectorSpymaster::new(&embeddings, Color::Red, 2);
    let mut svsp3 = SummedVectorSpymaster::new(&embeddings, Color::Red, 3);
    let mut svsp4 = SummedVectorSpymaster::new(&embeddings, Color::Red, 4);
    let mut dhsp = DoubleHintVectorSpymaster::new(&embeddings, Color::Red);
    let mut bwsp = BestWordVectorSpymaster::new(&embeddings, Color::Red);
    let mut swsp = SimpleWordVectorSpymaster::new(&embeddings, Color::Red);
    //let mut sp = HumanCliSpymaster{};
    let mut fo = SimpleWordVectorFieldOperative::new(&embeddings);
    //let mut fo = HumanCliFieldOperative{};

    let hint = swsp.give_hint(&map);

    println!("Simple: {:?}", &hint);
        let hint = bwsp.give_hint(&map);
    println!("Best word: {:?}", &hint);
        let hint = dhsp.give_hint(&map);
    println!("Double word: {:?}", &hint);
        let hint = svsp2.give_hint(&map);
    println!("Summed 2: {:?}", &hint);
            let hint = svsp3.give_hint(&map);
    println!("Summed 3: {:?}", &hint);
            let hint = svsp4.give_hint(&map);
    println!("Summed 4: {:?}", &hint);

        let mut map = Map::new_fixed2(&words);
        println!("{:?}", map.remaining_words_of_color(Blue));

        let hint = swsp.give_hint(&map);
    println!("Simple: {:?}", &hint);
        let hint = bwsp.give_hint(&map);
    println!("Best word: {:?}", &hint);
        let hint = dhsp.give_hint(&map);
    println!("Double word: {:?}", &hint);
        let hint = svsp2.give_hint(&map);
    println!("Summed 2: {:?}", &hint);
            let hint = svsp3.give_hint(&map);
    println!("Summed 3: {:?}", &hint);
            let hint = svsp4.give_hint(&map);
    println!("Summed 4: {:?}", &hint);

            let mut map = Map::new_fixed3(&words);
        println!("{:?}", map.remaining_words_of_color(Blue));

        let hint = swsp.give_hint(&map);
    println!("Simple: {:?}", &hint);
        let hint = bwsp.give_hint(&map);
    println!("Best word: {:?}", &hint);
        let hint = dhsp.give_hint(&map);
    println!("Double word: {:?}", &hint);
        let hint = svsp2.give_hint(&map);
    println!("Summed 2: {:?}", &hint);
            let hint = svsp3.give_hint(&map);
    println!("Summed 3: {:?}", &hint);
            let hint = svsp4.give_hint(&map);
    println!("Summed 4: {:?}", &hint);
    // println!("{:?}", fo.choose_words(&hint, &words));

    // let result = game(&mut sp, &mut fo,
    //        &mut RandomSpyMaster::new(&words), &mut RandomFieldOperative::new(), &mut map);
    //
    // println!("The winner is {}", result);
}


