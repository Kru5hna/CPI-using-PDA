use solana_program::{
    account_info::{AccountInfo, next_account_info},
    address_lookup_table::instruction,
    entrypoint::{ ProgramResult},
    entrypoint,
    lamports,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    stake::instruction::create_account,
    system_instruction, system_program,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let pda = next_account_info(iter)?;
    let user_acc = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    let seeds: &[&[u8]] = &[user_acc.key.as_ref(), b"user"];
    let (pda_public_key, bump) = Pubkey::find_program_address(seeds, program_id);

    let ix = system_instruction::create_account(
        user_acc.key,    // payer
        &pda_public_key, // new PDA account
        1000000000,      // lamports
        8,               // space
        program_id,      // owner
    );

    // Build seed slice correctly
    let signer_seeds: &[&[u8]] = &[user_acc.key.as_ref(), b"user", &[bump]];

    // Invoke with PDA signing
    invoke_signed(
        &ix,
        accounts,        // must contain user_acc (payer) + pda account
        &[signer_seeds], // PDA seeds with bump
    )?;

    Ok(())
}
