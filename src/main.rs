use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Card {
    Flower,
    Num(u8, u8),
    Spec(u8),
    Empty,
    Disable
}

impl Card {
    fn can_move_front_of(&self, target_card: &Card) -> bool{
        if let Card::Empty = target_card {
            return true;
        }
        // self can't be empty or flower
        if let (Card::Num(tar_kind, tar_num), Card::Num(kind, num)) = (target_card, self) {
            if tar_kind != kind && tar_kind == num {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, PartialEq)]
struct Decks {
    rev_cards: [u8; 3],
    stks: [Vec<Card>; 8],
    storage: [Card; 3]
}

fn give_a_deque() -> Vec<Card> {
    Vec::<Card>::new()
}

impl Decks {
    // TODO add card
    fn new(cards: String) -> Self {

        let mut stks: [Vec<Card>; 8] = Default::default();
        for idx in 0..8 {
            stks[idx].push(Card::Empty);
        }

        let cards: Vec<char> = cards.chars().collect();
        for idx in 0..40 {
            match cards[idx * 2] {
                'r' => {
                    stks[idx / 5].push(Card::Num(0, cards[idx * 2 + 1]  as u8 - 48));
                },
                'g' => {
                    stks[idx / 5].push(Card::Num(1, cards[idx * 2 + 1]  as u8 - 48));
                },
                'b' => {
                    stks[idx / 5].push(Card::Num(2, cards[idx * 2 + 1]  as u8 - 48));
                },
                'z' => {
                    stks[idx / 5].push(Card::Spec(0));
                },
                'f' => {
                    stks[idx / 5].push(Card::Spec(1));
                },
                'm' => {
                    stks[idx / 5].push(Card::Spec(2));
                },
                'l' => {
                    stks[idx / 5].push(Card::Flower);
                },
                _ => {}
            }
        }
        let rev_cards: [u8; 3] = [0, 0, 0];
        let storage = [Card::Empty; 3];
        Self {
            stks,
            storage,
            rev_cards
        }
    }

    fn new_empty() -> Self {
        let mut stks: [Vec<Card>; 8] = Default::default();
        let rev_cards: [u8; 3] = [0, 0, 0];
        let storage = [Card::Empty; 3];
        for idx in 0..8 {
            stks[idx].push(Card::Empty);
        }
        Self {
            stks,
            storage,
            rev_cards
        }
    }

    fn seek_possible_move(&self, pos: (u8, u8, u8)) -> Vec<(u8, u8)> {
        let mut res: Vec<(u8, u8)> = Vec::new();
        let now_card: Card;
        match pos.0 {
            0 => now_card = self.stks[pos.1 as usize][pos.2 as usize],
            1 => now_card = self.storage[pos.1 as usize],
            _ => now_card = Card::Empty
        }
        for idx in 0..8 {
            if pos.1 == idx && pos.0 == 0 {
                continue;
            }
            let last_card = self.stks[idx as usize].last().unwrap();
            if now_card.can_move_front_of(last_card) {
                res.push((0, idx));
            }
        }
        // do not let storage swap with storage
        if pos.0 == 1 {
            return res;
        }
        for idx in 0..3 {
            if pos.1 == idx && pos.0 == 1 {
                continue;
            }
            let last_card = self.storage[idx as usize];
            if now_card.can_move_front_of(&last_card) {
                res.push((1, idx));
            }
        }
        res
    }

    fn move_to(&mut self, from: (u8, u8, u8), to: (u8, u8)) {
        match from.0 {
            0 => {
                match to.0 {
                    0 => {
                        let move_cards: Vec<Card> = self.stks[from.1 as usize].drain(from.2 as usize ..).collect();
                        self.stks[to.1 as usize].extend(move_cards);
                    },
                    1 => {
                        self.storage[to.1 as usize] = self.stks[from.1 as usize].remove(from.2 as usize);
                    },
                    _ => {}
                }
            },
            1 => {
                match to.0 {
                    0 => {
                        self.stks[to.1 as usize].push(self.storage[from.1 as usize]);
                        self.storage[from.1 as usize] = Card::Empty;
                    },
                    _ => {}
                }
            },
            _ => {
            },
        }
    }
    fn scan_spec(&self) -> (bool, u8) {
        if !self.storage_has_empty() {
            return (false, 255);
        }
        let mut map = [0u8; 3];
        for idx in 0..8 {
            let last_card = self.stks[idx].last().unwrap().to_owned();
            if let Card::Spec(i) = last_card {
                map[i as usize] += 1;
            }
        }
        for idx in 0..3 {
            let last_card = self.storage[idx];
            if let Card::Spec(i) = last_card {
                map[i as usize] += 1;
            }
        }
        for idx in 0..3 {
            if map[idx] == 4 {
                return (true, idx as u8);
            }
        }
        (false, 255)
    }

    fn storage_has_empty(&self) -> bool {
        for card in self.storage {
            if let Card::Empty = card {
                return true;
            }
        }
        false
    }
}

fn main() {
}

#[test]
fn test_for_swap() {
    let mut decks = Decks::new_empty();
    decks.stks[0].push(Card::Num(1, 1));
    decks.stks[0].push(Card::Num(1, 2));
    decks.stks[0].push(Card::Num(1, 3));
    decks.move_to((0, 0, 2), (0, 3));
    let mut decks2 = Decks::new_empty();
    decks2.stks[0].push(Card::Num(1, 1));
    decks2.stks[3].push(Card::Num(1, 2));
    decks2.stks[3].push(Card::Num(1, 3));
    assert!(decks2 == decks);
}

#[test]
fn test_create() {
    // len = 80
    let cards = String::from("zzg5mmg2b6r8llg1b4r6g3mmg7r7r5b1ffr2b2mmb8g4ffr9r3g9r1b7b5r4g8ffzzb3zzb9zzffg6mm");
    let decks = Decks::new(cards);
    let res = format!("{:?}", decks);
    let should_be = String::from("Decks { rev_cards: [0, 0, 0], stks: [[Empty, Spec(0), Num(1, 5), Spec(2), Num(1, 2), Num(2, 6)], [Empty, Num(0, 8), Flower, Num(1, 1), Num(2, 4), Num(0, 6)], [Empty, Num(1, 3), Spec(2), Num(1, 7), Num(0, 7), Num(0, 5)], [Empty, Num(2, 1), Spec(1), Num(0, 2), Num(2, 2), Spec(2)], [Empty, Num(2, 8), Num(1, 4), Spec(1), Num(0, 9), Num(0, 3)], [Empty, Num(1, 9), Num(0, 1), Num(2, 7), Num(2, 5), Num(0, 4)], [Empty, Num(1, 8), Spec(1), Spec(0), Num(2, 3), Spec(0)], [Empty, Num(2, 9), Spec(0), Spec(1), Num(1, 6), Spec(2)]], storage: [Empty, Empty, Empty] }");
    assert!(should_be == res);
}
