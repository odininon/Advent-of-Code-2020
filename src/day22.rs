#[derive(Debug, Clone, Eq, PartialEq)]
struct Deck {
    cards: Vec<i32>,
}

impl Deck {
    fn new(mut cards: Vec<i32>) -> Self {
        cards.reverse();
        Deck { cards }
    }

    fn play_card(&mut self) -> i32 {
        self.cards.pop().unwrap()
    }

    fn win_cards(&mut self, cards: Vec<i32>) {
        self.cards.reverse();
        for card in cards {
            self.cards.push(card);
        }
        self.cards.reverse();
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for (i, elm) in self.cards.iter().enumerate() {
            score += (i + 1) as i32 * *elm;
        }

        score
    }
}

#[derive(Debug)]
struct Game {
    player1: Deck,
    player2: Deck,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Player {
    Player1,
    Player2,
}

impl Game {
    fn new(player1: &Deck, player2: &Deck) -> Self {
        Game {
            player1: player1.clone(),
            player2: player2.clone(),
        }
    }

    fn play_round(&mut self) {
        let player1_card = self.player1.play_card();
        let player2_card = self.player2.play_card();

        if player1_card > player2_card {
            self.player1.win_cards(vec![player1_card, player2_card])
        } else {
            self.player2.win_cards(vec![player2_card, player1_card])
        }
    }

    fn is_over(&self) -> bool {
        self.player1.cards.len() == 0 as usize || self.player2.cards.len() == 0 as usize
    }

    fn play(&mut self) -> Player {
        while !self.is_over() {
            self.play_round()
        }

        if self.player1.cards.len() == 0 as usize {
            Player::Player2
        } else {
            Player::Player1
        }
    }

    fn final_score(&self) -> i32 {
        let deck_to_score = if self.player1.cards.len() == 0 as usize {
            &self.player2
        } else {
            &self.player1
        };

        deck_to_score.score()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct RecursionGame {
    player1: Deck,
    player2: Deck,
    player1_history: Vec<Deck>,
    player2_history: Vec<Deck>,
}

impl RecursionGame {
    fn new(player1: &Deck, player2: &Deck) -> Self {
        RecursionGame {
            player1: player1.clone(),
            player2: player2.clone(),
            player1_history: vec![],
            player2_history: vec![],
        }
    }

    fn play(&mut self) -> Player {
        while !(self.player1.cards.len() == 0 as usize || self.player2.cards.len() == 0 as usize) {
            for deck in &self.player1_history {
                if deck == &self.player1 {
                    return Player::Player1;
                }
            }

            for deck in &self.player2_history {
                if deck == &self.player2 {
                    return Player::Player1;
                }
            }

            self.player1_history.push(self.player1.clone());
            self.player2_history.push(self.player2.clone());

            let player1_card = self.player1.play_card();
            let player2_card = self.player2.play_card();

            if player1_card as usize <= self.player1.cards.len()
                && player2_card as usize <= self.player2.cards.len()
            {
                let mut new_player1_deck = vec![];
                let mut player_1_copy = self.player1.cards.clone();
                while new_player1_deck.len() < player1_card as usize {
                    new_player1_deck.push(player_1_copy.pop().unwrap())
                }

                let deck1 = Deck::new(new_player1_deck);

                let mut new_player2_deck = vec![];
                let mut player_2_copy = self.player2.cards.clone();
                while new_player2_deck.len() < player2_card as usize {
                    new_player2_deck.push(player_2_copy.pop().unwrap())
                }

                let deck2 = Deck::new(new_player2_deck);

                let mut new_game = RecursionGame::new(&deck1, &deck2);
                match new_game.play() {
                    Player::Player1 => self.player1.win_cards(vec![player1_card, player2_card]),
                    Player::Player2 => {
                        self.player2.win_cards(vec![player2_card, player1_card]);
                    }
                }
            } else if player1_card > player2_card {
                self.player1.win_cards(vec![player1_card, player2_card])
            } else {
                self.player2.win_cards(vec![player2_card, player1_card])
            }
        }

        if self.player1.cards.len() == 0 as usize {
            Player::Player2
        } else {
            Player::Player1
        }
    }

    fn final_score(&self) -> i32 {
        let deck_to_score = if self.player1.cards.len() == 0 as usize {
            &self.player2
        } else {
            &self.player1
        };

        deck_to_score.score()
    }
}

pub fn solution(_input: Vec<String>) -> [i32; 2] {
    // let player1 = Deck::new(vec![9, 2, 6, 3, 1]);
    // let player2 = Deck::new(vec![5, 8, 4, 7, 10]);
    let player1 = Deck::new(vec![
        41, 33, 20, 32, 7, 45, 2, 12, 14, 29, 49, 37, 6, 11, 39, 46, 47, 38, 23, 22, 28, 10, 36,
        35, 24,
    ]);
    let player2 = Deck::new(vec![
        17, 4, 44, 9, 27, 18, 30, 42, 21, 26, 16, 48, 8, 15, 34, 50, 19, 43, 25, 1, 13, 31, 3, 5,
        40,
    ]);

    let mut game = Game::new(&player1, &player2);
    game.play();

    let mut recursion_game = RecursionGame::new(&player1, &player2);
    recursion_game.play();

    [game.final_score(), recursion_game.final_score()]
}
