mod contract;
#[cfg(feature = "daemon")]
mod daemon;
mod deploy;
mod error;
mod index_response;
mod interface;
#[cfg(feature = "daemon")]
mod keys;
mod mock;
// pub mod prelude;
mod state;
mod tx_handler;

pub use boot_contract_derive::boot_contract;
pub use boot_fns_derive::{ExecuteFns, QueryFns};
pub use contract::{Contract, ContractCodeReference};
pub use deploy::Deploy;
pub use error::BootError;
pub use index_response::IndexResponse;
pub use interface::{
    BootExecute, BootInstantiate, BootMigrate, BootQuery, BootUpload, CallAs, ContractInstance,
    CwInterface,
};
pub use mock::{
    core::{instantiate_custom_mock_env, instantiate_default_mock_env, Mock},
    state::MockState,
};
pub use state::{ChainState, StateInterface};
pub use tx_handler::{TxHandler, TxResponse};
// re-export as it is used in the public API
pub use cosmwasm_std::{Addr, Coin};
pub use cw_multi_test::ContractWrapper;

#[cfg(feature = "daemon")]
pub use daemon::{
    core::{instantiate_daemon_env, Daemon},
    error::DaemonError,
    networks,
    state::{DaemonOptions, DaemonOptionsBuilder},
};

/// Signals a supported execution environment
pub trait BootEnvironment: TxHandler + Clone {}
impl<T: TxHandler + Clone> BootEnvironment for T {}
