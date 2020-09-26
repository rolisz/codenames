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
use std::time::Instant;


fn main() {
    // let mut reader = BufReader::new(File::open("resources/ff.fifu").unwrap());
    // let mut reader = BufReader::new(File::open("resources/cc.en.300.fifu").unwrap());

    let now = Instant::now();

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
        println!("{}", now.elapsed().as_secs());

    let mut rng = thread_rng();

    let words = embeddings.vocab().words();

    let mut file = File::open("resources/wordlist").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let words = contents.lines().collect::<Vec<&str>>();

    let mut map = Map::new_fixed1(&words);
    //println!("{:?}", map);
    println!("Words on board: {:?}", map);
    let mut svsp2 = SummedVectorSpymaster{embeddings: &embeddings, color:Color::Red, cnt: 2};
    let mut svsp3 = SummedVectorSpymaster{embeddings: &embeddings, color:Color::Red, cnt: 3};
    let mut svsp4 = SummedVectorSpymaster{embeddings: &embeddings, color:Color::Red, cnt: 4};
    let mut dhsp = DoubleHintVectorSpymaster{embeddings: &embeddings, color: Color::Red};
    let mut bwsp = BestWordVectorSpymaster{embeddings: &embeddings, color: Color::Red};
    //let mut sp = HumanCliSpymaster{};
    let mut fo = SimpleWordVectorFieldOperative::new(&embeddings);
    //let mut fo = HumanCliFieldOperative{};


   //     let hint = bwsp.give_hint(&map);
   // println!("Best word: {:?}", &hint);

         let hint = dhsp.give_hint(&map);
     println!("Double word: {:?}", &hint);
    //     let hint = svsp2.give_hint(&map);
    // println!("Summed 2: {:?}", &hint);
    //         let hint = svsp3.give_hint(&map);
    // println!("Summed 3: {:?}", &hint);
    //         let hint = svsp4.give_hint(&map);
    // println!("Summed 4: {:?}", &hint);
    let resp = fo.choose_words(&hint, &map.remaining_words());
    println!("Choices: {:?}", resp);

        let mut map = Map::new_fixed2(&words);
        println!("Words on board: {:?}", map);

   //     let hint = bwsp.give_hint(&map);
   // println!("Best word: {:?}", &hint);

         let hint = dhsp.give_hint(&map);
     println!("Double word: {:?}", &hint);
    //     let hint = svsp2.give_hint(&map);
    // println!("Summed 2: {:?}", &hint);
    //         let hint = svsp3.give_hint(&map);
    // println!("Summed 3: {:?}", &hint);
    //         let hint = svsp4.give_hint(&map);
    // println!("Summed 4: {:?}", &hint);
    let resp = fo.choose_words(&hint, &map.remaining_words());
    println!("Choices: {:?}", resp);

            let mut map = Map::new_fixed3(&words);
        println!("Words on board: {:?}", map);

   //     let hint = bwsp.give_hint(&map);
   // println!("Best word: {:?}", &hint);

         let hint = dhsp.give_hint(&map);
     println!("Double word: {:?}", &hint);
    //     let hint = svsp2.give_hint(&map);
    // println!("Summed 2: {:?}", &hint);
    //         let hint = svsp3.give_hint(&map);
    // println!("Summed 3: {:?}", &hint);
    //         let hint = svsp4.give_hint(&map);
    // println!("Summed 4: {:?}", &hint);
    // println!("{:?}", fo.choose_words(&hint, &words));
   let resp = fo.choose_words(&hint, &map.remaining_words());
    println!("Choices: {:?}", resp);

    // let result = game(&mut sp, &mut fo,
    //        &mut RandomSpyMaster::new(&words), &mut RandomFieldOperative::new(), &mut map);
    //
    // println!("The winner is {}", result);
}


