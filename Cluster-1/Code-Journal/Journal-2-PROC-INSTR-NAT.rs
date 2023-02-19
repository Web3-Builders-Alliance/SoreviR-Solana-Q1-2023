/*
    We add a "use" line starting with the name of the crates, "borsh" and "solana_program", with
    list of the items we want to bring into scope.
*/
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
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
    instruction_data: &[u8],
) -> ProgramResult {
    // Attempt to serialize the BPF format to our struct using Borsh
    let instruction_data_object = InstructionData::try_from_slice(&instruction_data)?;

    //Display a greating
    msg!("Welcome to the park, {}!", instruction_data_object.name);

    //  Check if the height is more than 5 and return a display a msg
    if instruction_data_object.height > 5 {
        msg!("You are tall enough to ride this ride. Congratulations.");
    } else {
        msg!("You are NOT tall enough to ride this ride. Sorry mate.");
    };

    Ok(())
}

//  This create implmentations for the traits to the struct that follows it.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
//  This create a struct with the data of the user
pub struct InstructionData {
    name: String,
    height: u32,
}

/*
- What are the concepts (borrowing, ownership, vectors etc):
    Here are used struct and conditionals concepts.

- What is the contract doing? What is the mechanism?
    This contract is greeting an user and checking if its possible for the user
    to ride a ride depending on the height

- How could it be better? More efficient? Safer?
    I am not sure how could improve it yet.
*/
