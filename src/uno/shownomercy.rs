use colored::Colorize;
use rand::Rng;
use std::{
    collections::HashMap,
    io::{self, Write},
};
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CardType {
    Number(i8),   //2*9=18*4=72
    SEVENPASS,    //2
    ZEROPASS,     //1*4=4
    DRAW2,        //3*4=12
    DRAW4,        //3*4=12
    DRAW4REVERSE, //8
    DRAW6,        //4
    DRAW10,       //4
    WILDROULETTE, //8
    REVERSE,      //3*4=12
    SKIP,         //3*4=12
    SKIPEVERYONE, //2*4=8
    DISCARDALL,   //3*4=12
}
#[derive(Clone,Copy, Debug, PartialEq)]
pub(crate) enum Color {
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Card {
    pub(crate) card_type: CardType,
    pub(crate) color: Color,
}
#[derive(Debug)]
pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) deck_index: String,
}
pub(crate) struct PlayerRank {
    pub(crate) name: String,
    pub(crate) rank: usize,
}

pub struct ShowNoMercy {
    pub(crate) leaderboard: Vec<PlayerRank>,
    pub(crate)  players: Vec<Player>,
    pub(crate) player_deck: HashMap<String, Vec<Card>>,
    pub(crate) deck: Vec<Card>,
    pub(crate) playing_stack: Vec<Card>,
    pub(crate) current_player: u8,
    pub(crate) direction_reverse: bool,
    pub(crate) players_length: u8,
    pub(crate) pick_draw_cards: u8,
    pub(crate) card_to_select: Option<Color>,
    pub(crate) is_routlet_on_you:bool,
    top_players:usize,
    bottom_players:usize
}
#[derive(Debug,PartialEq)]
pub(crate) enum GameError {
    InvalidCardIndex,
    InvalidMove,
    InvalidPlayer,
    YouCannotSelectYourself,
    NOTMORETHAN6PLAYERS
}
pub trait ShowNoMercyTrait {
    fn maybe_refill_deck(&mut self);
    fn player_won(&mut self, player: usize, deck_index: &String);
    fn pick_draws(&mut self);
    fn pick_up_one(&mut self);
    fn wild_roulette_pickup(&mut self, card_color: Option<Color>);
    fn get_card(&mut self,player: u8) -> (Option<Card>,Option<usize>);
    fn from_str(&self,input: &str) -> Option<Color>;
    fn select_color(&mut self);
    fn get_color_to_select(&self) -> Option<Color>;
    fn discard_all_card(&mut self, card: &Card);
    fn zeropass(&mut self,card_index:usize);
    fn check_if_player_valid(&mut self, index: usize,card_index:usize) -> Result<(), GameError>;
    fn sevenpass(&mut self,card_index:usize);
    fn display_players(&self);
    fn get_current_card(&self) -> Card;
    fn skip_card(&mut self);
    fn next_turn(&mut self);
    fn check_is_move_valid(&mut self,card:&Card,card_index:usize) -> Result<(), GameError>;
    fn get_card_of_current_player(&mut self, player: u8) -> Option<Card> ;
    fn start_play(&mut self);
    fn print_current_card(& mut self);
    fn display_card_of_player(&mut self, player_index: u8);
    fn new(player_names: Vec<String>, no_of_players: u8) -> Result<Self, GameError> where Self: Sized;
    fn initialize_cards(&mut self);
    fn shuffle_cards(&mut self);
    fn give_cards_to_players(&mut self, player_names: Vec<String>, no_of_players: u8);
    fn initialize_color_card(&mut self, length: u8, color: Color, deck: &mut Vec<Card>);
    fn initialize_black_cards(&mut self, deck: &mut Vec<Card>);
    fn print_deck(&self);
}



