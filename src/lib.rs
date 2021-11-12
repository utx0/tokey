use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, msg};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "program_id: {}, accounts: {:?}, data: {:?}",
        program_id,
        accounts,
        instruction_data
    );

    // let key: &u8 = instruction_data
    //     .first()
    //     .ok_or_else(|| ProgramError::InvalidInstructionData)?;

    // let key: &u8 = instruction_data.first().unwrap();

    // match key {
    //     0 => msg!("Zero!"),
    //     1 => msg!("One!"),
    //     _ => msg!("Other: {:?}", key),
    // }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
