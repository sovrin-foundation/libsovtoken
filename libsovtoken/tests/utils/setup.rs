use std::collections::HashMap;
use std::iter::FromIterator;

use indy;
use sovtoken;
use sovtoken::logic::parsers::common::ResponseOperations;
use sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
use utils::did;
use utils::mint;
use utils::payment::fees as fees_utils;
use utils::payment::address as gen_address;
use utils::pool;
use utils::wallet::Wallet;
use serde_json;

const PROTOCOL_VERSION: usize = 2;


/**
Config to be passed to [`Setup::new`].

## Requirements
- `num_trustees` needs to be 1 or greater.
- `mint_tokens` needs to equal or less than `num_addresses`

## Fields
### num_trustees
A u8 which determines the number of trustees which will be created by a NYM
request. Their dids will be stored in the wallet. `num_trustees` needs to be at
least 1. The first trustee was already created in the genesis txn file.

### num_users
A u8 which determines the number of standard users which will be created by a
NYM request. Their dids will be stored in the wallet.

### num_addresses
A usize which determines the number of addresses which are generated with
a random key. These addresses are stored in the wallet.

### mint_tokens
An optional vector which will mint tokens to the corresponding index.
```no_run
let setup = Setup::new(&wallet, SetupConfig {
    num_trustees: 4,
    num_users: 0,
    num_addresses: 5,
    mint_tokens: Some(vec![300, 4, 10]),
    fees: None
});
```

In this case the following addresses will contain the tokens.
```
setup.addresses[0] => 300,
setup.addresses[1] => 4,
setup.addresses[2] => 10,
setup.addresses[3] => 0,
setup.addresses[4] => 0,
```

If `mint_tokens` is None, a mint will not occur.

### fees
A `serde_json::Value` which specifies the fees to be set on the ledger. You will
typically use the `json!` macro.

```no_run
let fees = json!({
    "1": 1
    "10002": 5,
    "303": 15
});

let setup = Setup::new(&wallet, SetupConfig {
    num_trustees: 4,
    num_users: 0,
    num_addresses: 1,
    mint_tokens: None,
    fees,
});
```

These fees set here, will all be set to 0, when the `Setup` is dropped.

[`Setup::new`]: Setup::new
*/
pub struct SetupConfig
{
    pub num_trustees: u8,
    pub num_users: u8,
    pub num_addresses: usize,
    pub mint_tokens: Option<Vec<u64>>,
    pub fees: Option<serde_json::Value>,
}


pub struct Setup<'a>
{
    pub addresses: Vec<String>,
    pub fees: Option<serde_json::Value>,
    pub node_count: u8,
    pub pool_handle: i32,
    pub trustees: Entities,
    pub users: Entities,
    wallet: &'a Wallet,
}

impl<'a> Setup<'a>
{

    /**
    Create a new Setup.

    Configures the pool, generates trustees and users, generate addresses, sets
    fees and mints tokens according to the [`SetupConfig`].

    [`SetupConfig`]: SetupConfig
    */
    pub fn new(wallet: &Wallet, config: SetupConfig) -> Setup
    {
        assert!(config.num_trustees > 0, "You need to have at least one trustee.");

        sovtoken::api::sovtoken_init();
        let pool_handle = Setup::setup_pool();
        let addresses = Setup::create_addresses(wallet, config.num_addresses);
        let trustees = Setup::create_trustees(wallet, pool_handle, config.num_trustees);

        let users;
        let mut fees = None;

        {
            let trustee_dids = trustees.dids();

            if let Some(token_vec) = config.mint_tokens {
                assert!(token_vec.len() <= config.num_addresses, "You are minting to more addresses than are available.");
                Setup::mint(wallet, pool_handle, &trustee_dids, &addresses, token_vec);
            }

            if let Some(f) = config.fees {
                fees_utils::set_fees(pool_handle, wallet.handle, PAYMENT_METHOD_NAME, &f.to_string(), &trustee_dids, Some(trustee_dids[0]));
                fees = Some(f);
            }

            users = Setup::create_users(wallet, pool_handle, trustee_dids[0], config.num_users);
        };

        Setup {
            addresses,
            fees,
            node_count: 4,
            pool_handle,
            trustees,
            users,
            wallet,
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

       let mint_rep = mint::mint_tokens(map, pool_handle, wallet.handle, dids).unwrap();
       assert_eq!(mint_rep.op, ResponseOperations::REPLY);
    }

    fn fees_reset_json(fees: Option<serde_json::Value>) -> Option<String>
    {
        if fees.is_some() {
            type FeesMap = HashMap<String, u64>;
            let fees: FeesMap = serde_json::from_value(fees.unwrap()).unwrap();
            let mut map = HashMap::new();
            
            for k in fees.keys() {
                map.insert(k, 0);
            }
            
            Some(serde_json::to_string(&map).unwrap())
        } else {
            None
        }
    }
}

impl<'a> Drop for Setup<'a> {
    fn drop(&mut self) {
        if let Some(reset_fees) = Setup::fees_reset_json(self.fees.take()) {
            let dids = self.trustees.dids();
            fees_utils::set_fees(
                self.pool_handle,
                self.wallet.handle,
                PAYMENT_METHOD_NAME,
                &reset_fees,
                &dids,
                Some(dids[0])
            );
        }
    }
}

/**
An entity with a did and a verkey.
*/
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

/**
Contain a vector of [`Entity`].

You can access elements like an array.
```
use utils::setup::{Entities, Entity};
let entities = Entities(vec![
    Entity::new((String::from("V4SGRU86Z58d6TV7PBUe6f"), String::from("4TFcJS5FBo42EModbbaeYXHFoQAnmZKWrWKt8yWTB6Bq")))
    Entity::new((String::from("7LQt1bEbk5zB6gaFbEPDzB"), String::from("GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL")))
    Entity::new((String::from("Ln7kZXHFxZg5689JZciJMJ"), String::from("BnDcRVr6ZUkrNmxB2pmUbKVeZSuSnBecLFJNteS9iiM4")))
]);

assert_eq!(entities[1].did, "7LQt1bEbk5zB6gaFbEPDzB")
```

[`Entity`]: Entity
*/
pub struct Entities(Vec<Entity>);

impl Entities {

    /**
    The dids of the entities without the verkey.
    */
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
