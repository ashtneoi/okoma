extern crate rand;

use std::env;
use std::process::exit;
use std::str::FromStr;

use rand::thread_rng;
use rand::Rng;


fn exit_with_usage() -> ! {
    println!("Usage: okoma COUNT");
    exit(2);
}


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

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit_with_usage();
    }

    let count_str = &args[1];
    let count = u64::from_str(&count_str).unwrap_or_else(
        |_| exit_with_usage()
    );
    for _ in 0..count {
        println!("{}", gen_word(&vowel, &cons, &mut rng));
    }
}


fn gen_word<R>(vowel: &[&str], cons: &[&str], rng: &mut R) -> String
where R: Rng {
    let mut word = "".to_string();
    let mut low = 2;
    if rng.gen() {
        word.push_str(rng.choose(&vowel).unwrap());
        low -= 1;
    }
    for _ in 0..(rng.gen_range(low, low + 2)) {
        word.push_str(rng.choose(&cons).unwrap());
        word.push_str(rng.choose(&vowel).unwrap());
    }
    word
}
