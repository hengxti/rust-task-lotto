use std::env;

use rand::{thread_rng, Rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        let mut rng = thread_rng();
        let mut numbers: Vec<usize> = Vec::new();

        for _i in 1..=take {
            let mut candidate = rng.gen_range(0..=from);
            while numbers.contains(&candidate) {
                candidate = rng.gen_range(0..=from);
            }
            numbers.push(candidate);
        }

        Self {
            take,
            from,
            numbers,
        }
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    //"6 of 45: [2, 3, 10, 25, 30, 40]"
    format!("{} of {}: {:?}", lotto.take, lotto.from, lotto.numbers)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argument_count = args.len() - 1;
    if argument_count % 2 != 0 {
        println!("Odd number of arguments not permitted");
        return;
    }

    let mut i = 1;
    loop {
        let take: usize = args[i].parse().expect("missing take argument");
        let from: usize = args[i + 1].parse().expect("missing from argument");

        if take > from {
            println!("take can not be larger than from");
            return;
        }
        let lotto = Lotto::new(take, from);
        println!("{}", format_lotto_results(&lotto));

        i += 2;
        if i >= argument_count + 1 {
            break;
        }
    }
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.numbers;

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.numbers;
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
