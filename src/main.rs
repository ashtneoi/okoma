extern crate rand;

use std::env;
use std::str::FromStr;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let vowel = [
        "a", "e", "i", "o", "u",
        "ai", "aia", "aio", "aiu",
        "ea", "eo", "ia", "io",
        "oa", "oia", "oio",
        "ua", "uia", "uio",
    ];
    let cons = [
        "b", "g", "k", "l", "m", "n", "r", "t", "y",
    ];

    let mut rng = thread_rng();

    let usage_msg = "Usage:  okoma COUNT";
    let count_str = env::args().nth(1).expect(usage_msg);
    let count = u64::from_str(&count_str).expect(usage_msg);
    for _ in 0..count {
        println!("{}", gen_word(&vowel, &cons, &mut rng));
    }
}


fn gen_word<R>(vowel: &[&str], cons: &[&str], rng: &mut R) -> String
where R: Rng {
    let mut word = "".to_string();
    if rng.gen() {
        word.push_str(rng.choose(&vowel).unwrap());
    }
    for _ in 0..(rng.gen_range(2, 4)) {
        word.push_str(rng.choose(&cons).unwrap());
        word.push_str(rng.choose(&vowel).unwrap());
    }
    word
}
