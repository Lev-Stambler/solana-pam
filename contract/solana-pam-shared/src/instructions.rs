//! Instruction types
use borsh::maybestd::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use core::mem::transmute;
use solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};
use std::{convert::TryInto, ops::Deref};

use std::str::FromStr;

pub type UserAccessList = Vec<Pubkey>;
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ProgramData {
    pub user_access_map: HashMap<[u8; 32], [u8; 32]>,
}

impl ProgramData {
    pub fn init() -> Self {
        ProgramData {
            user_access_map: HashMap::new(),
        }
    }
    pub fn update(&mut self, user: &Pubkey, new_access_list_account: &Pubkey) -> ProgramResult {
        self.user_access_map
            .insert(user.to_bytes(), new_access_list_account.to_bytes());
        Ok(())
    }
}

/// Instructions supported by the token program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum ProgInstruction {
    Init,
    /// UpdateAccessList updates the access list for the caller
    ///
    /// Accounts expected
    /// program_account (W) - program state account
    /// new_access_list - an account with the caller's updated access list
    UpdateAccessList,
    // TODO: bulk operations
    // TODO: make it work s.t. Add and Remove are only callable if the 2nd account is signed and its
    // signed by the contract owner s.t. user_access_map shows that (signer) => access_list.key()
    AddPKToAccessListAccount(Pubkey),
    RemovePKToAccessListAccount(Pubkey),
}

impl ProgInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input[0] {
            0 => Ok(ProgInstruction::Init),
            1 => Ok(ProgInstruction::UpdateAccessList),
            2 => {
                let pk = &input[1..33];
                Ok(ProgInstruction::AddPKToAccessListAccount(
                    Pubkey::new_from_array(pk.try_into().unwrap()),
                ))
            }
            3 => {
                let pk = &input[1..33];
                Ok(ProgInstruction::RemovePKToAccessListAccount(
                    Pubkey::new_from_array(pk.try_into().unwrap()),
                ))
            }
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

pub fn user_access_list_add_pk(mut access_list: UserAccessList, add: Pubkey) -> ProgramResult {
    if access_list.iter().find(|pk| pk.deref().eq(&add)) == None {
        access_list.push(add);
    }
    Ok(())
}

pub fn user_access_list_remove_pk(
    mut access_list: UserAccessList,
    remove: Pubkey,
) -> ProgramResult {
    if let Some(idx) = access_list.iter().position(|&r| r.eq(&remove)) {
        access_list.remove(idx);
    }
    Ok(())
}
