/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // let mut hands: BinaryHeap<_> = hands.iter().map(|&s| (PokerHand::parse(s), s)).collect();
    let mut hands: BinaryHeap<_> = hands.iter().map(|&s| (PokerHand::parse(s), s)).collect();
    println!("winning_hands hands -->>> {:?}", hands);

    let (winning, s) = hands.pop().unwrap();

    let mut result = vec![s];

    while let Some((value, s)) = hands.pop() {
        if value < winning {
            break;
        }
        result.push(s);
    }
    result
}
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PokerHand {
    counts: Vec<usize>,
    values: Vec<u8>,
}
fn parse_card(s: &str) -> (u8, u8) {
    let (value, suit) = s.split_at(s.len() - 1);
    println!("1 value: {}, suit: {}", value, suit);

    // TODO: below is the return (u8, u8)
    (
        match value.parse::<u8>() {
            Ok(v) => {
                println!("[parse_card] value is a number -->>> {}", v);
                v
            },
            Err(_) =>  {
                let tmp_v =  "JQKA".find(value).unwrap() as u8 + 11;
                println!("[parse_card] value is not number -->>> {}", tmp_v);
                tmp_v

                // "JQKA".find(value).unwrap() as u8 + 11
            },
        },
        suit.as_bytes()[0],
    )
}
impl PokerHand {
    fn parse(s: &str) -> Self {
        let (values, suits): (Vec<u8>, Vec<u8>) = s.split_whitespace().map(parse_card).unzip();
        println!("values -->>> {:?}, suits -->>> {:?}", values, suits);

        let mut groups = HashMap::<u8, usize>::new();
        for &v in values.iter() {
            *groups.entry(v).or_default() += 1;
        }
        println!("groups 1 -->>> {:?}", groups);

        let mut groups: Vec<_> = groups.into_iter().map(|(v, c)| (c, v)).collect();
        groups.sort_unstable_by_key(|&x| Reverse(x));
        println!("groups 2 -->>> {:?}", groups);

        let (mut counts, mut values): (Vec<_>, Vec<_>) = groups.iter().copied().unzip();
        println!("counts 1 -->>> {:?}, values 1 -->>> {:?}", counts, values);

        if counts.len() == 5 {
            println!("-->>> 5 different suits");
            if values == [14, 5, 4, 3, 2] {
                values = vec![5, 4, 3, 2, 1];
            }
            let is_straight = values[0] - values[4] == 4;

            let is_flush = suits[1..].iter().all(|&x| x == suits[0]);

            println!("is_straight -->>> {}, is_flush -->>> {}", is_straight, is_flush);

            match (is_straight, is_flush) {
                (true, true) => counts = vec![5],
                (true, false) => counts = vec![3, 1, 2],
                (false, true) => counts = vec![3, 1, 3],
                _ => { println!("-->>> no is_straight and no is_flush"); }
            }

            println!("counts 2 -->>> {:?}, values 2 -->>> {:?}", counts, values);
        }
        Self { counts, values }
    }
}
