#![no_std]

use gstd::{msg, prelude::*};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use core::iter;
use sails_rs::meta::{AnyServiceMeta, ProgramMeta};
use scale_info::MetaType;

#[derive(Encode, Decode, Clone, TypeInfo)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum BlockchainAction {
    AddTransaction(Transaction),
    ViewBlockchain,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum BlockchainEvent {
    TransactionAdded(Transaction),
    BlockchainViewed(Vec<Transaction>),
}

static mut BLOCKCHAIN: Vec<Transaction> = Vec::new();

#[no_mangle]
extern "C" fn handle() {
    let action: BlockchainAction = msg::load().unwrap();
    match action {
        BlockchainAction::AddTransaction(tx) => {
            unsafe {
                BLOCKCHAIN.push(tx.clone());
            }
            msg::reply(BlockchainEvent::TransactionAdded(tx), 0).unwrap();
        }
        BlockchainAction::ViewBlockchain => {
            unsafe {
                msg::reply(BlockchainEvent::BlockchainViewed(BLOCKCHAIN.clone()), 0).unwrap();
            }
        }
    }
}

#[derive(TypeInfo)]
pub struct BlockchainProgram;

impl ProgramMeta for BlockchainProgram {
    fn constructors() -> MetaType {
        MetaType::new::<BlockchainProgram>()
    }

    fn services() -> impl gstd::Iterator<Item = (&'static str, AnyServiceMeta)> {
        iter::empty()
    }
}
