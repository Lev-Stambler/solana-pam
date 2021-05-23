//! Program state processor
use solana_pam_shared::instructions::{
    pack_user_access_list, unpack_user_access_list, user_access_list_add_pk,
    user_access_list_remove_pk, ProgInstruction, ProgramData, UserAccessList,
};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

fn process_change_access_list_account(
    program_account: &AccountInfo,
    access_list_account: &AccountInfo,
    pk: Pubkey,
    add: bool,
) -> ProgramResult {
    let mut access_list_data = access_list_account.data.borrow_mut();
    let access_list = unpack_user_access_list(access_list_data.deref_mut()).unwrap();
    if add {
        user_access_list_add_pk(access_list, pk)
    } else {
        user_access_list_remove_pk(access_list, pk)
    }
}

fn process_init_access_list(
    mut program_data: ProgramData,
    access_list_account: &AccountInfo,
    signer: &Pubkey,
) -> ProgramResult {
    let mut access_list_data = access_list_account.data.borrow_mut();
    let data = access_list_data.deref_mut();
    data.clone_from_slice(pack_user_access_list(Vec::new()).as_slice());
    // let access_list = unpack_user_access_list(access_list_data.deref_mut()).unwrap();
    // msg!("New Access List: {:?}", access_list);
    program_data
        .user_access_map
        .insert(signer.to_bytes(), access_list_account.key.to_bytes());
    Ok(())
}

fn process_init(mut program_account: &AccountInfo) -> ProgramResult {
    msg!("PROCESSING INIT");
    let mut new_state = ProgramData::new();
    msg!("PROCESSING INIT 2");
    let new_state_vec = &new_state.try_to_vec().map_err(|e| {
        msg!("{}", e.to_string());
        e
    })?;
    msg!("PROCESSING INIT 2.5");
    program_account.try_borrow_mut_data()?.copy_from_slice(new_state_vec);
    msg!("PROCESSING INIT 3");
    Ok(())
}

fn get_user_access_list_pk<'a>(
    address: &Pubkey,
    program_data: &'a ProgramData,
) -> Option<&'a [u8; 32]> {
    program_data.user_access_map.get(&address.to_bytes())
}

fn user_matches_access_list<'a>(
    access_list: &Pubkey,
    user: &Pubkey,
    program_data: ProgramData,
) -> bool {
    if let Some(access_list_from_prog) = get_user_access_list_pk(user, &program_data) {
        if access_list
            .to_bytes()
            .iter()
            .zip(access_list_from_prog)
            .all(|(a, b)| a == b)
        {
            return true;
        }
    }
    false
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
    if instr == ProgInstruction::Init {
        return process_init(program_account);
    }
    let program_data =
        ProgramData::try_from_slice(program_account.try_borrow_data()?.as_ref()).unwrap();

    match instr {
        ProgInstruction::InitAccessList => {
            let update = next_account_info(account_info_iter)?;
            if let Some(address) = &update.signer_key() {
                process_init_access_list(program_data, update, address)
            } else {
                Err(ProgramError::Custom(111))
            }
        }
        ProgInstruction::Init => panic!("SHOULD NOT GET HERE"),
        ProgInstruction::AddPKToAccessListAccount(add) => {
            let update = next_account_info(account_info_iter)?;
            if let Some(address) = update.signer_key() {
                if user_matches_access_list(&update.key, address, program_data) {
                    process_change_access_list_account(program_account, update, add, true)
                } else {
                    Err(ProgramError::Custom(112))
                }
            } else {
                Err(ProgramError::Custom(111))
            }
        }
        ProgInstruction::RemovePKToAccessListAccount(remove) => {
            let update = next_account_info(account_info_iter)?;
            if let Some(address) = update.signer_key() {
                if user_matches_access_list(&update.key, address, program_data) {
                    process_change_access_list_account(program_account, update, remove, false)
                } else {
                    Err(ProgramError::Custom(112))
                }
            } else {
                Err(ProgramError::Custom(111))
            }
        }
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
