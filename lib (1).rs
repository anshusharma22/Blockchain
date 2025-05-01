#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Address, Symbol, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub image_url: String,
}

#[contracttype]
pub enum NFTStorageKey {
    Collection(Address),
}

#[contract]
pub struct NFTCollectionViewer;

#[contractimpl]
impl NFTCollectionViewer {
    // Store NFT under owner's collection
    pub fn add_nft(env: Env, owner: Address, name: String, description: String, image_url: String) {
        let key = NFTStorageKey::Collection(owner.clone());
        let mut collection: Vec<NFT> = env.storage().instance().get(&key).unwrap_or(Vec::new(&env));
        
        let nft = NFT {
            owner,
            name,
            description,
            image_url,
        };
        collection.push_back(nft);
        env.storage().instance().set(&key, &collection);
    }

    // Retrieve NFTs by owner address
    pub fn get_collection(env: Env, owner: Address) -> Vec<NFT> {
        let key = NFTStorageKey::Collection(owner);
        env.storage().instance().get(&key).unwrap_or(Vec::new(&env))
    }
}
