use scrypto::prelude::*;
use std::ops::Deref;
use crate::common::*;

#[derive(ScryptoSbor)]
struct ObjectTypeData {
    can_be_bought: bool,
    can_be_mortgaged: bool,
    can_be_rent: bool,
    price: u32,
    key_image_url: Url,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct NewPeopleEvent {
    people_id: u64,
    birth_date: i64,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct NameEvent {
    people_id: u64,
    name: String,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct NewObjectEvent {
    name: String,
    ids: Vec<u64>,
    mortgaged: bool,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct BankDepositEvent {
    people_id: u64,
    amount: u32,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct AllowRentEvent {
    object_id: u64,
    allow: bool,
    daily_price: Option<u32>,
    account: Option<u64>,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct RentEvent {
    object_id: u64,
    people_id: u64,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct TerminateRentEvent {
    object_id: u64,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct SoldObjectEvent {
    object_id: u64,
    price: u32,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct BoughtObjectEvent {
    object_id: u64,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct ChoiceEvent {
    choice: String,
    people_id: u64,
    number: u64,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct BankWithdrawEvent {
    amount: u32,
    people_id: u64,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct SoldPeopleEvent {
    people_id: u64,
    price: u32, 
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct BoughtPeopleEvent {
    people_id: u64,
}

#[blueprint]
#[events(
    NewPeopleEvent,
    NameEvent,
    NewObjectEvent,
    BankDepositEvent,
    AllowRentEvent,
    RentEvent,
    TerminateRentEvent,
    SoldObjectEvent,
    BoughtObjectEvent,
    ChoiceEvent,
    BankWithdrawEvent,
    SoldPeopleEvent,
    BoughtPeopleEvent,
)]
#[types(
    String,
    PeopleData,
    ObjectData,
    ObjectTypeData,
    u64,
    SoldObjectReceipt,
    SoldPeopleReceipt,
    u32,
)]
mod radix_life {

    enable_method_auth! {
        roles {
            updater => updatable_by: [OWNER];
        },
        methods {
            add_object_type => restrict_to: [OWNER];
            withdraw_xrd => restrict_to: [OWNER];
            update_coin_xrd_price => restrict_to: [OWNER];
            add_choice => restrict_to: [OWNER];
            update_object_type => restrict_to: [OWNER];

            new_egg => restrict_to: [updater];
            new_object => restrict_to: [updater];
            update_people_data => restrict_to: [updater];
            update_object_data => restrict_to: [updater];

            buy_egg => PUBLIC;
            buy_objects => PUBLIC;
            buy_coins => PUBLIC;
            give_name => PUBLIC;
            deposit_to_bank_account => PUBLIC;
            withdraw_from_bank_account => PUBLIC;
            mortgage => PUBLIC;
            allow_rent => PUBLIC;
            rent => PUBLIC;
            terminate_rent => PUBLIC;
            sell_object => PUBLIC;
            buy_used_object => PUBLIC;
            close_object_sale => PUBLIC;
            make_choice => PUBLIC;
            sell_people => PUBLIC;
            buy_people => PUBLIC;
            close_people_sale => PUBLIC;
        }
    }

    struct RadixLife {
        eggs_on_sale: u32,
        egg_xrd_price: Decimal,
        coin_xrd_price: Decimal,
        last_people_id: u64,
        people_resource_manager: NonFungibleResourceManager,
        last_object_id: u64,
        coin_resource_manager: FungibleResourceManager,
        hatch_time: i64,
        egg_image_url: Url,
        account_locker: Global<AccountLocker>,
        xrd_vault: Vault,
        sold_objects_resource_manager: NonFungibleResourceManager,
        choices: KeyValueStore<String, u32>,
        people_vault: NonFungibleVault,
        people_prices: KeyValueStore<u64, u32>,
        sold_people_resource_manager: NonFungibleResourceManager,
        object_resource_manager: NonFungibleResourceManager,
        object_types: KeyValueStore<String, ObjectTypeData>,
        used_objects_vault: NonFungibleVault,
        last_receipt_id: u64,
    }

    impl RadixLife {

        pub fn new(
            owner_badge_address: ResourceAddress,
            updater_badge_address: ResourceAddress,
            eggs_on_sale: u32,
            egg_xrd_price: Decimal,
            coin_xrd_price: Decimal,
            hatch_time: i64,
            egg_image_url: String,
        ) -> Global<RadixLife> {
            assert!(
                eggs_on_sale > 2,
                "Egg on sale must be bigger than two",
            );
            assert!(
                egg_xrd_price > Decimal::ZERO,
                "Egg price must be bigger than zero",
            );
            assert!(
                coin_xrd_price > Decimal::ZERO,
                "Coin price must be bigger than zero",
            );
            assert!(
                hatch_time >= 0,
                "Hatch time can't be negative",
            );

            let (address_reservation, component_address) = Runtime::allocate_component_address(RadixLife::blueprint_id());

            let component_or_updater_rule = rule!(require(CompositeRequirement::AnyOf(vec![
                    global_caller(component_address).into(),
                    require(updater_badge_address),
            ])));

            let people_resource_manager = ResourceBuilder::new_integer_non_fungible_with_registered_type::<PeopleData>(
                OwnerRole::Updatable(rule!(require(owner_badge_address)))
            )
            .metadata(metadata!(
                roles {
                    metadata_setter => rule!(require(owner_badge_address));
                    metadata_setter_updater => rule!(require(owner_badge_address));
                    metadata_locker => rule!(require(owner_badge_address));
                    metadata_locker_updater => rule!(require(owner_badge_address));
                },
                init {
                    "name" => "RadixLife people", updatable;
                }
            ))
            .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(require(owner_badge_address));
            ))
            .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                non_fungible_data_updater => rule!(require(global_caller(component_address)));
                non_fungible_data_updater_updater => rule!(require(owner_badge_address));
            ))
            .burn_roles(burn_roles!(
                burner => rule!(deny_all);
                burner_updater => rule!(require(owner_badge_address));
            ))
            .create_with_no_initial_supply();

            let coin_resource_manager = ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(require(owner_badge_address))))
            .divisibility(DIVISIBILITY_MAXIMUM)
            .metadata(metadata!(
                roles {
                    metadata_setter => rule!(require(owner_badge_address));
                    metadata_setter_updater => rule!(require(owner_badge_address));
                    metadata_locker => rule!(require(owner_badge_address));
                    metadata_locker_updater => rule!(require(owner_badge_address));
                },
                init {
                    "symbol" => "RLC", updatable;
                    "name" => "RadixLife coin", updatable;
                }
            ))
            .mint_roles(mint_roles!(
                minter => component_or_updater_rule.clone();
                minter_updater => rule!(require(owner_badge_address));
            ))
            .burn_roles(burn_roles!(
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(require(owner_badge_address));
            ))
            .create_with_no_initial_supply();

            let account_locker = Blueprint::<AccountLocker>::instantiate(
                OwnerRole::Updatable(rule!(require(owner_badge_address))),  // owner_role
                component_or_updater_rule,                                  // storer_role
                rule!(require(owner_badge_address)),                        // storer_updater_role
                rule!(deny_all),                                            // recoverer_role
                rule!(require(owner_badge_address)),                        // recoverer_updater_role
                None
            );

            let sold_objects_resource_manager = ResourceBuilder::new_integer_non_fungible_with_registered_type::<SoldObjectReceipt>(
                OwnerRole::Updatable(rule!(require(owner_badge_address)))
            )
            .metadata(metadata!(
                roles {
                    metadata_setter => rule!(require(owner_badge_address));
                    metadata_setter_updater => rule!(require(owner_badge_address));
                    metadata_locker => rule!(require(owner_badge_address));
                    metadata_locker_updater => rule!(require(owner_badge_address));
                },
                init {
                    "name" => "RadixLife object on sale", updatable;
                }
            ))
            .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(require(owner_badge_address));
            ))
            .burn_roles(burn_roles!(
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(require(owner_badge_address));
            ))
            .create_with_no_initial_supply();

            let sold_people_resource_manager = ResourceBuilder::new_integer_non_fungible_with_registered_type::<SoldPeopleReceipt>(
                OwnerRole::Updatable(rule!(require(owner_badge_address)))
            )
            .metadata(metadata!(
                roles {
                    metadata_setter => rule!(require(owner_badge_address));
                    metadata_setter_updater => rule!(require(owner_badge_address));
                    metadata_locker => rule!(require(owner_badge_address));
                    metadata_locker_updater => rule!(require(owner_badge_address));
                },
                init {
                    "name" => "RadixLife people on sale", updatable;
                }
            ))
            .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(require(owner_badge_address));
            ))
            .burn_roles(burn_roles!(
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(require(owner_badge_address));
            ))
            .create_with_no_initial_supply();

            let object_resource_manager = ResourceBuilder::new_integer_non_fungible_with_registered_type::<ObjectData>(
                OwnerRole::Updatable(rule!(require(owner_badge_address)))
            )
            .metadata(metadata!(
                roles {
                    metadata_setter => rule!(require(owner_badge_address));
                    metadata_setter_updater => rule!(require(owner_badge_address));
                    metadata_locker => rule!(require(owner_badge_address));
                    metadata_locker_updater => rule!(require(owner_badge_address));
                },
                init {
                    "name" => "RadixLife object", updatable;
                }
            ))
            .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(require(owner_badge_address));
            ))
            .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                non_fungible_data_updater => rule!(require(global_caller(component_address)));
                non_fungible_data_updater_updater => rule!(require(owner_badge_address));
            ))
            .burn_roles(burn_roles!(
                burner => rule!(require(updater_badge_address));
                burner_updater => rule!(require(owner_badge_address));
            ))
            .recall_roles(recall_roles!(
                recaller => rule!(require(updater_badge_address));
                recaller_updater => rule!(require(owner_badge_address));
            ))
            .create_with_no_initial_supply();

            Self {
                eggs_on_sale: eggs_on_sale,
                egg_xrd_price: egg_xrd_price,
                coin_xrd_price: coin_xrd_price,
                last_people_id: 0,
                people_resource_manager: people_resource_manager,
                last_object_id: 0,
                coin_resource_manager: coin_resource_manager,
                hatch_time: hatch_time,
                egg_image_url: UncheckedUrl(egg_image_url),
                account_locker: account_locker,
                xrd_vault: Vault::new(XRD),
                sold_objects_resource_manager: sold_objects_resource_manager,
                choices: KeyValueStore::new_with_registered_type(),
                people_vault: NonFungibleVault::new(people_resource_manager.address()),
                people_prices: KeyValueStore::new_with_registered_type(),
                sold_people_resource_manager: sold_people_resource_manager,
                object_resource_manager: object_resource_manager,
                object_types: KeyValueStore::new_with_registered_type(),
                used_objects_vault: NonFungibleVault::new(object_resource_manager.address()),
                last_receipt_id: 0,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Updatable(rule!(require(owner_badge_address))))
            .roles(roles!(
                updater => rule!(require(updater_badge_address));
            ))
            .with_address(address_reservation)
            .globalize()
        }

        pub fn add_object_type(
            &mut self,
            name: String,
            price: u32,
            key_image_url: String,
            can_be_bought: bool,
            can_be_mortgaged: bool,
            can_be_rent: bool,
        ) {
            self.object_types.insert(
                name,
                ObjectTypeData {
                    can_be_bought: can_be_bought,
                    can_be_mortgaged: can_be_mortgaged,
                    can_be_rent: can_be_rent,
                    price: price,
                    key_image_url: UncheckedUrl(key_image_url),
                }
            );
        }

        pub fn withdraw_xrd(&mut self) -> Bucket {
            self.xrd_vault.take_all()
        }

        pub fn update_coin_xrd_price(
            &mut self,
            coin_xrd_price: Decimal,
        ) {
            assert!(
                coin_xrd_price > Decimal::ZERO,
                "Coin price must be bigger than zero",
            );
            self.coin_xrd_price = coin_xrd_price;
        }

        fn mint_egg(
            &mut self,
            father: u64,
            mother: u64,
        ) -> NonFungibleBucket {
            self.last_people_id += 1;

            let birth_date = Clock::current_time_rounded_to_seconds().add_seconds(self.hatch_time).unwrap();

            Runtime::emit_event(
                NewPeopleEvent {
                    people_id: self.last_people_id,
                    birth_date: birth_date.seconds_since_unix_epoch,
                }
            );

            self.people_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(self.last_people_id.into()),
                PeopleData {
                    name: "".to_string(),
                    birth_date: birth_date,
                    father: father,
                    mother: mother,
                    gender: "unknown".to_string(),
                    occupation: "unemployed".to_string(),
                    partner: 0,
                    mood_status: "normal".to_string(),
                    health_status: "healthy".to_string(),
                    schooling: "none".to_string(),
                    key_image_url: self.egg_image_url.clone(),
                }
            )
        }

        pub fn new_egg(
            &mut self,
            father: u64,
            mother: u64,
            account: Global<Account>,
        ) {
            let egg_bucket = self.mint_egg(father, mother);

            self.account_locker.store(
                account,
                egg_bucket.into(),
                true,
            );
        }

        pub fn new_object(
            &mut self,
            name: String,
            mortgaged: bool,
            account: Global<Account>,
        ) {
            let object_type = self.object_types.get(&name).expect("Object not found");

            self.last_object_id += 1;

            let object_bucket = self.object_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(self.last_object_id.into()),
                ObjectData {
                    name: name,
                    mortgaged: mortgaged,
                    rent_allowed: false,
                    daily_rent_price: 0,
                    rent_to: 0,
                    key_image_url: object_type.key_image_url.clone(),
                }
            );

            self.account_locker.store(
                account,
                object_bucket.into(),
                true,
            );
        }

        pub fn buy_egg(
            &mut self,
            mut xrd_bucket: Bucket,
        ) -> (
            NonFungibleBucket,
            Bucket,
        ) {
            assert!(
                xrd_bucket.resource_address() == XRD,
                "Wrong coin",
            );
            assert!(
                xrd_bucket.amount() >= self.egg_xrd_price,
                "Insufficient amount",
            );
            self.xrd_vault.put(xrd_bucket.take(self.egg_xrd_price));

            assert!(
                self.eggs_on_sale > 0,
                "Sale ended",
            );
            self.eggs_on_sale -= 1;

            (
                self.mint_egg(0, 0),
                xrd_bucket,
            )
        }

        pub fn buy_objects(
            &mut self,
            mut coin_bucket: Bucket,
            name: String,
            amount: u8,
            mortgaged: bool,
        ) -> (
            NonFungibleBucket,
            Bucket,
        ) {
            assert!(
                coin_bucket.resource_address() == self.coin_resource_manager.address(),
                "Wrong coin",
            );
            assert!(
                amount > 0,
                "Can't buy zero objects",
            );

            let object_type = self.object_types.get(&name).expect("Object not found");

            assert!(
                object_type.can_be_bought,
                "This object can't be bought"
            );

            let total_price = match mortgaged {
                false => object_type.price * amount as u32,
                true => {
                    assert!(
                        object_type.can_be_mortgaged,
                        "This object can't be mortgaged",
                    );
                    object_type.price * amount as u32 / 2
                },
            };
            coin_bucket.take(Decimal::from(total_price)).burn();

            let mut ids: Vec<u64> = Vec::new();

            let first = self.last_object_id + 1;
            self.last_object_id += amount as u64;
            let mut objects_bucket = NonFungibleBucket::new(self.object_resource_manager.address());
            for id in first..=self.last_object_id {
                objects_bucket.put(
                    self.object_resource_manager.mint_non_fungible(
                        &NonFungibleLocalId::integer(id.into()),
                        ObjectData {
                            name: name.clone(),
                            mortgaged: mortgaged,
                            rent_allowed: false,
                            daily_rent_price: 0,
                            rent_to: 0,
                            key_image_url: object_type.key_image_url.clone(),
                        }
                    )
                );

                ids.push(id);
            }

            Runtime::emit_event(
                NewObjectEvent {
                    name: name,
                    ids: ids,
                    mortgaged: mortgaged,
                }
            );

            (
                objects_bucket,
                coin_bucket,
            )
        }

        pub fn buy_coins(
            &mut self,
            xrd_bucket: Bucket,
        ) -> FungibleBucket {
            assert!(
                xrd_bucket.resource_address() == XRD,
                "Wrong coin",
            );

            let coin_amount = xrd_bucket.amount() / self.coin_xrd_price;

            self.xrd_vault.put(xrd_bucket);

            self.coin_resource_manager.mint(coin_amount)
        }

        pub fn give_name(
            &self,
            people_proof: Proof,
            mut name: String,
        ) {
            let non_fungible = people_proof.check_with_message(
                self.people_resource_manager.address(),
                "Wrong NFT",
            )
            .as_non_fungible()
            .non_fungible::<PeopleData>();

            assert!(
                non_fungible.data().name.as_str() == "",
                "Name already assigned",
            );

            name = name.trim().to_string();
            assert!(
                name.len() > 0 && name.len() < 256,
                "Invalid name size",
            );
            assert!(
                name.chars().all(|c| c.is_ascii_alphanumeric() || c == ' '),
                "Illegal character in name"
            );
            
            self.people_resource_manager.update_non_fungible_data(
                &non_fungible.local_id(),
                "name",
                name.clone(),
            );

            let local_id = match &non_fungible.local_id() {
                NonFungibleLocalId::Integer(local_id) => local_id.value(),
                _ => Runtime::panic("Should not happen".to_string()),
            };

            Runtime::emit_event(
                NameEvent {
                    people_id: local_id,
                    name: name,
                }
            );
        }

        pub fn deposit_to_bank_account(
            &self,
            people_id: u64,
            coin_bucket: Bucket,
        ) {
            assert!(
                coin_bucket.resource_address() == self.coin_resource_manager.address(),
                "Wrong coin",
            );

            Runtime::emit_event(
                BankDepositEvent {
                    people_id: people_id,
                    amount: u32::try_from(coin_bucket.amount().checked_floor().unwrap()).unwrap(),
                }
            );

            coin_bucket.burn();
        }

        pub fn withdraw_from_bank_account(
            &self,
            people_proof: Proof,
            amount: u32,
        ) {
            let non_fungible = people_proof.check_with_message(
                self.people_resource_manager.address(),
                "Wrong NFT",
            )
            .as_non_fungible()
            .non_fungible::<PeopleData>();
            let people_id = match non_fungible.local_id() {
                NonFungibleLocalId::Integer(id) => id.value(),
                _ => Runtime::panic("Should not happen".to_string()),
            };

            Runtime::emit_event(
                BankWithdrawEvent {
                    amount: amount,
                    people_id: people_id,
                }
            );
        }

        pub fn update_people_data(
            &self,
            people_id: u64,
            non_fungible_data: Option<HashMap<String, String>>,
            partner: Option<u64>,
            key_image_url: Option<String>,
        ) {
            let id = NonFungibleLocalId::integer(people_id.into());

            match non_fungible_data {
                None => {},
                Some(non_fungible_data) => {
                    for (name, value) in non_fungible_data.iter() {
                        self.people_resource_manager.update_non_fungible_data(
                            &id,
                            name,
                            value
                        );
                    }
                }
            }

            match partner {
                None => {},
                Some(partner) => {
                    self.people_resource_manager.update_non_fungible_data(
                        &id,
                        "partner",
                        partner
                    );
                },
            }

            match key_image_url {
                None => {},
                Some(key_image_url) => {
                    self.people_resource_manager.update_non_fungible_data(
                        &id,
                        "key_image_url",
                        UncheckedUrl(key_image_url)
                    );
                },
            }
        }

        pub fn update_object_data(
            &self,
            object_id: u64,
            mortgaged: Option<bool>,
            rent_to: Option<u64>,
        ) {
            match mortgaged {
                None => {},
                Some(mortgaged) => {
                    self.object_resource_manager.update_non_fungible_data(
                        &NonFungibleLocalId::integer(object_id.into()),
                        "mortgaged",
                        mortgaged
                    );
                },
            }

            match rent_to {
                None => {},
                Some(rent_to) => {
                    self.object_resource_manager.update_non_fungible_data(
                        &NonFungibleLocalId::integer(object_id.into()),
                        "rent_to",
                        rent_to
                    );
                },
            }
        }

        pub fn mortgage(
            &self,
            object_proof: Proof,
            deposit_account: Option<u64>,
        ) -> Option<FungibleBucket> {
            let non_fungible = object_proof.check_with_message(
                self.object_resource_manager.address(),
                "Wrong NFT",
            )
            .as_non_fungible()
            .non_fungible::<ObjectData>();
            let non_fungible_data = non_fungible.data();

            let object_type = self.object_types.get(&non_fungible_data.name).expect("Object not found");

            assert!(
                object_type.can_be_mortgaged,
                "This object can't be mortgaged",
            );
            assert!(
                !non_fungible_data.mortgaged,
                "Object already mortgaged",
            );

            self.object_resource_manager.update_non_fungible_data(
                non_fungible.local_id(),
                "mortgaged",
                true
            );

            let amount = object_type.price / 2;

            match deposit_account {
                None => Some(self.coin_resource_manager.mint(amount)),
                Some(people_id) => {
                    Runtime::emit_event(
                        BankDepositEvent {
                            people_id: people_id,
                            amount: amount,
                        }
                    );
                    None
                },
            }
        }

        fn get_u64_id(
            local_id: &NonFungibleLocalId,
        ) -> u64 {
            match local_id {
                NonFungibleLocalId::Integer(id) => id.value(),
                _ => Runtime::panic("Should not happen".to_string()),
            }
        }

        pub fn allow_rent(
            &self,
            object_proof: Proof,
            allow: bool,
            daily_price: Option<u32>,
            account: Option<u64>,
        ) {
            let non_fungible = object_proof.check_with_message(
                self.object_resource_manager.address(),
                "Wrong NFT",
            )
            .as_non_fungible()
            .non_fungible::<ObjectData>();
            let non_fungible_data = non_fungible.data();

            let object_type = self.object_types.get(&non_fungible_data.name).expect("Object not found");

            assert!(
                object_type.can_be_rent,
                "This object can't be rent",
            );

            assert!(
                non_fungible_data.rent_to == 0 || daily_price.is_none(),
                "Can't update price on already rent objects",
            );

            self.object_resource_manager.update_non_fungible_data(
                non_fungible.local_id(),
                "rent_allowed",
                allow
            );
            self.object_resource_manager.update_non_fungible_data(
                non_fungible.local_id(),
                "daily_rent_price",
                daily_price
            );

            Runtime::emit_event(
                AllowRentEvent {
                    object_id: RadixLife::get_u64_id(non_fungible.local_id()),
                    allow: allow,
                    daily_price: daily_price,
                    account: account,
                }
            );
        }

        pub fn rent(
            &self,
            people_proof: Proof,
            name: String,
            object_id: u64,
        ) {
            let non_fungible = people_proof.check_with_message(
                self.people_resource_manager.address(),
                "Wrong NFT",
            )
            .as_non_fungible()
            .non_fungible::<PeopleData>();
            let people_id = match non_fungible.local_id() {
                NonFungibleLocalId::Integer(id) => id.value(),
                _ => Runtime::panic("Should not happen".to_string()),
            };

            let object_type = self.object_types.get(&name).expect("Object not found");
            assert!(
                object_type.can_be_rent,
                "This object can't be rent",
            );

            let nf_object_id = NonFungibleLocalId::Integer(object_id.into());

            // If is possible to create offchain objects, without minting and NFT, and rent them
            if self.object_resource_manager.non_fungible_exists(&nf_object_id) {
                let non_fungible_data = self.object_resource_manager.get_non_fungible_data::<ObjectData>(&nf_object_id);
                assert!(
                    non_fungible_data.rent_allowed,
                    "Object not for rent",
                );
                assert!(
                    non_fungible_data.rent_to == 0,
                    "Object already rent",
                );
                assert!(
                    name == non_fungible_data.name,
                    "Wrong name"
                );

                self.object_resource_manager.update_non_fungible_data(
                    &nf_object_id,
                    "rent_to",
                    people_id,
                );
            }

            Runtime::emit_event(
                RentEvent {
                    object_id: object_id,
                    people_id: people_id,
                }
            );
        }

        pub fn terminate_rent(
            &self,
            people_proof: Proof,
            object_id: u64,
        ) {
            let non_fungible = people_proof.check_with_message(
                self.people_resource_manager.address(),
                "Wrong NFT",
            )
            .as_non_fungible()
            .non_fungible::<PeopleData>();
            let people_id = match non_fungible.local_id() {
                NonFungibleLocalId::Integer(id) => id.value(),
                _ => Runtime::panic("Should not happen".to_string()),
            };

            let nf_object_id = NonFungibleLocalId::Integer(object_id.into());

            // If is possible to create offchain objects, without minting and NFT, and rent them
            if self.object_resource_manager.non_fungible_exists(&nf_object_id) {
                let non_fungible_data = self.object_resource_manager.get_non_fungible_data::<ObjectData>(&nf_object_id);
                assert!(
                    non_fungible_data.rent_to == people_id,
                    "Object not rent to you",
                );

                self.object_resource_manager.update_non_fungible_data(
                    &nf_object_id,
                    "rent_to",
                    0,
                );
            }

            Runtime::emit_event(
                TerminateRentEvent {
                    object_id: object_id,
                }
            );
        }

        pub fn sell_object(
            &mut self,
            object_bucket: NonFungibleBucket,
            price: u32,
        ) -> NonFungibleBucket {
            let non_fungible = object_bucket.non_fungible::<ObjectData>();
            let object_id = match non_fungible.local_id() {
                NonFungibleLocalId::Integer(id) => id.value(),
                _ => Runtime::panic("Should not happen".to_string()),
            };

            let non_fungible_data = non_fungible.data();
            assert!(
                self.object_resource_manager.address() == object_bucket.resource_address(),
                "Wrong NFT",
            );
            assert!(
                !non_fungible_data.rent_allowed && non_fungible_data.rent_to == 0,
                "Can't sell rented object",
            );

            self.used_objects_vault.put(object_bucket);

            Runtime::emit_event(
                SoldObjectEvent {
                    object_id: object_id,
                    price: price,
                }
            );
          
            self.last_receipt_id += 1;

            self.sold_objects_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(self.last_receipt_id.into()),
                SoldObjectReceipt {
                    object_id: object_id,
                    price: price,
                    key_image_url: non_fungible_data.key_image_url,
                }
            )
        }

        pub fn buy_used_object(
            &mut self,
            object_id: u64,
            mut coin_bucket: Bucket,
        ) -> (
            NonFungibleBucket,
            Bucket,
        ) {
            assert!(
                coin_bucket.resource_address() == self.coin_resource_manager.address(),
                "Wrong coin",
            );

            let nf_object_id = NonFungibleLocalId::integer(object_id);
            let non_fungible_data = self.sold_objects_resource_manager.get_non_fungible_data::<SoldObjectReceipt>(
                &nf_object_id
            );

            coin_bucket.take(Decimal::from(non_fungible_data.price)).burn();

            Runtime::emit_event(
                BoughtObjectEvent {
                    object_id: object_id,
                }
            );

            (
                self.used_objects_vault.take_non_fungible(&nf_object_id),
                coin_bucket,
            )
        }

        pub fn close_object_sale(
            &mut self,
            sold_object_bucket: Bucket,
        ) -> Bucket {
            assert!(
                sold_object_bucket.resource_address() == self.sold_objects_resource_manager.address(),
                "Wrong NFT",
            );
            let non_fungible = sold_object_bucket.as_non_fungible().non_fungible::<SoldObjectReceipt>();
            let non_fungible_data = non_fungible.data();

            let nf_object_id = NonFungibleLocalId::integer(non_fungible_data.object_id);

            sold_object_bucket.burn();

            match self.used_objects_vault.contains_non_fungible(&nf_object_id) {
                false => self.coin_resource_manager.mint(non_fungible_data.price).into(),
                true => self.used_objects_vault.take_non_fungible(&nf_object_id).into(),
            }
        }

        pub fn add_choice(
            &mut self,
            choice: String,
            price: Option<u32>,
        ) {
            match price {
                None => {
                    self.choices.remove(&choice);
                },
                Some(price) => self.choices.insert(choice, price),
            }
        }

        pub fn update_object_type(
            &mut self,
            name: String,
            price: u32,
            can_be_bought: bool,
            can_be_mortgaged: bool,
            can_be_rent: bool,
        ) {
            let mut object_type = self.object_types.get_mut(&name).expect("Object not found");

            object_type.price = price;
            object_type.can_be_bought = can_be_bought;
            object_type.can_be_mortgaged = can_be_mortgaged;
            object_type.can_be_rent = can_be_rent;
        }

        pub fn make_choice(
            &self,
            people_proof: Proof,
            choice: String,
            coin_bucket: Option<Bucket>,
            number: u64,
        ) {
            let non_fungible = people_proof.check_with_message(
                self.people_resource_manager.address(),
                "Wrong NFT",
            )
            .as_non_fungible()
            .non_fungible::<PeopleData>();
            let people_id = match non_fungible.local_id() {
                NonFungibleLocalId::Integer(id) => id.value(),
                _ => Runtime::panic("Should not happen".to_string()),
            };

            let price_ref = self.choices.get(&choice).expect("Choice not found");
            let price = price_ref.deref();

            Runtime::emit_event(
                ChoiceEvent {
                    choice: choice,
                    people_id: people_id,
                    number: number,
                }
            );

            if *price > 0 {
                let coin_bucket = coin_bucket.expect("Missing payment");

                assert!(
                    coin_bucket.resource_address() == self.coin_resource_manager.address(),
                    "Wrong coin"
                );

                assert!(
                    coin_bucket.amount() >= Decimal::from(*price),
                    "Not enough coins",
                );

                coin_bucket.burn();
            }
        }

        pub fn sell_people(
            &mut self,
            people_bucket: NonFungibleBucket,
            price: u32,
        ) -> NonFungibleBucket {
            let non_fungible = people_bucket.non_fungible::<PeopleData>();
            let people_id = match non_fungible.local_id() {
                NonFungibleLocalId::Integer(id) => id.value(),
                _ => Runtime::panic("Should not happen".to_string()),
            };

            assert!(
                self.people_vault.resource_address() == people_bucket.resource_address(),
                "Wrong NFT",
            );
            self.people_vault.put(people_bucket);

            Runtime::emit_event(
                SoldPeopleEvent { 
                    people_id: people_id,
                    price: price,
                }
            );
        
            self.last_receipt_id += 1;

            self.sold_people_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(self.last_receipt_id.into()),
                SoldPeopleReceipt {
                    people_id: people_id,
                    price: price,
                    key_image_url: non_fungible.data().key_image_url,
                }
            )
        }

        pub fn buy_people(
            &mut self,
            people_id: u64,
            mut coin_bucket: Bucket,
        ) -> (
            NonFungibleBucket,
            Bucket,
        ) {
            assert!(
                coin_bucket.resource_address() == self.coin_resource_manager.address(),
                "Wrong coin",
            );

            let nf_people_id = NonFungibleLocalId::integer(people_id);
            let non_fungible_data = self.sold_people_resource_manager.get_non_fungible_data::<SoldPeopleReceipt>(
                &nf_people_id
            );

            coin_bucket.take(Decimal::from(non_fungible_data.price)).burn();

            Runtime::emit_event(
                BoughtPeopleEvent {
                    people_id: people_id,
                }
            );

            (
                self.people_vault.take_non_fungible(&nf_people_id),
                coin_bucket,
            )
        }

        pub fn close_people_sale(
            &mut self,
            sold_people_bucket: Bucket,
        ) -> Bucket {
            assert!(
                sold_people_bucket.resource_address() == self.sold_people_resource_manager.address(),
                "Wrong NFT",
            );
            let non_fungible = sold_people_bucket.as_non_fungible().non_fungible::<SoldPeopleReceipt>();
            let non_fungible_data = non_fungible.data();

            let nf_people_id = NonFungibleLocalId::integer(non_fungible_data.people_id);

            sold_people_bucket.burn();

            match self.people_vault.contains_non_fungible(&nf_people_id) {
                false => self.coin_resource_manager.mint(non_fungible_data.price).into(),
                true => self.people_vault.take_non_fungible(&nf_people_id).into(),
            }
        }
    }
}
