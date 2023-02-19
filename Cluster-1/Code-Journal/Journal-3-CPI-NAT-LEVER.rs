/*
    We add a "use" line starting with the name of the crates, "borsh" and "solana_program", with
    list of the items we want to bring into scope.
*/
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

#[cfg(not(feature = "no-entrypoint"))] //I didn't understand what this expression does
entrypoint!(process_instruction);

//  This is the principal function that process the program instructions supplied by the runtime.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //   To get the power status.
    match PowerStatus::try_from_slice(&instruction_data) {
        Ok(power_status) => return initialize(program_id, accounts, power_status),
        Err(_) => {}
    }

    // To set the power status.
    match SetPowerStatus::try_from_slice(&instruction_data) {
        Ok(set_power_status) => return switch_power(accounts, set_power_status.name),
        Err(_) => {}
    }

    Err(ProgramError::InvalidInstructionData)
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_span = (power_status.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke(
        &system_instruction::create_account(
            &user.key,
            &power.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[user.clone(), power.clone(), system_program.clone()],
    )?;

    // This serialize the power.data from power_status
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    Ok(())
}

pub fn switch_power(accounts: &[AccountInfo], name: String) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;

    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;
    power_status.is_on = !power_status.is_on;
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    msg!("{} is pulling the power switch!", &name);

    match power_status.is_on {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };

    Ok(())
}

//  This create implmentations for the traits to the struct that follows it.
#[derive(BorshDeserialize, BorshSerialize, Debug)]
//  This create a struct with the data
pub struct SetPowerStatus {
    pub name: String,
}

//  This create implmentations for the traits to the struct that follows it.
#[derive(BorshDeserialize, BorshSerialize, Debug)]
//  This create a struct with the data
pub struct PowerStatus {
    pub is_on: bool,
}

/*
- What are the concepts (borrowing, ownership, vectors etc):
    Here are used struct, match concepts.

- What is the contract doing? What is the mechanism?
    The contract turn on and off the power status

- How could it be better? More efficient? Safer?
    I don't know how to improve it yet.
*/
