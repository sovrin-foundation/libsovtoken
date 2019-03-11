use sovtoken::utils::ErrorCode;
use sovtoken::utils::callbacks::ClosureHandler;

type DidAndVerKey = (String, String);

#[derive(Clone, Copy)]
pub enum NymRole
{
    Trustee,
    User,
}

impl NymRole
{
    pub fn prepare(&self) -> Option<&str>
    {
        match self {
            NymRole::Trustee => Some("TRUSTEE"),
            NymRole::User => None,
        }
    }
}


/**
Generate a did and send a nym request for it.
*/
pub fn create_nym(
    wallet_handle: i32,
    pool_handle: i32,
    did_trustee: &str,
    role: NymRole
) -> Result<DidAndVerKey, ErrorCode> {
    let (did, verkey) = _new_did(wallet_handle,"{}").unwrap();

    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let submitter_did = c_str!(did_trustee);
    let target_did = c_str!(&did);

    let verkey_str = opt_c_str!(Some(&verkey));
    let data_str = opt_c_str!(None);
    let role_str = opt_c_str!(role.prepare());

    let err = ErrorCode::from(unsafe {
        indy_sys::indy_build_nym_request(command_handle,
                                       submitter_did.as_ptr(),
                                       target_did.as_ptr(),
                                       opt_c_ptr!(verkey, verkey_str),
                                       opt_c_ptr!(data, data_str),
                                       opt_c_ptr!(role, role_str),
                                       cb)
    });
    err.try_err()?;
    let (err, val) = receiver.recv()?;
    err.try_err()?;
    let req_nym = Ok(val)?;

    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let submitter_did = c_str!(submitter_did);
    let request_json = c_str!(request_json);

    let err = ErrorCode::from(unsafe {
        indy_sys::indy_sign_and_submit_request(command_handle,
                                             pool_handle,
                                             wallet_handle,
                                             submitter_did.as_ptr(),
                                             request_json.as_ptr(),
                                             cb)
    });

    Ok((did, verkey))
}

/**
Creates multiple dids and corresponding nym requests.
*/
pub fn create_multiple_nym(
    wallet_handle: i32,
    pool_handle: i32,
    did_trustee: &str,
    n: u8,
    role: NymRole
) -> Result<Vec<DidAndVerKey>, ErrorCode> {
    let mut v: Vec<(String, String)> = Vec::new();
    for _ in 0..n {
        let new_did = create_nym(wallet_handle, pool_handle, did_trustee, role)?;
        v.push(new_did);
    }

    Ok(v)
}

/**
Create and store the initial dids of trustees.

Includes the initial trustee.
*/
pub fn initial_trustees(num_trustees: u8, wallet_handle: i32, pool_handle: i32) -> Result<Vec<DidAndVerKey>, ErrorCode> {
    let first = initial_trustee(wallet_handle);

    let mut trustees = create_multiple_nym(
        wallet_handle,
        pool_handle,
        &first.0,
        num_trustees - 1,
        NymRole::Trustee
    )?;
    trustees.insert(0, first);

    Ok(trustees)
}

/**
Store the did of the intial trustee
*/
pub fn initial_trustee(wallet_handle: i32) -> DidAndVerKey {
    let first_json_seed = json!({
        "seed":"000000000000000000000000Trustee1"
    }).to_string();

    _new_did(wallet_handle,&first_json_seed).unwrap();

}

/**
Discard the verkey and return the did from a `Vec<DidAndVerKey`.
*/
pub fn did_str_from_trustees<'a>(trustees: &'a Vec<DidAndVerKey>) -> Vec<&'a str> {
    trustees
        .iter()
        .map(|(ref did, _)| did.as_str())
        .collect()
}

fn _new_did(wallet_handle: i32, did_json: &str) -> Result<(String, String), ErrorCode>{
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let did_json = c_str!(did_json);

    ErrorCode::from(unsafe {
        indy_sys::indy_create_and_store_my_did(command_handle, wallet_handle, did_json.as_ptr(), cb)
    });
    err.try_err()?;

    let (err, val, val2) = receiver.recv()?;

    err.try_err()?;

    Ok((val, val2));
}