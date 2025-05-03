current card : if at start u get   
            - discard all
            random card will be choosen programmatically will benefit first user
            - WILDROULETTE
            random card will be choosen programmatically and first user has to pick up until that card comes
            - seven pass : first person can swap decks with anyone 
            - zero pass : deck will pass on to left direction card of player A B C D E to E A B C D
            - any draw : either pick draw cards or put value of high or equal
            - skip : direct turn of B
            - skipeveryone: no change still A turn



left:-
skip player who picked up the draw cards 
end game state or Player out
pick card if doesn't have in deck
pick draw cards to deck

Refactor error parsing


// BLACK

// if i play black

current_card.card_type == card.card_type // black card same type
current_card.color == card.color // isme bhi jaa sakta ha same black
current_card.color == Color::BLACK // isme bhi
card.color== Color::BLACK // isme bhi
self.pick_draw_cards > 0 
(current_card.card_type==CardType::WILDROULETTE && self.card_to_select!=None)