impl ShowNoMercyTrait for ShowNoMercy {
    fn new(player_names: Vec<String>, no_of_players: u8) -> Result<Self, GameError>{
        if no_of_players>6 {
            return Err(GameError::NOTMORETHAN6PLAYERS);
        }
        let mut game = ShowNoMercy {
            leaderboard:vec![],
            top_players:1,
            bottom_players:no_of_players as usize,
            players: vec![],
            current_player: 0,
            deck: vec![],
            player_deck: HashMap::new(),
            direction_reverse: false,
            playing_stack: vec![],
            players_length: no_of_players,
            pick_draw_cards: 0,
            card_to_select:None,
            is_routlet_on_you:false
        };
        game.initialize_cards();
        game.shuffle_cards();
        game.shuffle_cards();
        game.shuffle_cards();
        game.shuffle_cards();
        game.shuffle_cards();
        game.give_cards_to_players(player_names, no_of_players);
        game.print_deck();
        for _ in 0..5  {
            println!("\n");
        }
        Ok(game)
    }
    fn print_deck(&self) {
        for (_, card) in self.deck.iter().enumerate() {
            let fstr = format!("{:?} of {:?}", card.card_type, card.color);
            match card.color {
                Color::RED => {
                    println!("{}", fstr.red());
                }
                Color::GREEN => {
                    println!("{}", fstr.green());
                }
                Color::BLUE => {
                    println!("{}", fstr.blue());
                }
                Color::YELLOW => {
                    println!("{}", fstr.yellow());
                }
                Color::BLACK => {
                    println!("{}", fstr.black());
                }
            };
        }
    }
    fn print_current_card(&mut self) {
        let current = &self.playing_stack[self.playing_stack.len() - 1];
        let fstr = format!(
            "CURRENT CARD PLAYED {:?} of {:?}",
            current.card_type, current.color
        );
        if let Some(color) =self.card_to_select  {
            if current.color!=Color::BLACK && color!=current.color {
                self.card_to_select=Some(current.color);
            }
        }
        let color=self.card_to_select;
       
        match current.color {
            Color::RED => {
                println!("{}", fstr.red());
            }
            Color::GREEN => {
                println!("{}", fstr.green());
            }
            Color::BLUE => {
                println!("{}", fstr.blue());
            }
            Color::YELLOW => {
                println!("{}", fstr.yellow());
            }
            Color::BLACK => {
                println!("{}", fstr.black());
            }
        };
        match color {
            Some(c)=>{
                let fmtt= format!("COLOR TO PLAY : {:?}",c);
                match c {
                    Color::RED => {
                        println!("{}", fmtt.red());
                    }
                    Color::GREEN => {
                        println!("{}", fmtt.green());
                    }
                    Color::BLUE => {
                        println!("{}", fmtt.blue());
                    }
                    Color::YELLOW => {
                        println!("{}", fmtt.yellow());
                    }
                    Color::BLACK => {
                        println!("{}", fmtt.black());
                    }
                };
            },None=>{}
        }
    }
    fn get_current_card(&self) -> Card {
        return self.playing_stack[self.playing_stack.len() - 1].clone();
    }
    fn initialize_cards(&mut self) {
        let mut deck = Vec::new();
        self.initialize_color_card(9, Color::RED, &mut deck);
        self.initialize_color_card(9, Color::BLUE, &mut deck);
        self.initialize_color_card(9, Color::GREEN, &mut deck);
        self.initialize_color_card(9, Color::YELLOW, &mut deck);
        self.initialize_black_cards(&mut deck);
        self.deck = deck;
    }
    fn initialize_color_card(&mut self, length: u8, color: Color, deck: &mut Vec<Card>) {
        for i in 0..length + 1 {
            if i == 0 {
                deck.push(Card {
                    card_type: CardType::ZEROPASS,
                    color: color.clone(),
                });
            } else if i == 7 {
                deck.push(Card {
                    card_type: CardType::SEVENPASS,
                    color: color.clone(),
                });
                deck.push(Card {
                    card_type: CardType::SEVENPASS,
                    color: color.clone(),
                });
            } else {
                deck.push(Card {
                    card_type: CardType::Number(i as i8),
                    color: color.clone(),
                });
                deck.push(Card {
                    card_type: CardType::Number(i as i8),
                    color: color.clone(),
                });
            }
        }
        for i in 1..4 {
            if i != 3 {
                deck.push(Card {
                    card_type: CardType::SKIPEVERYONE,
                    color: color.clone(),
                })
            }
            deck.push(Card {
                card_type: CardType::DISCARDALL,
                color: color.clone(),
            });
            deck.push(Card {
                card_type: CardType::DRAW2,
                color: color.clone(),
            });
            deck.push(Card {
                card_type: CardType::DRAW4,
                color: color.clone(),
            });
            deck.push(Card {
                card_type: CardType::REVERSE,
                color: color.clone(),
            });
            deck.push(Card {
                card_type: CardType::SKIP,
                color: color.clone(),
            });
        }
    }
    fn initialize_black_cards(&mut self, deck: &mut Vec<Card>) {
        for i in 1..9 {
            if i < 5 {
                deck.push(Card {
                    card_type: CardType::DRAW6,
                    color: Color::BLACK,
                });
                deck.push(Card {
                    card_type: CardType::DRAW10,
                    color: Color::BLACK,
                });
            }
            deck.push(Card {
                card_type: CardType::DRAW4REVERSE,
                color: Color::BLACK,
            });
            deck.push(Card {
                card_type: CardType::WILDROULETTE,
                color: Color::BLACK,
            });
        }
    }
    fn shuffle_cards(&mut self) {
        let card_length = self.deck.len();
        let mut rng = rand::rng();
        for i in (1..card_length).rev() {
            let j = rng.random_range(0..=i);
            self.deck.swap(i, j);
        }
    }
    fn give_cards_to_players(&mut self, player_names: Vec<String>, no_of_players: u8) {
        let mut players = Vec::new();
        let mut player_deck = HashMap::new();
        for i in 0..no_of_players {
            let ind = char::from_u32(i as u32 + 65).unwrap().to_string();
            players.push(Player {
                name: player_names[i as usize].clone(),
                deck_index: ind,
            });
        }
        for _ in 1..8 {
            for j in 0..no_of_players {
                let card = self.deck.pop().unwrap();
                let player = &players[j as usize];
                player_deck
                    .entry(player.deck_index.clone())
                    .or_insert_with(Vec::new)
                    .push(card);
            }
        }
        self.player_deck = player_deck;
        self.players = players;
    }
    fn display_card_of_player(&mut self, player_index: u8) {
        let player = &self.players[player_index as usize];
        println!("TURN OF {}", player.name.bright_cyan());
        let cards = self.player_deck.get(&player.deck_index).unwrap();
        for (i, card) in cards.iter().enumerate() {
            let fstr = format!("{:?} of {:?}", card.card_type, card.color);
            match card.color {
                Color::RED => {
                    println!("{}) {}", i, fstr.red());
                }
                Color::GREEN => {
                    println!("{}) {}", i, fstr.green());
                }
                Color::BLUE => {
                    println!("{}) {}", i, fstr.blue());
                }
                Color::YELLOW => {
                    println!("{}) {}", i, fstr.yellow());
                }
                Color::BLACK => {
                    println!("{}) {}", i, fstr.black());
                }
            };
        }
        
    }
    fn start_play(&mut self) {
        loop {
            let start = self.deck.pop().unwrap();
            self.playing_stack.push(start);
            let card = self.get_current_card();
            match card.card_type {
                CardType::DRAW10 => {
                    self.pick_draw_cards = 10;
                    self.card_to_select=Some(Color::RED);
                    break;
                }
                CardType::DRAW2 => {
                    self.pick_draw_cards = 2;
                    break;
                }
                CardType::DRAW4 => {
                    self.pick_draw_cards = 4;
                    break;
                }
                CardType::DRAW4REVERSE => {
                    self.pick_draw_cards = 4;
                    self.direction_reverse = !self.direction_reverse;
                    //any random color
                    self.card_to_select=Some(Color::RED);
                    break;
                }
                CardType::SKIP => {
                    self.skip_card();
                    break;
                }
                CardType::DRAW6 => {
                    self.pick_draw_cards = 6;
                    self.card_to_select=Some(Color::RED);
                    break;
                }
                CardType::Number(_) => {
                    break;
                }
                CardType::SEVENPASS => {
                    continue;
                }
                CardType::ZEROPASS => {
                    continue;
                }
                CardType::WILDROULETTE => {
                    continue;
                }
                CardType::REVERSE => {
                    self.direction_reverse = !self.direction_reverse;
                    break;
                }
                CardType::SKIPEVERYONE => {
                    break;
                }
                CardType::DISCARDALL => {
                    continue;
                }
            }
        }
        loop {
            if self.players.is_empty() {
                break;
            }
            let player_index = self.current_player;
            if player_index  >= self.players.len() as u8 {
                self.current_player = 0;
            }
            self.print_current_card();
            self.maybe_refill_deck();
            let deck_index;
            let player_name;
             {
                 let player = &self.players[player_index as usize];
                 deck_index = player.deck_index.clone(); 
                 player_name=player.name.clone();
             }
            
                let card = self.get_card_of_current_player(player_index);
                match card {
                    Some(c)=>{
                        if c.card_type==CardType::SKIPEVERYONE {
                            self.playing_stack.push(c);
                            for _ in 0..5  {
                                println!("\n");
                            }
                            continue;
                        }
                        if c.card_type==CardType::SKIP{
                            self.playing_stack.push(c);
                            for _ in 0..5  {
                                println!("\n");
                            }
                            continue;
                        }
                        self.playing_stack.push(c);
                    }None=>{}
                };
                if let Some(cards) = self.player_deck.get(&deck_index) {
                    if cards.is_empty(){
                        self.player_won(player_index as usize, &deck_index);
                        if self.players_length <= 0 {
                            break;
                        }
                        if self.current_player >= self.players.len() as u8 {
                            self.current_player = 0;
                        }
                        if self.players_length==1 {
                            let remove=self.players.remove(0);
                            self.leaderboard.push(PlayerRank { name: remove.name, rank: self.bottom_players });
                            break;
                        }
                        continue; 
                    }
                    if cards.len() >= 25 {
                        println!("PLAYER : {} ELEMINATED",player_name);
                        let remove=self.players.remove(player_index as usize);
                        self.leaderboard.push(PlayerRank { name: remove.name, rank: self.bottom_players });
                        self.bottom_players-=1;
                        self.players_length-=1;
                        self.player_deck.remove(&deck_index);
                        if self.players_length == 1 {
                            println!("REACHED HERE");
                            let winner = &self.players[0].deck_index.clone();
                            self.player_won(0, winner);
                            break;
                        }
                        if self.current_player >= self.players.len() as u8 {
                            self.current_player = 0;
                        }
                    }
                    
                } else {
                    continue;
                }
            
           
            for _ in 0..5  {
                println!("\n");
            }
            if self.players_length<=0 {
                break;
            }
            self.next_turn();
        }
        self.leaderboard.sort_by(|x,y| x.rank.cmp(&y.rank));
        for (_,p) in   self.leaderboard.iter().enumerate(){
            println!("{} | {}",p.rank,p.name);
        }
    }
    fn get_card_of_current_player(&mut self, player: u8) -> Option<Card> {
        self.display_card_of_player(player);
        let (card,index)= self.get_card(player);
        let player_curr = &self.players[player as usize];
        let cards= self.player_deck.get_mut(&player_curr.deck_index);
        match card {
            Some(c)=>{
                match cards {
                    Some(deck)=>{
                        if c.card_type==CardType::SEVENPASS || c.card_type==CardType::ZEROPASS || c.card_type==CardType::DISCARDALL {
                            return Some(c);
                        }
                        println!("{} played {:?}",player_curr.name,c);
                        let index=index.unwrap();
                        deck.remove(index);
                        return Some(c);
                    }None=>{
                        return None;
                    }
                }

            }None=>{
                return None;
            }
        }
    }
    fn get_card(&mut self,player: u8)->(Option<Card>,Option<usize>){
        let player = &self.players[player as usize];
        let cards = self.player_deck.get(&player.deck_index).unwrap().clone();
        loop {
            println!("TO PICK UP CARD ENTER : pick");//self.pick_up_one();
            if self.pick_draw_cards>0 {
            println!("TO PICK UP DRAW CARD ENTER current number of draw {}: pick draws",self.pick_draw_cards);//self.pick_draws();
            }
            if self.card_to_select!=None && self.is_routlet_on_you==true {
                println!("WILD ROULETEE ON YOU....... PICK UP CARD UNTIL {:?}",self.card_to_select);
                self.wild_roulette_pickup(self.card_to_select);
                return (None,None);
            }
            println!("Enter Index of card from your deck to play :");
            let _ = io::stdout().flush();
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Error reading input. Try again.");
                continue;
            }
            let trimmed = input.trim();
            match trimmed {
                "pick" => {
                    if self.pick_draw_cards!=0 {
                        println!("YOU CANNOT PICK UP CARD IF THERE ARE DRAWS CARD TO PICK UP");
                        continue;
                    }
                    println!("You picked a card.");
                    self.pick_up_one();
                    return (None,None);
                }
                "pick draws" => {
                    println!("You picked a draw card.");
                    if self.pick_draw_cards<=0 {
                        println!("NO DRAW CARDS TO PICKUP");
                        continue;
                    }
                    self.pick_draws();
                    return (None,None);
                }
                _ => {
                    match trimmed.parse::<usize>() {
                        Ok(index) => {
                            let copy_card=cards.get(index).cloned();
                            
                            if let Some(card) = copy_card {
                                match self.check_is_move_valid(&card,index) {
                                    Ok(_) =>{
                                         return (Some(card),Some(index))
                                        },
                                    Err(e) => {
                                        println!("ERROR: {:?} — Retry", e);
                                       
                                    }
                                }
                            } else {
                                println!("Invalid card index. Try again.");
                            }
                        }
                        Err(_) => {
                            println!("Invalid input. Enter a number or 'pick'.");
                        }
                    }
                }
            }
        }
    }
    fn next_turn(&mut self) {
        if self.direction_reverse {
            self.current_player =
                (self.current_player + self.players_length - 1) % self.players_length;
        } else {
            self.current_player = (self.current_player + 1) % self.players_length;
        }
    }   
    fn wild_roulette_pickup(&mut self,card_color:Option<Color>){
        self.maybe_refill_deck();
        let color = match card_color {
            Some(c) => c,
            None => return,
        };
        let player = &self.players[self.current_player as usize];
        let cards = self.player_deck.get_mut(&player.deck_index).unwrap();
        loop {
            let start = self.deck.pop();
            match start {
                Some(poped_card)=>{
                    if  poped_card.color==color{
                        cards.push(poped_card);
                        self.is_routlet_on_you=false;
                        break;
                    }else{
                        cards.push(poped_card);
                    }
                }None=>{
                    continue;
                }
                
            }
           
        }
    }
    fn check_is_move_valid(&mut self,card:&Card,card_index:usize) -> Result<(), GameError> {
                let current_card = self.get_current_card();
              
                if current_card.card_type == card.card_type
                    || current_card.color == card.color
                    || current_card.color == Color::BLACK
                    || card.color== Color::BLACK
                    || self.pick_draw_cards > 0
                    || (current_card.card_type==CardType::WILDROULETTE && self.card_to_select!=None)
                {
                    if current_card.card_type == card.card_type {
                        if self.pick_draw_cards > 0 {
                            match card.card_type {
                                CardType::Number(_) => {}
                                CardType::SEVENPASS => {}
                                CardType::ZEROPASS => {}
                                CardType::DRAW2 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 2;
                                }
                                CardType::DRAW4 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 4;
                                }
                                CardType::DRAW4REVERSE => {
                                    self.pick_draw_cards = self.pick_draw_cards + 4;
                                    self.direction_reverse = !self.direction_reverse;
                                    //select color for next
                                    self.select_color();
                                }
                                CardType::DRAW6 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 6;
                                    //select color for next
                                    self.select_color();
                                }
                                CardType::DRAW10 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 10;
                                    //select color for next
                                    self.select_color();
                                }
                                CardType::WILDROULETTE => {}
                                CardType::REVERSE => {}
                                CardType::SKIP => {}
                                CardType::SKIPEVERYONE => {}
                                CardType::DISCARDALL => {}
                            }
                            return Ok(());
                        }
                        match card.card_type {
                            CardType::Number(_) => {}
                            CardType::SEVENPASS => {
                                self.sevenpass(card_index);
                            }
                            CardType::ZEROPASS => {
                                println!("ZERO PASS");
                                self.zeropass(card_index);
                            }
                            CardType::DRAW2 => {
                                self.pick_draw_cards = self.pick_draw_cards + 2;
                            }
                            CardType::DRAW4 => {
                                self.pick_draw_cards = self.pick_draw_cards + 4;
                            }
                            CardType::DRAW4REVERSE => {
                                self.pick_draw_cards = self.pick_draw_cards + 4;
                                self.direction_reverse = !self.direction_reverse;
                                //ask for colour
                                self.select_color();
                            }
                            CardType::DRAW6 => {
                                self.pick_draw_cards = self.pick_draw_cards + 6;
                                //ask for colour
                                self.select_color();
                            }
                            CardType::DRAW10 => {
                                self.pick_draw_cards = self.pick_draw_cards + 10;
                                //ask for colour
                                self.select_color();
                            }
                            CardType::WILDROULETTE => {
                                self.select_color();
                                self.is_routlet_on_you=true;
                            }
                            CardType::REVERSE => {
                                self.direction_reverse = !self.direction_reverse;
                            }
                            CardType::SKIP => {
                                self.skip_card();
                            }
                            CardType::SKIPEVERYONE => {}
                            CardType::DISCARDALL => {
                                self.discard_all_card(&card);
                            }
                        };
                        return Ok(());
                    } else if self.pick_draw_cards > 0 {
                        if current_card.card_type == CardType::DRAW2 {
                            match card.card_type {
                                CardType::Number(_) => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SEVENPASS => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::ZEROPASS => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW2 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 2;
                                    return Ok(());
                                }
                                CardType::DRAW4 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 4;
                                    return Ok(());
                                }
                                CardType::DRAW4REVERSE => {
                                    self.pick_draw_cards = self.pick_draw_cards + 4;
                                    self.direction_reverse = !self.direction_reverse;
                                    //ask for colour
                                    self.select_color();
                                    return Ok(());
                                }
                                CardType::DRAW6 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 6;
                                    //ask for colour
                                    self.select_color();
                                    return Ok(());
                                }
                                CardType::DRAW10 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 10;
                                    //ask for colour
                                    self.select_color();
                                    return Ok(());
                                }
                                CardType::WILDROULETTE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::REVERSE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SKIP => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SKIPEVERYONE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DISCARDALL => {
                                    return Err(GameError::InvalidMove);
                                }
                            };
                        } else if current_card.card_type == CardType::DRAW4
                            || current_card.card_type == CardType::DRAW4REVERSE
                        {
                            match card.card_type {
                                CardType::Number(_) => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SEVENPASS => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::ZEROPASS => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW2 => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW4 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 4;
                                    return Ok(());
                                }
                                CardType::DRAW4REVERSE => {
                                    self.pick_draw_cards = self.pick_draw_cards + 4;
                                    self.direction_reverse = !self.direction_reverse;
                                    //ask for colour
                                    self.select_color();
                                    return Ok(());
                                }
                                CardType::DRAW6 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 6;
                                    //ask for colour
                                    self.select_color();
                                    return Ok(());
                                }
                                CardType::DRAW10 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 10;
                                    //ask for colour
                                    self.select_color();
                                    return Ok(());
                                }
                                CardType::WILDROULETTE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::REVERSE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SKIP => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SKIPEVERYONE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DISCARDALL => {
                                    return Err(GameError::InvalidMove);
                                }
                            };
                        } else if current_card.card_type == CardType::DRAW6 {
                            match card.card_type {
                                CardType::Number(_) => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SEVENPASS => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::ZEROPASS => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW2 => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW4 => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW4REVERSE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW6 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 6;
                                    //ask for colour
                                    self.select_color();
                                    return Ok(());
                                }
                                CardType::DRAW10 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 10;
                                    //ask for colour
                                    self.select_color();
                                    return Ok(());
                                }
                                CardType::WILDROULETTE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::REVERSE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SKIP => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SKIPEVERYONE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DISCARDALL => {
                                    return Err(GameError::InvalidMove);
                                }
                            };
                        } else if current_card.card_type == CardType::DRAW10 {
                            match card.card_type {
                                CardType::Number(_) => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SEVENPASS => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::ZEROPASS => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW2 => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW4 => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW4REVERSE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW6 => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DRAW10 => {
                                    self.pick_draw_cards = self.pick_draw_cards + 10;
                                    //ask for colour
                                    self.select_color();
                                    return Ok(());
                                }
                                CardType::WILDROULETTE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::REVERSE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SKIP => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::SKIPEVERYONE => {
                                    return Err(GameError::InvalidMove);
                                }
                                CardType::DISCARDALL => {
                                    return Err(GameError::InvalidMove);
                                }
                            };
                        } else {
                            return Err(GameError::InvalidCardIndex);
                        }
                    } else if current_card.color == Color::BLACK {
                        //only select black card
                        match card.card_type {
                            CardType::Number(_) => {
                                if card.color!=self.card_to_select.unwrap(){
                                    return Err(GameError::InvalidMove);
                                }
                            }
                            CardType::SEVENPASS => {
                                if card.color!=self.card_to_select.unwrap() {
                                    return Err(GameError::InvalidMove);
                                }
                                self.sevenpass(card_index);
                            }
                            CardType::ZEROPASS => {
                                if card.color!=self.card_to_select.unwrap() {
                                    return Err(GameError::InvalidMove);
                                }
                                println!("ZERO PASS");
                                self.zeropass(card_index);
                            }
                            CardType::DRAW2 => {
                                if card.color!=self.card_to_select.unwrap() {
                                    return Err(GameError::InvalidMove);
                                }
                                self.pick_draw_cards= self.pick_draw_cards+2;
                            }
                            CardType::DRAW4 => {
                                if card.color!=self.card_to_select.unwrap() {
                                    return Err(GameError::InvalidMove);
                                }
                                self.pick_draw_cards = self.pick_draw_cards + 4;
                            }
                            CardType::DRAW4REVERSE => {
                                self.pick_draw_cards = self.pick_draw_cards + 4;
                                self.direction_reverse = !self.direction_reverse;
                                //ask for colour
                                self.select_color();
                            }
                            CardType::DRAW6 => {
                                self.pick_draw_cards = self.pick_draw_cards + 6;
                                //ask for colour
                                self.select_color();
                            }
                            CardType::DRAW10 => {
                                self.pick_draw_cards = self.pick_draw_cards + 10;
                                //ask for colour
                                self.select_color();
                            }
                            CardType::WILDROULETTE => {
                                self.select_color();
                                self.is_routlet_on_you=true;
                            }
                            CardType::REVERSE => {
                                if card.color!=self.card_to_select.unwrap() {
                                    return Err(GameError::InvalidMove);
                                }
                                self.direction_reverse= !self.direction_reverse;
                            }
                            CardType::SKIP => {
                                if card.color!=self.card_to_select.unwrap() {
                                    return Err(GameError::InvalidMove);
                                }
                                self.skip_card();
                            }
                            CardType::SKIPEVERYONE => {
                                if card.color!=self.card_to_select.unwrap() {
                                    return Err(GameError::InvalidMove);
                                }
                            }
                            CardType::DISCARDALL => {
                                if card.color!=self.card_to_select.unwrap() {
                                    return Err(GameError::InvalidMove);
                                }
                                self.discard_all_card(card);
                            }
                        }
                        return Ok(());
                    } else if current_card.color == card.color {
                        // select only color cards
                        match card.card_type {
                            CardType::Number(_) => {
                                return Ok(());
                            }
                            CardType::SEVENPASS => {
                                self.sevenpass(card_index);
                            }
                            CardType::ZEROPASS => {
                                println!("ZERO PASS");
                                self.zeropass(card_index);
                            }
                            CardType::DRAW2 => {
                                self.pick_draw_cards = self.pick_draw_cards + 2;
                            }
                            CardType::DRAW4 => {
                                self.pick_draw_cards = self.pick_draw_cards + 4;
                            }
                            CardType::DRAW4REVERSE => {}
                            CardType::DRAW6 => {}
                            CardType::DRAW10 => {}
                            CardType::WILDROULETTE => {}
                            CardType::REVERSE => {
                                self.direction_reverse = !self.direction_reverse;
                            }
                            CardType::SKIP => {
                                self.skip_card();
                            }
                            CardType::SKIPEVERYONE => {}
                            CardType::DISCARDALL => {
                                println!("DISCARD ALL PASS");
                                self.discard_all_card(&card);
                            }
                        };
                        return Ok(());
                    }else if card.color== Color::BLACK{
                        match card.card_type {
                            CardType::Number(_) => {}
                            CardType::SEVENPASS => {}
                            CardType::ZEROPASS => {}
                            CardType::DRAW2 => {}
                            CardType::DRAW4 => {}
                            CardType::DRAW4REVERSE => {
                                self.pick_draw_cards = self.pick_draw_cards + 4;
                                self.direction_reverse = !self.direction_reverse;
                                //ask for colour
                                self.select_color();
                            }
                            CardType::DRAW6 => {
                                self.pick_draw_cards = self.pick_draw_cards + 6;
                                //ask for colour
                                self.select_color();
                            }
                            CardType::DRAW10 => {
                                self.pick_draw_cards = self.pick_draw_cards + 10;
                                //ask for colour
                                self.select_color();
                            }
                            CardType::WILDROULETTE => {
                                self.select_color();
                                self.is_routlet_on_you=true;
                            }
                            CardType::REVERSE => {}
                            CardType::SKIP => {}
                            CardType::SKIPEVERYONE => {}
                            CardType::DISCARDALL => {}
                        }
                    }
                } else {
                   return Err(GameError::InvalidMove);
                }
                return Ok(());
    }
    fn skip_card(&mut self) {
        if self.players_length==1 {
            self.current_player=0;
        }
        if self.direction_reverse {
            self.current_player =(self.current_player + self.players_length - 2) % self.players_length;
        } else {
            self.current_player = (self.current_player + 2) % self.players_length;
        }
    }
    fn display_players(&self) {
        for (i, player) in self.players.iter().enumerate() {
            println!("{}) {}", i, player.name);
        }
    }
    fn check_if_player_valid(&mut self, index: usize,card_index:usize) -> Result<(), GameError> {
        let current = self.current_player as usize;
        let player= self.players.get(current).unwrap();
        let cards= self.player_deck.get_mut(&player.deck_index).unwrap();
        cards.remove(card_index);
        if index == current {
            return Err(GameError::YouCannotSelectYourself);
        }
        let (p1, p2) = if index < current {
            let (left, right) = self.players.split_at_mut(current);
            (left.get_mut(index), right.get_mut(0))
        } else {
            let (left, right) = self.players.split_at_mut(index);
            (right.get_mut(0), left.get_mut(current))
        };
        match (p1, p2) {
            (Some(p), Some(cp)) => {
                let temp = p.deck_index.clone();
                p.deck_index = cp.deck_index.clone();
                cp.deck_index = temp;
            }
            _ => return Err(GameError::InvalidPlayer),
        }

        Ok(())
    }
    fn sevenpass(&mut self,card_index:usize) {
        self.display_players();
        loop {
            println!("Enter Index of Player deck to choose :");
            let io = io::stdout().flush();
            match io {
                Ok(_) => {
                    let mut input = String::new();
                    let error = io::stdin().read_line(&mut input);
                    match error {
                        Ok(_) => match input.trim().parse::<usize>() {
                            Ok(number) => {
                                println!("You entered: {}", number);
                                let check = self.check_if_player_valid(number,card_index);
                                match check {
                                    Ok(_) => {
                                        break;
                                    }
                                    Err(_) => {
                                        println!("INVALID MOVE RETRY");
                                        continue;
                                    }
                                }
                            }
                            Err(_) => {
                                println!("Invalid input. Please enter a valid unsigned number.");
                                continue;
                            }
                        },
                        Err(_) => {
                            println!("ERROR TAKING INDEX DO AGAIN");
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("ERROR TAKING INDEX DO AGAIN");
                    continue;
                }
            }
        }
    }
    fn player_won(&mut self,player:usize,deck_index:&String){
    let player=self.players.remove(player);
    self.player_deck.remove(deck_index);
    self.leaderboard.push(PlayerRank { name: player.name, rank: self.top_players });
    self.top_players+=1;
    self.players_length-=1;
   }
    fn zeropass(&mut self,card_index:usize) {
        let player = self.current_player as usize;
        let player_length = self.players_length;
        if player >= player_length as usize || player_length <= 0 {
            return;
        }
        let deck_index = self.players[player].deck_index.clone();
        let cards= self.player_deck.get_mut(&deck_index).unwrap();
        cards.remove(card_index);
        let curr_player = self.players.get(player).unwrap();
        if cards.is_empty() {
            let fm= format!("PLAYER : {} WON",curr_player.name);
            println!("{}",fm.bright_magenta());
            self.player_won(player,&deck_index);
        }
        let mut deck_indices: Vec<_> =
            self.players.iter().map(|p| p.deck_index.clone()).collect();
            println!("BEFORE {:?}",deck_indices);
            deck_indices.rotate_right(1);
            println!("AFTER {:?}",deck_indices);
        for (player, new_index) in self.players.iter_mut().zip(deck_indices) {
            player.deck_index = new_index;
        }
    }
    fn discard_all_card(&mut self, current_card: &Card) {
        let player = &self.players[self.current_player as usize];
        println!("PLAYER {:?}",player);
        if let Some(cards) = self.player_deck.get_mut(&player.deck_index) {
            let mut i=0;
            while i< cards.len() {
                if cards[i].color==current_card.color {
                    let remove= cards.remove(i);
                    self.playing_stack.push(remove);
                }else {
                    i=i+1;
                }
            }
        }
    }
    fn select_color(&mut self){
        let get_color= self.get_color_to_select();
        self.card_to_select= get_color;
    }
    fn get_color_to_select(&self) -> Option<Color> {
        loop {
            print!("Enter a color (red, green, yellow, blue): ");
            let _ = io::stdout().flush();  // Ensures the prompt is printed immediately

            let mut input = String::new();
            let read = io::stdin().read_line(&mut input);

            // Handle the possible error from read_line
            match read {
                Ok(_) => {
                    match self.from_str(&input) {
                        Some(color) if color != Color::BLACK => return Some(color),
                        _ => {
                            println!("Invalid color or BLACK is not allowed, try again.");
                        }
                    }
                }
                Err(_) => {
                    println!("Error reading input. Please try again.");
                    continue; // Continue the loop to ask again
                }
            }
        }
    }
    fn from_str(&self,input: &str) -> Option<Color> {
        match input.trim().to_lowercase().as_str() {
            "red" => Some(Color::RED),
            "green" => Some(Color::GREEN),
            "yellow" => Some(Color::YELLOW),
            "blue" => Some(Color::BLUE),
            _ => None, 
        }
    }
    fn pick_up_one(&mut self ){
        let current= self.current_player as usize;
        let player= self.players.get(current);
        match player {
            Some(p)=>{
                let deck= self.player_deck.get_mut(&p.deck_index);
                match deck{
                    Some(dek)=>{
                        let card= self.deck.pop().unwrap();
                        dek.push(card);
                    },
                    None=>{
                    }
                }
            },None=>{}
        }
    }
    fn pick_draws(&mut self){
        self.maybe_refill_deck();
        if self.pick_draw_cards<=0 {
            return;            
        }
        let current= self.current_player as usize;
        let player= self.players.get(current);
        let cards= self.pick_draw_cards;
        match player {
            Some(p)=>{
                let deck= self.player_deck.get_mut(&p.deck_index);
                match deck{
                    Some(dek)=>{
                        for _ in 0..cards  {
                            let card= self.deck.pop().unwrap();
                            dek.push(card);
                        }
                        self.pick_draw_cards=0;
                    },
                    None=>{
                    }
                }
            },None=>{}
        }
    }
    fn maybe_refill_deck(&mut self) {
        if self.deck.len() < 30 && self.playing_stack.len() > 10 {
            let keep_top = self.playing_stack.pop().unwrap();
            let recycled: Vec<Card> = self.playing_stack
                .drain(..self.playing_stack.len() * 80 / 100)
                .collect();
            self.deck.extend(recycled);
            self.shuffle_cards();
            self.shuffle_cards();
            self.playing_stack.push(keep_top);
            println!("♻️ Deck was refilled from playing stack.");
        }
    }
}


