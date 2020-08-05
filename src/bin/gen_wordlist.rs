use std::io::{BufReader, BufWriter};
use std::fs::File;
use finalfusion::prelude::*;
use finalfusion::vocab::{Vocab, SimpleVocab};
use finalfusion::norms::NdNorms;
use finalfusion::storage::{NdArray, StorageWrap};
use ndarray::{Array2, Array1};
use std::collections::HashSet;
use finalfusion::io::WriteEmbeddings;


fn main() {
     let mut reader = BufReader::new(File::open("resources/english-skipgram-mincount-50-ctx-10-ns-5-dims-300.fifu").unwrap());
    //
    // Read the embeddings.
    let embeddings: Embeddings<VocabWrap, StorageViewWrap> =
        Embeddings::read_embeddings(&mut reader)
            .unwrap();

    let words = embeddings.vocab().words();
    let mut total = 0;
    let mut letters = 0;
    let mut lowercase = 0;
    let mut uppercase = 0;
    let mut alphanum = 0;
    let mut select = HashSet::new();
    for w in words {
        //println!("{}", w);
        total += 1;
        if w.chars().all(char::is_alphabetic) {
            letters += 1;
            if !w.chars().any(char::is_uppercase) {
                lowercase +=1;
                select.insert(w.clone());
            } else {
                uppercase +=1;
            }
        } else {
            alphanum +=1;
        }
    }
    println!("{} {} {} {} {}", total, letters, alphanum, lowercase, uppercase);

    let mut selected_vocab = Vec::new();
    let mut selected_storage = Array2::zeros((select.len(), embeddings.dims()));
    let mut selected_norms = Array1::zeros((select.len(),));

    for (idx, word) in select.into_iter().enumerate() {
        match embeddings.embedding_with_norm(&word) {
            Some(embed_with_norm) => {
                selected_storage
                    .row_mut(idx)
                    .assign(&embed_with_norm.embedding);
                selected_norms[idx] = embed_with_norm.norm;
            }
            None => panic!("Cannot get embedding for: {}", word),
        }

        selected_vocab.push(word);
    }

    let new_embs = Embeddings::new(
        None,
        SimpleVocab::new(selected_vocab),
        NdArray::from(selected_storage),
        NdNorms::new(selected_norms),
    );
    let f = File::create("resources/smaller_embs.fifu").unwrap();
    let mut reader = BufWriter::new(f);
    new_embs.write_embeddings(&mut reader);
}