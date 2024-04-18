
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::AccountInfo,
    account_info::next_account_info,
    program::invoke_signed,
    sysvar::rent::Rent,
    system_instruction, 
    sysvar::Sysvar,
    borsh::try_from_slice_unchecked
};
pub mod instructions;
pub mod state;
use instructions::MovieInstruction;
// use solana_sdk::{account_info::next_account_info, program::invoke_signed, rent::Rent, system_instruction, sysvar::Sysvar};
use state::MovieAccountState;

entrypoint!(process_instructions);

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    description: String,
    rating: u8
) -> ProgramResult {
    let accounts_info_iter = &mut accounts.iter();
    let initializer = next_account_info(accounts_info_iter)?; 
    let pda_account = next_account_info(accounts_info_iter)?;
    let system_program = next_account_info(accounts_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id
    ); 
    let accounts_len = 1+1+(4 + title.len())+(4+description.len());
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(accounts_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key, 
            rent_lamports, 
            accounts_len.try_into().unwrap(), 
            program_id
        ), 
        &[initializer.clone(), pda_account.clone(), system_program.clone()], 
        &[&[initializer.key.as_ref(), title.as_bytes().as_ref(), &[bump_seed]]]
    )?;
    msg!("PDA account created {}", pda);

    msg!("deserializing data");    
    let mut account_data = MovieAccountState::try_from_slice(&pda_account.data.borrow_mut())?;
    
    account_data.title = title;
    account_data.description = description;
    account_data.rating = rating;
    account_data.is_initialized = true;

    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("serialized and saved movie review!");
    Ok(())
}

pub fn process_instructions(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let instruction = MovieInstruction::unpack(instruction_data)?;
    match instruction {
        MovieInstruction::AddMovieReview { title, rating, description } => {
            add_movie_review(program_id, accounts, title, description, rating)
            msg!("Title: {}", title);
            msg!("Description: {}", description);
            msg!("Rating: {}", rating);
            Ok(())
        }
    }
}