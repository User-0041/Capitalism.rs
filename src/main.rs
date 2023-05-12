use rand::Rng;

struct Player {
    id: u32,
    coins: u32,
}

impl Player {
    fn new(id: u32) -> Player {
        Player { id, coins: 1 }
    }
}

struct Game {
    players: Vec<Player>,
}

impl Game {
    fn new(num_players: u32) -> Game {
        let mut players = Vec::new();
        for i in 0..num_players {
            players.push(Player::new(i + 1));
        }
        Game { players }
    }

    fn play_round(&mut self) {
        let player1_index = rand::thread_rng().gen_range(0..self.players.len());
        let mut player2_index = rand::thread_rng().gen_range(0..self.players.len() - 1);
        if player2_index >= player1_index {
            player2_index += 1;
        }

        let player1_id = self.players[player1_index].id;
        let player2_id = self.players[player2_index].id;

        let winner = if rand::random::<bool>() { player1_id } else { player2_id };
        let loser = if winner == player1_id { player2_id } else { player1_id };

        self.update_coins(winner, loser);
        self.print_game_state();
    }

    fn update_coins(&mut self, winner_id: u32, loser_id: u32) {
        let mut losers = Vec::new();

        for (i, player) in self.players.iter_mut().enumerate() {
            if player.id == winner_id {
                player.coins += 1;
            } else if player.id == loser_id {
                player.coins -= 1;
                if player.coins == 0 {
                    losers.push(i);
                }
            }
        }

        for i in losers.into_iter().rev() {
            self.players.remove(i);
        }
    }

    fn print_game_state(&self) {
        println!("Current Game State:");
        for player in &self.players {
            let coins = "*".repeat(player.coins as usize);
            println!("Player {} - Coins: {}", player.id, coins);
        }
        println!("---------------------");
    }

    fn play_game(&mut self) {
        self.print_game_state();
        while self.players.len() > 1 {
            self.play_round();
        }

        println!("Player {} wins the game!", self.players[0].id);
    }
}

fn main() {
    let num_players = 5;
    let mut game = Game::new(num_players);
    game.play_game();
}