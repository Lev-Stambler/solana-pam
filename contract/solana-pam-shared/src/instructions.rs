//! Instruction types
use borsh::maybestd::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use core::mem::transmute;
use solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};
use std::convert::TryInto;

use std::str::FromStr;

pub type UserAccessList = Vec<Pubkey>;
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ProgramData {
    pub user_access_lists: HashMap<[u8; 32], [u8; 32]>,
}

impl ProgramData {
    pub fn init() -> Self {
        ProgramData {
            user_access_lists: HashMap::new(),
        }
    }
    pub fn update(&mut self, user: &Pubkey, new_access_list_account: &Pubkey) -> ProgramResult {
        self.user_access_lists
            .insert(user.to_bytes(), new_access_list_account.to_bytes());
        Ok(())
    }
}

/// Instructions supported by the token program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum ProgInstruction {
    Init = 0,
    /// UpdateAccessList updates the access list for the caller
    ///
    /// Accounts expected
    /// program_account (W) - program state account
    /// new_access_list - an account with the caller's updated access list
    UpdateAccessList,
}

impl ProgInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input[0] {
            0 => Ok(ProgInstruction::Init),
            1 => Ok(ProgInstruction::UpdateAccessList),
            _ => Err(ProgramError::Custom(11)),
        }
    }
}

pub fn pack_user_access_list(input: UserAccessList) -> Vec<u8> {
    let keys: Vec<[u8; 32]> = input.iter().map(|pk| pk.to_bytes()).collect();
    unsafe {
        let ret: Vec<u8> = transmute(keys);
        ret
    }
}

pub fn unpack_user_access_list(input: &mut [u8]) -> Result<UserAccessList, ProgramError> {
    if input.len() % 32 != 0 {
        Err(ProgramError::Custom(12))
    } else {
        // TODO: unwrap for all
        let keys: Vec<Pubkey> = input
            .chunks(32)
            .into_iter()
            .map(|k| Pubkey::new_from_array(k.try_into().unwrap()))
            .collect();
        Ok(keys)
    }
}
