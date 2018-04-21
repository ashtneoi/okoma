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
    let wg = WordGen::new(vv, cc);
    for _ in 0..count {
        println!("{}", wg.gen(&mut rng));
    }
}

// Turns out this is basically rand::distributions::WeightedChoice.
// TODO: Use that instead.
struct WeightedDist {
    d: Vec<u32>,
    sum: u32,
}

impl WeightedDist {
    fn new(p: &[u32]) -> WeightedDist {
        assert!(p.len() > 0);
        let mut d = Vec::<u32>::new();
        let sum = p.iter().sum();
        {
            let mut cumul_sum: u32 = 0;
            for x in p {
                d.push(cumul_sum);
                cumul_sum += *x;
            }
        }
        WeightedDist { d, sum }
    }

    fn gen<R: Rng>(&self, rng: &mut R) -> usize {
        let n: u32 = rng.gen_range(0, self.sum); // TODO: this is slow
        let ii = self.d.binary_search(&n);
        match ii {
            Ok(i) => i,
            Err(i) => i - 1, // self.d[0] is always 0 so this is fine
        }
    }
}

struct WordGen<'a, 'b> {
    vd: WeightedDist,
    v: Vec<&'a str>,
    cd: WeightedDist,
    c: Vec<&'b str>,
}

impl<'a, 'b> WordGen<'a, 'b> {
    fn new(vv: &[(&'a str, u32)], cc: &[(&'b str, u32)]) -> WordGen<'a, 'b> {
        let v: (Vec<_>, Vec<_>) = vv.iter().cloned().unzip();
        let c: (Vec<_>, Vec<_>) = cc.iter().cloned().unzip();
        WordGen {
            vd: WeightedDist::new(&v.1),
            v: v.0,
            cd: WeightedDist::new(&c.1),
            c: c.0,
        }
    }

    fn gen<R: Rng>(&self, rng: &mut R) -> String {
        let mut word = "".to_string();
        let mut low = 2;
        if rng.gen() {
            low -= 1;
            word.push_str(self.v[self.vd.gen(rng)]);
        }
        for _ in 0..(rng.gen_range(low, low + 2)) {
            word.push_str(self.c[self.cd.gen(rng)]);
            word.push_str(self.v[self.vd.gen(rng)]);
        }
        word
    }
}
