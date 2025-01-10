use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct PeopleData {
    #[mutable]
    pub name: String,
    pub birth_date: Instant,
    pub father: u64,
    pub mother: u64,
    #[mutable]
    pub gender: String,
    #[mutable]
    pub occupation: String,
    #[mutable]
    pub partner: u64,
    #[mutable]
    pub mood_status: String,
    #[mutable]
    pub health_status: String,
    #[mutable]
    pub schooling: String,
    #[mutable]
    pub key_image_url: Url,
}

#[derive(ScryptoSbor, Clone)]
pub enum PriceRange {
    Cheap,
    Normal,
    Luxury,
}

#[derive(ScryptoSbor, NonFungibleData, Clone)]
pub struct ObjectData {
    pub name: String,
    pub price_range: PriceRange,
    #[mutable]
    pub mortgaged: bool,
    #[mutable]
    pub rent_allowed: bool,
    #[mutable]
    pub daily_rent_price: u32,
    #[mutable]
    pub rent_to: u64,
    pub key_image_url: Url,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct SoldObjectReceipt {
    pub name: String,
    pub price: u32,
    pub key_image_url: Url,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct SoldPeopleReceipt {
    pub price: u32,
    pub key_image_url: Url,
} 
