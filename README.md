# 📚 Tutorial: Building an NFT Card Game with Ink! Smart Contract

This tutorial will guide you through the process of building an NFT-based card game using Ink!, the smart contract language for the Polkadot ecosystem. You'll implement essential components of the game, including card creation, ownership transfer, and gameplay mechanics.

## 📝 Prerequisites

Before we start, ensure you have the following:

1. **🦀 Rust**: Install Rust and set up your environment.
   - Install Rust: [Rust Installation Guide](https://www.rust-lang.org/tools/install)
   - Add the wasm32-unknown-unknown target:
     ```bash
     rustup target add wasm32-unknown-unknown
     ```

2. **💻 Visual Studio Code (VS Code)**: Set up VS Code for Rust and Ink! development.
   - Download and install VS Code from the [official website](https://code.visualstudio.com/).
   - Install the following extensions in VS Code:
     - **Rust Extension**:
       - Go to the Extensions view (`Ctrl+Shift+X`).
       - Search for "Rust" and install the [Rust extension by rust-lang](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust).
     - **Ink! Extension**:
       - In the Extensions view, search for "Ink!" and install the [Ink! extension](https://marketplace.visualstudio.com/items?itemName=ParityTechnologies.ink-vscode).
     - **Rust Analyzer Extension (Recommended)**:
       - This extension provides enhanced Rust language support, including code completion and inline error checking.
       - Search for "Rust Analyzer" in the Extensions view and install the [Rust Analyzer extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer).

### 📦 Setup on Different Operating Systems

[OS-specific setup instructions remain the same as in the original tutorial]

## 🚀 Step 1: Set Up a New Ink! Project

Create a new Ink! project for your NFT card game contract.

```bash
cargo contract new nft_card_game
cd nft_card_game
```

This will create a basic Ink! project structure.

## 🛠️ Step 2: Update Cargo.toml

Open the `Cargo.toml` file in your project and update the dependencies:

```toml
[package]
name = "nft_card_game"
version = "0.1.0"
authors = ["[Your Name] <[your_email@example.com]>"]
edition = "2021"

[dependencies]
ink = { version = "4.3", default-features = false }

scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }
scale-info = { version = "2.9", default-features = false, features = ["derive"], optional = true }

[dev-dependencies]
ink_e2e = "4.3"

[lib]
path = "lib.rs"

[features]
default = ["std"]
std = [
    "ink/std",
    "scale/std",
    "scale-info/std",
]
ink-as-dependency = []
e2e-tests = []
```

## 🧑‍💻 Step 3: Implement NFT Card Game Logic

### Define the contract structure

```rust
#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;

#[ink::contract]
mod nft_card_game {
    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct NftCardGame {
        // We'll add fields here
    }

    impl NftCardGame {
        #[ink(constructor)]
        pub fn new() -> Self {
            // We'll implement this later
            Self {}
        }
    }
}
```

### Define the Card struct and Error enum

Add the following inside the `mod nft_card_game` block:

```rust
#[derive(scale::Decode, scale::Encode, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotOwner,
    TokenNotFound,
    NotApproved,
    TokenAlreadyExists,
}

#[derive(scale::Decode, scale::Encode, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Card {
    name: String,
    attack: u32,
    defense: u32,
}
```

### Implement the contract storage

Update the `NftCardGame` struct:

```rust
#[ink(storage)]
pub struct NftCardGame {
    owner: AccountId,
    cards: StorageHashMap<u32, Card>,
    card_owners: StorageHashMap<u32, AccountId>,
    next_token_id: u32,
}
```

### Implement the constructor

Replace the empty `new()` function with:

```rust
#[ink(constructor)]
pub fn new() -> Self {
    Self {
        owner: Self::env().caller(),
        cards: StorageHashMap::new(),
        card_owners: StorageHashMap::new(),
        next_token_id: 1,
    }
}
```

### Implement card creation

Add the following method to the `impl NftCardGame` block:

```rust
#[ink(message)]
pub fn create_card(&mut self, name: String, attack: u32, defense: u32) -> Result<u32, Error> {
    if self.env().caller() != self.owner {
        return Err(Error::NotOwner);
    }
    let token_id = self.next_token_id;
    self.next_token_id += 1;
    let card = Card { name, attack, defense };
    self.cards.insert(token_id, card);
    self.card_owners.insert(token_id, self.owner);
    Ok(token_id)
}
```

### Implement card retrieval

Add this method:

```rust
#[ink(message)]
pub fn get_card(&self, token_id: u32) -> Option<Card> {
    self.cards.get(&token_id).cloned()
}
```

### Implement card transfer

Add the transfer method:

```rust
#[ink(message)]
pub fn transfer(&mut self, to: AccountId, token_id: u32) -> Result<(), Error> {
    let owner = self.card_owners.get(&token_id).ok_or(Error::TokenNotFound)?;
    if *owner != self.env().caller() {
        return Err(Error::NotApproved);
    }
    self.card_owners.insert(token_id, to);
    Ok(())
}
```

### Implement game logic

Add the play_game method:

```rust
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
```

### Add tests

At the end of the file, add some basic tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

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

    #[ink::test]
    fn play_game_works() {
        let mut nft_game = NftCardGame::new();
        let token_id1 = nft_game.create_card("Dragon".to_string(), 100, 50).unwrap();
        let token_id2 = nft_game.create_card("Knight".to_string(), 80, 60).unwrap();
        let winner = nft_game.play_game(token_id1, token_id2).unwrap();
        assert_eq!(winner, token_id1);
    }
}
```

### Compile and test

1. Compile the contract:
   ```
   cargo +nightly contract build
   ```
2. Run the tests:
   ```
   cargo +nightly test
   ```

Congratulations! You've created a basic NFT card game smart contract using Ink!. This contract allows for card creation, ownership tracking, transfers, and simple game logic.
This code sets up the basic structure for creating, retrieving, transferring, and playing with NFT cards using only Ink!.

## 🔨 Step 4: Compile and Deploy

Compile your contract using the following command:

```bash
cargo +nightly contract build
```

This will generate a `.contract` file that you can deploy on a Substrate-based chain.

## 🌐 Step 5: Deploy and Interact

You can deploy and interact with your contract using tools like the [Contracts UI](https://contracts-ui.substrate.io/) or by writing a custom frontend application.

## 🎉 Conclusion

You've now set up a basic environment to build and interact with an NFT card game using only Ink!. Feel free to expand on this tutorial by implementing more complex game mechanics or adding additional features to your NFT card game. For more information, explore the [Ink! documentation](https://use.ink/) for more advanced use cases and examples.
