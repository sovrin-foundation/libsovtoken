/*!
 * A set of test helpers dealing with the wallet.
 */

extern crate indyrs as indy;
extern crate sovtoken;

use self::indy::wallet;
use self::sovtoken::utils::random::rand_string;

use indy::future::Future;

static USEFUL_CREDENTIALS: &'static str = r#"
   {
       "key": "12345678901234567890123456789012"
   }
"#;

/**
A test wallet that deletees itself when it leaves scope.

Use by calling `let wallet = Wallet::new();` and pass the `wallet.handle`.

```
use utils::wallet::Wallet;
// The wallet is opened and created.
let wallet_1 = Wallet::new();
{
    let wallet_2 = Wallet::new();
    // we have the wallet and wallet handle.
    assert!(wallet.handle > 0);
}
// Now wallet_2 is out of scope, it closes and deletes itself.
assert!(wallet.handle > 0);
```

*/
pub struct Wallet {
    name: String,
    pub handle: i32,
}

impl Wallet {
    /* constructors */
    pub fn new() -> Wallet {
        let wallet_name: String = rand_string(20);
        let mut wallet = Wallet { name: wallet_name, handle: -1 };
        wallet.create();
        wallet.open();

        return wallet;
    }

    pub fn from_name(name: &str) -> Wallet {
        let mut wallet = Wallet { name: name.to_string(), handle: -1 };
        wallet.create();
        wallet.open();

        return wallet;
    }

    /* private static method to help create config that is passed to wallet functions */
    fn create_wallet_config(wallet_name: &str) -> String {
        let config = json!({ "id" : wallet_name.to_string() }).to_string();
        return config.to_string();
    }

    /* private instance methods for open/create/etc...*/

    fn open(&mut self) -> i32 {
        let config: String = Wallet::create_wallet_config(&self.name);
        let handle = wallet::open_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();
        self.handle = handle;
        return handle;
    }

    fn create(&self) {
        let config = Wallet::create_wallet_config(&self.name);
        wallet::create_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap()
    }

    fn close(&self) {
        wallet::close_wallet(self.handle).wait().unwrap()
    }

    fn delete(&self) {
        let config: String = Wallet::create_wallet_config(&self.name);
        return wallet::delete_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        self.close();
        self.delete();
    }
}
