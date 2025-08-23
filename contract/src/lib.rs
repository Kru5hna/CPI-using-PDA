use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::{ ProgramResult},entrypoint, pubkey::Pubkey};



entrypoint!(process_instruction);

fn process_instruction(
   program_id: &Pubkey,
   accounts: &[AccountInfo],
   instruction_data: &[u8],
) -> ProgramResult {
   let mut iter = accounts.iter();
   let mut data_Account = next_account_info(&mut iter); 

   Ok(())
}