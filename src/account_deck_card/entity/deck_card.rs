#[derive(Debug, Clone)]
pub struct DeckCard {
    card_id: i32,
    card_count: i32,
}

impl DeckCard {
    pub fn new(card_id: i32, card_count: i32) -> DeckCard{ DeckCard { card_id, card_count }
    }

    pub fn get_card(&self) -> i32 {
        self.card_id
    }

    pub fn get_card_count(&self) -> i32 {
        self.card_count
    }

    pub fn get_card_count_mut(&mut self) -> &mut i32 { &mut self.card_count }

    pub fn set_card_count(&mut self, count: i32) { self.card_count = count}
}