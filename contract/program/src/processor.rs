//! Program state processor
use solana_pam_shared::instructions::{
    unpack_user_access_list, ProgInstruction, ProgramData, UserAccessList,
};
use std::ops::DerefMut;
use std::rc::Rc;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::str::from_utf8;

fn process_update_access_list(
    program_account: &AccountInfo,
    access_list_account: &AccountInfo,
    signer: &Pubkey,
) -> ProgramResult {
    let mut program_data = program_account.data.borrow_mut();
    let mut prog_data = ProgramData::try_from_slice(&program_data.deref_mut()).unwrap();
    let mut access_list_data = access_list_account.data.borrow_mut();
    let access_list = unpack_user_access_list(access_list_data.deref_mut()).unwrap();
    msg!("New Access List: {:?}", access_list);
    prog_data.update(signer, access_list_account.key)
}

fn process_init(program_account: &AccountInfo) -> ProgramResult {
    program_account
        .data
        .borrow_mut()
        .copy_from_slice(&ProgramData::init().try_to_vec().unwrap());
    Ok(())
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let mut missing_required_signature = false;
    let program_account = next_account_info(account_info_iter)?;
    let x = program_account;
    // for account_info in account_info_iter {
    //     if let Some(address) = account_info.signer_key() {
    //         msg!("Signed by {:?}", address);
    //     } else {
    //         missing_required_signature = true;
    //     }
    // }
    if missing_required_signature {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let instr = ProgInstruction::unpack(input)?;
    match instr {
        ProgInstruction::UpdateAccessList => {
            let update = next_account_info(account_info_iter)?;
            if let Some(address) = &update.signer_key() {
                process_update_access_list(program_account, update, address)
            } else {
                Err(ProgramError::Custom(111))
            }
        }
        ProgInstruction::Init => process_init(),
    }
    // let memo = from_utf8(input).map_err(|err| {
    //     msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
    //     ProgramError::InvalidInstructionData
    // })?;
    // msg!("Memo (len {}): {:?}", memo.len(), memo);

    // Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{
        account_info::IntoAccountInfo, program_error::ProgramError, pubkey::Pubkey,
    };
    use solana_sdk::account::Account;

    #[test]
    fn test_utf8_memo() {
        let program_id = Pubkey::new(&[0; 32]);

        let string = b"letters and such";
        assert_eq!(Ok(()), process_instruction(&program_id, &[], string));

        let emoji = "🐆".as_bytes();
        let bytes = [0xF0, 0x9F, 0x90, 0x86];
        assert_eq!(emoji, bytes);
        assert_eq!(Ok(()), process_instruction(&program_id, &[], &emoji));

        let mut bad_utf8 = bytes;
        bad_utf8[3] = 0xFF; // Invalid UTF-8 byte
        assert_eq!(
            Err(ProgramError::InvalidInstructionData),
            process_instruction(&program_id, &[], &bad_utf8)
        );
    }

    #[test]
    fn test_signers() {
        let program_id = Pubkey::new(&[0; 32]);
        let memo = "🐆".as_bytes();

        let pubkey0 = Pubkey::new_unique();
        let pubkey1 = Pubkey::new_unique();
        let pubkey2 = Pubkey::new_unique();
        let mut account0 = Account::default();
        let mut account1 = Account::default();
        let mut account2 = Account::default();

        let signed_account_infos = vec![
            (&pubkey0, true, &mut account0).into_account_info(),
            (&pubkey1, true, &mut account1).into_account_info(),
            (&pubkey2, true, &mut account2).into_account_info(),
        ];
        assert_eq!(
            Ok(()),
            process_instruction(&program_id, &signed_account_infos, memo)
        );

        assert_eq!(Ok(()), process_instruction(&program_id, &[], memo));

        let unsigned_account_infos = vec![
            (&pubkey0, false, &mut account0).into_account_info(),
            (&pubkey1, false, &mut account1).into_account_info(),
            (&pubkey2, false, &mut account2).into_account_info(),
        ];
        assert_eq!(
            Err(ProgramError::MissingRequiredSignature),
            process_instruction(&program_id, &unsigned_account_infos, memo)
        );

        let partially_signed_account_infos = vec![
            (&pubkey0, true, &mut account0).into_account_info(),
            (&pubkey1, false, &mut account1).into_account_info(),
            (&pubkey2, true, &mut account2).into_account_info(),
        ];
        assert_eq!(
            Err(ProgramError::MissingRequiredSignature),
            process_instruction(&program_id, &partially_signed_account_infos, memo)
        );
    }
}
