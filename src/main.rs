extern crate rand;

use rand::distributions::{
    IndependentSample,
    Weighted,
    WeightedChoice,
};
use rand::thread_rng;
use rand::Rng;
use std::env;
use std::process::exit;
use std::str::FromStr;

fn exit_with_usage() -> ! {
    println!("Usage: okoma COUNT");
    exit(2);
}

fn main() {
    let vv: &[(&str, u32)] = &[
        ("a", 10),
        ("e", 5),
        ("i", 8),
        ("o", 4),
        ("u", 2),
        ("ai", 3),
        ("aia", 1),
        ("aio", 1),
        ("aiu", 2),
        ("ea", 1),
        ("eo", 1),
        ("ia", 6),
        ("io", 3),
        ("oa", 6),
        ("oia", 1),
        ("oio", 2),
        ("ua", 2),
        ("uia", 1),
        ("uio", 1),
    ];
    let cc: &[(&str, u32)] = &[
        ("b", 2),
        ("g", 1),
        ("k", 8),
        ("l", 7),
        ("m", 8),
        ("n", 7),
        ("r", 4),
        ("t", 4),
    ];

    let mut rng = thread_rng();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit_with_usage();
    }

    let count_str = &args[1];
    let count = u32::from_str(&count_str).unwrap_or_else(
        |_| exit_with_usage()
    );

    let mut v: Vec<_> = WordGen::build_weighted_vec(&mut vv.iter());
    let mut c: Vec<_> = WordGen::build_weighted_vec(&mut cc.iter());

    let wg = WordGen::new(&mut v, &mut c);
    for _ in 0..count {
        println!("{}", wg.gen(&mut rng));
    }
}

struct WordGen<'a, 'b> {
    vdist: WeightedChoice<'a, &'a str>,
    cdist: WeightedChoice<'b, &'b str>,
}

impl<'a, 'b> WordGen<'a, 'b> {
    fn build_weighted_vec<'c, 'd, I>(x: &'c mut I) -> Vec<Weighted<&'d str>>
    where
        I: Iterator<Item = &'d (&'d str, u32)>,
    {
        x.map(
            |&(s, w)| Weighted { weight: w, item: s }
        ).collect()
    }

    fn new(v: &'a mut [Weighted<&'a str>], c: &'b mut [Weighted<&'b str>])
        -> WordGen<'a, 'b>
    {
        WordGen {
            vdist: WeightedChoice::new(v),
            cdist: WeightedChoice::new(c),
        }
    }

    fn gen<R: Rng>(&self, rng: &mut R) -> String {
        let mut word = "".to_string();
        let mut low = 2;
        if rng.gen() {
            low -= 1;
            word.push_str(self.vdist.ind_sample(rng));
        }
        for _ in 0..(rng.gen_range(low, low + 2)) {
            word.push_str(self.cdist.ind_sample(rng));
            word.push_str(self.vdist.ind_sample(rng));
        }
        word
    }
}
