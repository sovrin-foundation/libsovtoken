/*!
 * A set of test helpers dealing with the wallet.
 */

extern crate env_logger;
extern crate sovtoken;

use sovtoken::utils::ErrorCode;
use self::sovtoken::utils::random::rand_string;
use sovtoken::utils::callbacks::ClosureHandler;

static USEFUL_CREDENTIALS : &'static str = r#"
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
        let wallet_name : String = rand_string(20);
        let mut wallet = Wallet { name : wallet_name , handle: -1 };
        wallet.create().unwrap();
        wallet.open().unwrap();

        return wallet;
    }

    pub fn from_name(name: &str) -> Wallet {
        let mut wallet = Wallet { name: name.to_string(), handle: -1 };
        wallet.create().unwrap();
        wallet.open().unwrap();

        return wallet;
    }

    /* private static method to help create config that is passed to wallet functions */
    fn create_wallet_config(wallet_name: &str) -> String {
        let config = json!({ "id" : wallet_name.to_string() }).to_string();
        return config.to_string();
    }

    /* private instance methods for open/create/etc...*/
    
    fn open(&mut self) -> Result<i32, ErrorCode> {
        let config : String = Wallet::create_wallet_config(&self.name);

        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let config = c_str!(&config);
        let credentials = c_str!(USEFUL_CREDENTIALS);

        let _err = ErrorCode::from(unsafe {
            indy_sys::indy_open_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), cb)
        });

        _err.try_err()?;
        let (_err, val) = receiver.recv()?;
        _err.try_err()?;
        let handle = Ok(val);

        self.handle = handle;
        return Ok(handle);
    }

    fn create(&self) -> Result<(), ErrorCode> {
        let config = Wallet::create_wallet_config(&self.name);
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let config = c_str!(&config);
        let credentials = c_str!(USEFUL_CREDENTIALS);

        let err =ErrorCode::from(unsafe {
            indy_sys::indy_create_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), cb)
        });
        err.try_err()?;
        match receiver.recv() {
            Ok(err) => err.try_err(),
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    fn close(&self) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = ErrorCode::from(unsafe { indy_sys::indy_close_wallet(command_handle, self.handle, cb) });
        err.try_err()?;
        match receiver.recv() {
            Ok(err) => err.try_err(),
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    fn delete(&self) -> Result<(), ErrorCode> {
        let config : String = Wallet::create_wallet_config(&self.name);
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let config = c_str!(&config);
        let credentials = c_str!(USEFUL_CREDENTIALS);

        let err = ErrorCode::from(unsafe {
            indy_sys::indy_delete_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), cb)
        });
        err.try_err()?;
        match receiver.recv() {
            Ok(err) => err.try_err(),
            Err(e) => Err(ErrorCode::from(e))
        }
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        self.close().unwrap();
        self.delete().unwrap();
    }
}
