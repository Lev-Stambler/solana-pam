//! Instruction types
use borsh::de::BorshDeserialize;
use hex_slice::AsHex;
use solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};
use std::str::FromStr;

pub type UserAccessList = Vec<Pubkey>;

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

fn array_to_pk(input: &[u8]) -> Result<Pubkey, ProgramError> {
    Pubkey::from_str(&hex::encode(input)).map_err(|_| ProgramError::Custom(22))
}

pub fn unpack_user_access_list(input: &mut [u8]) -> Result<UserAccessList, ProgramError> {
    if input.len() % 32 != 0 {
        Err(ProgramError::Custom(12))
    } else {
        // TODO: unwrap for all
        let keys: Vec<Pubkey> = input
            .chunks(32)
            .into_iter()
            .map(|k| array_to_pk(k).unwrap())
            .collect();
        Ok(keys)
    }
}
