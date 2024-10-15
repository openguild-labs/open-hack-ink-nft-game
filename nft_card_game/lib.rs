// This attribute ensures the contract is compiled without the standard library when the "std" feature is not enabled
#![cfg_attr(not(feature = "std"), no_std, no_main)]

// This attribute marks this module as an ink! smart contract
#[ink::contract]
mod nft_card_game {
    use ink::storage::Mapping;

    // Custom error types for the contract
    #[derive(scale::Decode, scale::Encode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        TokenNotFound,
        NotApproved,
        TokenAlreadyExists,
    }

    // Struct representing a card in the game
    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Card {
        name: String,
        attack: u32,
        defense: u32,
    }

    // Main contract storage structure
    #[ink(storage)]
    pub struct NftCardGame {
        owner: AccountId,
        cards: Mapping<u32, Card>,
        card_owners: Mapping<u32, AccountId>,
        next_token_id: u32,
    }

    impl NftCardGame {
        // Constructor function to initialize the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                cards: Mapping::default(),
                card_owners: Mapping::default(),
                next_token_id: 1,
            }
        }

        // Function to create a new card (only callable by the owner)
        #[ink(message)]
        pub fn create_card(
            &mut self,
            name: String,
            attack: u32,
            defense: u32,
        ) -> Result<u32, Error> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            let token_id = self.next_token_id;
            self.next_token_id += 1;
            let card = Card {
                name,
                attack,
                defense,
            };
            self.cards.insert(token_id, &card);
            self.card_owners.insert(token_id, &self.owner);
            Ok(token_id)
        }

        // Function to retrieve a card's details by its token ID
        #[ink(message)]
        pub fn get_card(&self, token_id: u32) -> Option<Card> {
            self.cards.get(&token_id)
        }

        // Function to transfer ownership of a card
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, token_id: u32) -> Result<(), Error> {
            let owner = self
                .card_owners
                .get(&token_id)
                .ok_or(Error::TokenNotFound)?;
            if owner != self.env().caller() {
                return Err(Error::NotApproved);
            }
            self.card_owners.insert(token_id, &to);
            Ok(())
        }

        // Function to simulate a game between two cards
        #[ink(message)]
        pub fn play_game(&self, player1_card: u32, player2_card: u32) -> Option<u32> {
            let card1 = self.get_card(player1_card)?;
            let card2 = self.get_card(player2_card)?;
            let player1_power = card1.attack + card1.defense;
            let player2_power = card2.attack + card2.defense;
            if player1_power > player2_power {
                Some(player1_card)
            } else if player2_power > player1_power {
                Some(player2_card)
            } else {
                None // It's a tie
            }
        }
    }

    // Unit tests for the contract
    #[cfg(test)]
    mod tests {
        use super::*;

        // Test the card creation functionality
        #[ink::test]
        fn create_card_works() {
            let mut nft_game = NftCardGame::new();
            let token_id = nft_game.create_card("Dragon".to_string(), 100, 50).unwrap();
            assert_eq!(token_id, 1);
            let card = nft_game.get_card(token_id).unwrap();
            assert_eq!(card.name, "Dragon");
            assert_eq!(card.attack, 100);
            assert_eq!(card.defense, 50);
        }

        // Test the game playing functionality
        #[ink::test]
        fn play_game_works() {
            let mut nft_game = NftCardGame::new();
            let token_id1 = nft_game.create_card("Dragon".to_string(), 100, 50).unwrap();
            let token_id2 = nft_game.create_card("Knight".to_string(), 80, 60).unwrap();
            let winner = nft_game.play_game(token_id1, token_id2).unwrap();
            assert_eq!(winner, token_id1);
        }
    }
}
