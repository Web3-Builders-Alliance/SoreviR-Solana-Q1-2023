/*
    We add a "use" line starting with the name of the crate, "solana_program", and
    listed the items we want to bring into scope.
*/
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

/*
    Here we declare the program entrypoint to begin program execution,
    a provided function to process the program instruction.
*/
entrypoint!(process_instruction);

//  This is the principal function that process the program instructions supplied by the runtime.
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // This is to confirm that the program ID specified in the instructions matches the actual ID of the program.
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    };

    /*
    This is to check the number of accounts passed to the intructions.
    If its less than 4, return an error
    */
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Here its accesing the items in the account_infos
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // This is to verify that the account has not been initialized
    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized);
    };

    // Now this verify that an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount);
    };

    /*
    I think this checks the account_to_change has the
    correct program_id associated with it.
    */
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId);
    };

    // Here verify the pubkey.
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    };

    Ok(())
}

/*
- What are the concepts (borrowing, ownership, vectors etc):
    Here are used the borrowing, vectors and conditionals concepts.

- What is the contract doing? What is the mechanism?
    This contract verify if the accounts that are passed in are correct.

- How could it be better? More efficient? Safer?
    I am not sure how could improve it yet.
*/
