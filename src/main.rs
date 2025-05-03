mod uno;
use uno::shownomercy::{self, ShowNoMercyTrait};

#[allow(unused_variables)] // only hide unused variable warnings
fn main() {
    // let vec= vec![String::from("Sachin"),String::from("Taushi")];
    // let mut game= shownomercy::ShowNoMercy::new(vec, vec.len());
    let players= vec![String::from("Sachin"),String::from("Ai"),String::from("Ai2"),String::from("Ai3"),String::from("Ai4"),String::from("Ai5")];
    let len=players.len();
    let mut game= shownomercy::ShowNoMercy::new(players.to_owned(),  len as u8).unwrap();
    game.start_play();
    
}
#[cfg(test)]
mod tests {
    use crate::uno::shownomercy::{self, Card, CardType, Color, ShowNoMercy, ShowNoMercyTrait,Player};


    #[test]
    fn test_unomercy_initialization() {
        let vec= vec![String::from("Hello"),String::from("HI"),String::from("WHATSAP"),String::from("WTF"),String::from("WW")];
        let mut game= shownomercy::ShowNoMercy::new(vec, 5).unwrap();
        assert_eq!(game.players_length,5);
    }

    #[test]
    fn test_discard_all_chain() {
        let mut game = ShowNoMercy::new(vec!["Alice".into()], 1).unwrap();
        game.players.push(
            Player{
                deck_index:String::from("AliceDeck"),
                name:String::from("A")
            }
        );
        game.player_deck.insert("A".into(), vec![
            Card { card_type: CardType::DISCARDALL, color: Color::RED },
            Card { card_type: CardType::Number(5), color: Color::RED },
            Card { card_type: CardType::Number(3), color: Color::RED },
        ]);
        game.current_player=0;
        let initial_len = game.player_deck["A"].len();
        println!("{initial_len}");
        game.playing_stack.push(Card { card_type: CardType::Number(3), color: Color::RED });
        let check=game.check_is_move_valid(&Card { card_type: CardType::DISCARDALL, color: Color::RED }, 0);
        match check {
            Ok(_)=>{
                game.playing_stack.push(Card { card_type: CardType::DISCARDALL, color: Color::RED });
            },Err(_)=>{
            }
        }
        assert_eq!(game.playing_stack.last().unwrap().card_type, CardType::DISCARDALL);
        let final_len = game.player_deck["A"].len();
        println!("{final_len}");
        assert!(final_len < initial_len); // RED cards discarded
    }
}