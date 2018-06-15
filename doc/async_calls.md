# Async Calls To Libindy

### What we have now

Libindy executes all handlers in its command thread. Right now in handler we try to make a synchronous call to sign inputs. That will hang because signing is executed on the same thread that executes callback and we are synchronously waiting for the result of signing on it -- it is a deadlock.

### What should be done

We need to avoid synchronous calls and use callbacks so that we can send the next command to libindy and the execution will continue in callback.

**NB:** We do really need to test such cases with calls from libindy, because it will be executed that way.

### How it can be done

In build_payment_req_handler we need signed inputs to build a payment request. So we should cover logic that works with signed inputs in callbacks and pass it to libindy. Example:

###### api/mod.rs
```rust
#[no_mangle]
pub extern "C" fn build_payment_req_handler(command_handle: i32,
                                            wallet_handle: i32,
                                            submitter_did: *const c_char,
                                            inputs_json: *const c_char,
                                            outputs_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                 err: i32,
                                                                 payment_req_json: *const c_char) -> i32>) -> i32 {

    //deserialization and all checks that we need to do before signing

    let fees = Fees::new(inputs, outputs);
    let result = fees.sign(&CryptoSdk {}, wallet_handle, move |fees_signed| {
        debug!("Signed fees >>> {:?}", fees_signed);
    
        //logic that works with signed fees before responding
    
        debug!("payment_request >>> {:?}", payment_request);
    
        cb(command_handle, ErrorCode::Success as i32, payment_request.as_ptr());
    });
    
    //mapping result of sending the request to ErrorCode
    ErrorCode::Success as i32
}
```
###### logic/fees.rs
```rust
impl Fees {
    //...
    
    // we are not returning Fees anymore, Result is just for a ErrorCode
    // also we receive a callback from the api
    pub fn sign<A: CryptoAPI>(mut self, crypto_api: &'static A, wallet_handle: IndyHandle, cb: Box<Fn(Result<Fees, ErrorCode>) + 'static + Send + Sync>) -> Result<(), ErrorCode> {
        if self.outputs.len() < 1 || self.inputs.len() < 1 {
            return Err(ErrorCode::CommonInvalidStructure);
        }

        for output in &mut self.outputs {
            let address = address::verkey_checksum_from_address(output.address.clone())?;
            output.address = address;
        }
        trace!("Indicator stripped from outputs");

        Fees::sign_inputs(crypto_api, wallet_handle, &self.inputs.clone(), &self.outputs.clone(), Box::new(move |inputs| {
                    match inputs {
                        Ok(inputs) => {
                            let mut fees = self.clone();
                            fees.inputs = inputs;
                            for input in &mut fees.inputs {
                                let address = match address::verkey_checksum_from_address(input.address.clone()){
                                    Ok(addr) => addr,
                                    Err(err) => {cb(Err(err)); return;}
                                };
                                input.address = address;
                            }
                            trace!("Indicator stripped from inputs");
                            cb(Ok(fees));
                        }
                        Err(err) => {cb(Err(err));}
                    };
                }
            )
        )
    }
}
//...

// InputSigner needs a rework to be asynchronous 

trait InputSigner<A: CryptoAPI> {
    fn sign_inputs(crypto_api: &'static A, wallet_handle: IndyHandle, inputs: &Inputs, outputs: &Outputs, cb: Box<Fn(Result<Inputs, ErrorCode>) + 'static + Send + Sync>)
                   -> Result<(), ErrorCode>
    {
        let inputs_result: Arc<Mutex<Inputs>> = Default::default();

        let res_cnt = inputs.len();
        let cb = Arc::new(move |input: Result<Input, ErrorCode>| {
            match input {
                Ok(input) => {
                    let mut results = inputs_result.lock().unwrap();
                    results.push(input);
                    if results.len() == res_cnt {
                        cb(Ok(results.to_vec()))
                    }
                }
                Err(err) => cb(Err(err))
            }
        });

        for input in inputs {
            let cb = cb.clone();
            match Self::sign_input(crypto_api, wallet_handle, input, outputs, Box::new(cb)) {
                err @ Err(_) => { return err; }
                _ => ()
            }
        }

        Ok(())
    }

    /**
     * Signs an [`Input`] with indy_crypto_sign
     *
     * Validates the `input`'s `address`, but not the `outputs`.
     * The message that will be signed is
     * `[[<address>, <seq_no>], [<Output>, <Output>, ...]]`
     *
     * [`Input`]: Input
     */
    fn sign_input(
        crypto_api: &'static A,
        wallet_handle: IndyHandle,
        input: &Input,
        outputs: &Outputs,
        cb: Box<Arc<Fn(Result<Input, ErrorCode>) + 'static + Send + Sync>>,
    ) -> Result<(), ErrorCode>
    {
        let verkey = address::verkey_from_address(input.address.clone())?;
        debug!("Received verkey for payment address >>> {:?}", verkey);

        let message_json_value = json!([[input.address, input.seq_no], outputs]);
        debug!("Message to sign >>> {:?}", message_json_value);

        let message = serde_json::to_string(&message_json_value)
            .map_err(|_| ErrorCode::CommonInvalidStructure)?
            .to_string();

        let input = input.to_owned();

        let ca = move |signed_string: Result<String, ErrorCode>| {
            debug!("Received encoded signature >>> {:?}", signed_string);
            let signed_input = signed_string.map(|sig| input.clone().sign_with(sig));
            cb(signed_input);
        };

        let ec = crypto_api.indy_crypto_sign(
            wallet_handle,
            verkey.clone(),
            message.clone(),
            ca,
        );

        if ec == ErrorCode::Success {
            Ok(())
        } else {
            Err(ec)
        }
    }
}
```