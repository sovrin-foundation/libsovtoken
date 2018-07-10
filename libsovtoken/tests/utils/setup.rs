use std::collections::HashMap;
use std::iter::FromIterator;

use indy;
use sovtoken;
use utils::did;
use utils::mint;
use utils::payment::address as gen_address;
use utils::pool;
use utils::wallet::Wallet;

const PROTOCOL_VERSION: usize = 2;

pub struct SetupConfig
{
    pub num_trustees: u8,
    pub num_users: u8,
    pub num_addresses: usize,
    pub mint_tokens: Option<Vec<u64>>
}

pub struct Setup
{
    pub addresses: Vec<String>,
    pub node_count: u8,
    pub pool_handle: i32,
    pub trustees: Entities,
    pub users: Entities,
}

impl Setup
{
    pub fn new(wallet: &Wallet, config: SetupConfig) -> Setup
    {
        assert!(config.num_trustees > 0, "You need to have at least one trustee.");

        sovtoken::api::sovtoken_init();
        let pool_handle = Setup::setup_pool();
        let addresses = Setup::create_addresses(wallet, config.num_addresses);
        let trustees = Setup::create_trustees(wallet, pool_handle, config.num_trustees);

        let users = {
            let trustee_dids = trustees.dids();

            if let Some(token_vec) = config.mint_tokens {
                assert!(token_vec.len() <= config.num_addresses, "You are minting to more addresses than are available.");
                Setup::mint(wallet, pool_handle, &trustee_dids, &addresses, token_vec);
            }

            Setup::create_users(wallet, pool_handle, trustee_dids[0], config.num_users)
        };

        Setup {
            addresses,
            node_count: 4,
            pool_handle,
            trustees,
            users,
        }
    }

    fn setup_pool() -> i32
    {
        let pc_string = pool::create_pool_config();
        let pool_config = Some(pc_string.as_str());
        indy::pool::Pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

        let pool_name = pool::create_pool_ledger(pool_config);
        let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();      

        pool_handle  
    }

    fn create_users(wallet: &Wallet, pool_handle: i32, did_trustee: &str, num_users: u8) -> Entities
    {
        did::create_multiple_nym(wallet.handle, pool_handle, did_trustee, num_users, did::NymRole::User)
            .unwrap()
            .into_iter()
            .map(Entity::new)
            .collect()
    }

    fn create_trustees(wallet: &Wallet, pool_handle: i32, num_trustees: u8) -> Entities
    {
        did::initial_trustees(num_trustees, wallet.handle, pool_handle)
            .unwrap()
            .into_iter()
            .map(Entity::new)
            .collect()
    }

    fn create_addresses(wallet: &Wallet, num_addresses: usize) -> Vec<String>
    {
        gen_address::generate_n(wallet, num_addresses)
    }

    fn mint(wallet: &Wallet, pool_handle: i32, dids: &Vec<&str>, addresses: &Vec<String>, token_vec: Vec<u64>)
    {
        let map: HashMap<String, u64> = addresses
            .clone()
            .into_iter()
            .zip(token_vec.into_iter())
            .collect();

        mint::mint_tokens(map, pool_handle, wallet.handle, dids).unwrap();
    }
}

pub struct Entity
{
    pub did: String,
    pub verkey: String,
}

impl Entity {
    fn new((did, verkey): (String, String)) -> Self
    {
        Entity {
            did,
            verkey
        }
    }
}


use std::ops::{Index, IndexMut};

pub struct Entities(Vec<Entity>);

impl Entities {
    pub fn dids(&self) -> Vec<&str> {
        self.0
            .iter()
            .map(|trust| trust.did.as_str())
            .collect()
    }
}

impl Index<usize> for Entities
{
    type Output = Entity;

    fn index(&self, i: usize) -> &Entity {
        &self.0[i]
    }
}

impl IndexMut<usize> for Entities
{
    fn index_mut(&mut self, i: usize) -> & mut Entity {
        &mut self.0[i]
    }
}

impl FromIterator<Entity> for Entities
{
    fn from_iter<I: IntoIterator<Item=Entity>>(iter: I) -> Entities {
        let mut v = Vec::new();
        for entity in iter {
            v.push(entity);
        }

        Entities(v)
    }
}
