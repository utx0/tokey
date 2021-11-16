use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, msg};

use std::convert::TryInto;

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
    let (key, rem) = instruction_data
        .split_first()
        .ok_or_else(|| ProgramError::InvalidInstructionData)?;

    msg!("key: {:?}", key);
    msg!("rem: {:?}", rem);

    match key {
        0 => {
            msg!("Zero!");

            let value: u64 = rem
                .get(0..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .unwrap_or(0);

            msg!("value: {}", value);
        }
        1 => msg!("One!"),
        _ => msg!("Other: {:?}", key),
    }

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
