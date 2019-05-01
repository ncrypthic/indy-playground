extern crate indyrs;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;
extern crate futures;

use indyrs::{IndyError, IndyHandle};
use indyrs::future::Future;
use serde::{Serialize, Deserialize};

#[macro_use]
extern crate log;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LedgerConfig {
    genesis_txn: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ledger {
    name: String,
    config: LedgerConfig,
}

impl Ledger {
    fn setup(&self) -> impl Future<Item=IndyHandle, Error=IndyError> + '_{
        use indyrs::{pool, ErrorCode};
        use futures::future::{ok, err};
        let maybe_config = serde_json::to_string(&self.config);
        pool::set_protocol_version(2)
            .then(move |_| match maybe_config {
                Ok(json) => ok((json, &self.name)),
                Err(_) => err(IndyError{
                    error_code: ErrorCode::CommonInvalidParam1,
                    message: "Bad json config".to_string(),
                    indy_backtrace: None,
                }),
            })
            .and_then(|data| pool::create_pool_ledger_config(&data.1, Some(&data.0)).then(|_| ok(data)))
            .and_then(|data| pool::open_pool_ledger(&data.1, Some(&data.0)).then(|res| match res {
                Ok(x) => {
                    info!("Successfully open ledger: {}", x);
                    futures::future::ok(x)
                },
                Err(err) => {
                    error!("Failed to open ledger: {}", err);
                    futures::future::err(err)
                }
            }))
    }
}

fn main() {
    use tokio_core::reactor::Core;
    let ledger_config = LedgerConfig{
        genesis_txn: "pool_genesis.json".to_owned(),
    };
    let ledger = Ledger{
        name: "Node4".to_owned(),
        config: ledger_config,
    };

    let mut core = Core::new().unwrap();
    let res = core.run(ledger.setup());
    match res {
        Ok(_) => println!("Success"),
        Err(err) => println!("Error: {}", err),
    }
    println!("This should run before ledger");
}
