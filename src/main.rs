extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let syll_vowel = [
        "a", "e", "i", "o", "u",
        "ai", "aia", "aio", "aiu",
        "ea", "eo", "ia", "io",
        "oa", "oia", "oio",
        "ua", "uia", "uio",
    ];
    let syll_cons = [
        "b", "g", "k", "l", "m", "n", "r", "t", "y",
    ];

    let mut rng = thread_rng();
    let n = 3;

    let mut word = "".to_string();
    for _ in 0..n {
        word.push_str(rng.choose(&syll_cons).unwrap());
        word.push_str(rng.choose(&syll_vowel).unwrap());
    }
    println!("{}", word);
}
