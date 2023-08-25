use std::collections::HashSet;

// --- struct ---

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Card {
    Flower,
    Num(u8, u8),
    Spec(u8),
    Empty,
    Disable
}

impl Card {
    const HASH_STK: [[char; 9]; 3] = [
        ['1', '2', '3', '4', '5', '6', '7', '8', '9'],
        ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o'],
        ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l']
    ];
    const HASH_SPEC: [char; 3] = ['z', 'x', 'c'];

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

    fn into_char(&self) -> char {
        match self {
            Card::Num(tp, x) => Card::HASH_STK[tp.to_owned() as usize][(x.to_owned() - 1) as usize],
            Card::Spec(x) => Card::HASH_SPEC[x.to_owned() as usize],
            Card::Flower => 'v',
            Card::Empty => 'b',
            Card::Disable => 'n',
        }
    }
}

enum MoveErr {
    CantMove
}

#[derive(Debug, PartialEq)]
struct Decks {
    rev_cards: [u8; 3],
    stks: [Vec<Card>; 8],
    storage: [Card; 3]
}

impl Decks {
    fn new(cards: String) -> Self {
        let mut stks: [Vec<Card>; 8] = Default::default();
        for idx in 0..8 {
            stks[idx].push(Card::Empty);
        }

        let cards: Vec<char> = cards.chars().collect();
        for idx in 0..40 {
            match cards[idx * 2] {
                'r' => stks[idx / 5].push(Card::Num(0, cards[idx * 2 + 1]  as u8 - 48)),
                'g' => stks[idx / 5].push(Card::Num(1, cards[idx * 2 + 1]  as u8 - 48)),
                'b' => stks[idx / 5].push(Card::Num(2, cards[idx * 2 + 1]  as u8 - 48)),
                'z' => stks[idx / 5].push(Card::Spec(0)),
                'f' => stks[idx / 5].push(Card::Spec(1)),
                'm' => stks[idx / 5].push(Card::Spec(2)),
                'l' => stks[idx / 5].push(Card::Flower),
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

    /// try_to_move(from, to) -> (to, from) return the back way
    fn try_to_move(&mut self, from: (u8, u8, u8), to: (u8, u8)) -> Result<((u8, u8, u8), (u8, u8)), MoveErr> {
        match from.0 {
            0 => {
                match to.0 {
                    0 => {
                        let fidx = self.stks[to.1 as usize].len() as u8;
                        let move_cards: Vec<Card> = self.stks[from.1 as usize].drain(from.2 as usize ..).collect();
                        self.stks[to.1 as usize].extend(move_cards);
                        Ok(((to.0, to.1, fidx), (from.0, from.1)))
                    },
                    1 => {
                        self.storage[to.1 as usize] = self.stks[from.1 as usize].remove(from.2 as usize);
                        Ok(((to.0, to.1, 233), (from.0, from.1)))
                    },
                    _ => {
                        Err(MoveErr::CantMove)
                    }
                }
            },
            1 => {
                match to.0 {
                    0 => {
                        let fidx = self.stks[to.1 as usize].len() as u8;
                        self.stks[to.1 as usize].push(self.storage[from.1 as usize]);
                        self.storage[from.1 as usize] = Card::Empty;
                        Ok(((to.0, to.1, fidx), (from.0, from.1)))
                    },
                    _ => {
                        Err(MoveErr::CantMove)
                    }
                }
            },
            _ => {
                Err(MoveErr::CantMove)
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

    fn into_string(&self) -> String {
        let mut tmp = Vec::from(self.storage);
        tmp.sort();
        let mut flag = String::from("");
        for i in tmp {
            flag.push(i.into_char());
        }
        for idx in 0..8 {
            for tmp in &self.stks[idx] {
                if let Card::Empty = tmp {
                    continue;
                }
                flag.push(tmp.into_char());
            }
            flag.push(';');
        }
        flag
    }

    fn cut_num(&mut self, from: (u8, u8)) -> Result<(u8, u8, u8), MoveErr> {
        let now_card: Card;
        match from.0 {
            0 => now_card = *self.stks[from.1 as usize].last().unwrap(),
            1 => now_card = self.storage[from.1 as usize],
            _ => {
                return Err(MoveErr::CantMove);
            }
        }
        if let Card::Num(tp, nu) = now_card {
            if nu == self.rev_cards[tp as usize] + 1 {
                match from.0 {
                    0 => {
                        self.stks[from.1 as usize].pop().unwrap();
                    },
                    1 => {
                        self.storage[from.1 as usize] = Card::Empty;
                    },
                    _ => {
                        return Err(MoveErr::CantMove);
                    }
                }
                self.rev_cards[tp as usize] += 1;
                Ok((from.0, from.1, tp))
            }
            else {
                Err(MoveErr::CantMove)
            }
        }
        else {
            Err(MoveErr::CantMove)
        }
    }

    fn recover_cut_num(&mut self, to: (u8, u8, u8)) -> Result<(), MoveErr> {
        match to.0 {
            0 => {
                self.stks[to.1 as usize].push(Card::Num(to.2, self.rev_cards[to.2 as usize]));
            },
            1 => {
                self.storage[to.1 as usize] = Card::Num(to.2, self.rev_cards[to.2 as usize]);
            },
            _ => {
                return Err(MoveErr::CantMove);
            }
        }
        self.rev_cards[to.2 as usize] -= 1;
        Ok(())
    }
}

// --- solve ---

struct Solve {
    flag_set: HashSet<String>,
    deck: Decks,
    solution: Vec<String>
}

impl Solve {
    fn new(init: String) -> Self {
        let deck = Decks::new(init);
        let flag_set = HashSet::<String>::new();
        let solution = Vec::<String>::new();
        Self {
            deck,
            flag_set,
            solution
        }
    }
    fn mark(&mut self) {
        self.flag_set.insert(self.deck.into_string());
    }

    fn dfs(mut now: Solve) -> Solve {
        now
    }
}

fn main() {
}

// --- test ---

#[test]
fn test_for_swap() {
    let mut decks = Decks::new_empty();
    decks.stks[0].push(Card::Num(1, 1));
    decks.stks[0].push(Card::Num(1, 2));
    decks.stks[0].push(Card::Num(1, 3));
    assert!(decks.try_to_move((0, 0, 2), (0, 3)).is_ok());
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

#[test]
fn test_hash() {
    let cards = String::from("zzg5mmg2b6r8llg1b4r6g3mmg7r7r5b1ffr2b2mmb8g4ffr9r3g9r1b7b5r4g8ffzzb3zzb9zzffg6mm");
    let decks = Decks::new(cards);
    assert!(decks.into_string() == "bbbztcwh;8vqf6;ecu75;ax2sc;krx93;o1jg4;ixzdz;lzxyc;");
}

#[test]
fn test_sol_sturt() {
    let cards = String::from("zzg5mmg2b6r8llg1b4r6g3mmg7r7r5b1ffr2b2mmb8g4ffr9r3g9r1b7b5r4g8ffzzb3zzb9zzffg6mm");
    let mut sol = Solve::new(cards);
    sol.mark();
    assert!(sol.flag_set.contains("bbbztcwh;8vqf6;ecu75;ax2sc;krx93;o1jg4;ixzdz;lzxyc;"));
}
