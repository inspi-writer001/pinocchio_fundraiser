use bytemuck::{Pod, Zeroable};
use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    AccountView, Address, ProgramResult,
};
use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::{
    instructions::InitializeAccount,
    state::{Mint, TokenAccount},
};

use crate::state::Fundraiser;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct InitData {
    min_amount_sendable: u64,
    max_amount_sendable: u64,
}

impl InitData {
    pub const LEN: usize = core::mem::size_of::<InitData>();
}

pub fn process_initialize_instruction(accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    // load accounts
    let [maker, mint_to_raise, fundraiser, vault, system_program, token_program, associated_token_program, rent_sysvar, _extra @ ..] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // check that maker is a signer
    assert!(maker.is_signer(), "Maker should be signer");

    // cast data to type
    let parsed_data = bytemuck::from_bytes::<InitData>(&data[..InitData::LEN]);

    // constraints
    // check that mint exists [similar to mut in ancor]
    let mint_as_state_account = Mint::from_account_view(mint_to_raise).unwrap();
    assert!(
        mint_as_state_account.is_initialized(),
        "Mint you passed does not exist"
    );

    // check that fundraiser is empty
    assert!(fundraiser.is_data_empty(), "Wrong Fundraiser");

    // check that vault is empty and is not initialized
    let vault_as_state_account = TokenAccount::from_account_view(vault).unwrap();
    assert!(
        !vault_as_state_account.is_initialized() && vault_as_state_account.amount() == 0,
        "Vault is already initialized"
    );

    let rent = Rent::get()?;
    let minimum_balance = rent.minimum_balance_unchecked(Fundraiser::LEN);

    let seed = [b"fundraiser".as_ref(), maker.address().as_ref()];
    let (created_fundraiser, fundraiser_bump) = Address::find_program_address(&seed, &crate::ID);

    let bump = fundraiser_bump.to_le_bytes();

    let pda_seeds = [
        Seed::from(b"fundraiser"),
        Seed::from(maker.address().as_ref()),
        Seed::from(&bump),
    ];

    // compare fundraiser accounts from client vs onchain
    assert_eq!(
        &created_fundraiser,
        fundraiser.address(),
        "Fundraiser does not match"
    );

    // Create Fundraiser Account
    CreateAccount {
        from: maker,
        lamports: minimum_balance,
        owner: &crate::ID,
        space: Fundraiser::LEN as u64,
        to: fundraiser,
    }
    .invoke_signed(&[Signer::from(&pda_seeds)])?;

    // Create ATA for Fundraiser
    InitializeAccount {
        account: vault,
        mint: mint_to_raise,
        owner: fundraiser,
        rent_sysvar,
    }
    .invoke()?;

    // write to the created account
    let mut mut_borrow = fundraiser.try_borrow_mut().unwrap();
    let mut fundraiser_mutable = bytemuck::from_bytes_mut::<Fundraiser>(&mut mut_borrow);

    fundraiser_mutable.amount_to_raise = parsed_data.max_amount_sendable;

    Ok(())
}
