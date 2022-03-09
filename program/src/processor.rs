use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program::{invoke_signed},
    account_info::{AccountInfo, next_account_info, Account} , entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent}, config::program,
};

use solana_program::sysvar::Sysvar;
use crate::error::EchoError;
use crate::instruction::EchoInstruction;
pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("instruction data {:?}", instruction_data);
        let expected = EchoInstruction::InitializeAuthorizedEcho{
            buffer_seed: 3,
            buffer_size: 2,
        };
        let vec = BorshSerialize::try_to_vec(&expected);
        msg!("vec here {:?}", vec);
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            EchoInstruction::Echo { data } => {
                msg!("Instruction: Echo");
                // Err(EchoError::NotImplemented.into())
                process_echo(_program_id, _accounts, &data)
            }
            EchoInstruction::InitializeAuthorizedEcho {
                buffer_seed,
                buffer_size,
            } => {
                msg!("Instruction: InitializeAuthorizedEcho");
                // Err(EchoError::NotImplemented.into())
                process_initialize_authorized_echo(_program_id, _accounts, buffer_size, buffer_seed)
            }
            EchoInstruction::AuthorizedEcho { data } => {
                msg!("Instruction: AuthorizedEcho");
                // Err(EchoError::NotImplemented.into())
                process_authorized_echo(_program_id, _accounts, &data)
            }
            EchoInstruction::InitializeVendingMachineEcho {
                price,
                buffer_size,
            } => {
                // msg!("Instruction: InitializeVendingMachineEcho");
                // Err(EchoError::NotImplemented.into())
                process_initialize_vending_machine_echo(_program_id, accounts, price, buffer_size)
            }
            EchoInstruction::VendingMachineEcho { data: _ } => {
                msg!("Instruction: VendingMachineEcho");
                Err(EchoError::NotImplemented.into())
            }
        }
    }
}

fn process_echo(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    echo_buffer: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let echo_buffer_account = next_account_info(account_info_iter)?;
    let mutable_echo_account = &mut echo_buffer_account.try_borrow_mut_data()?;
    mutable_echo_account.clone_from_slice(echo_buffer);
    Ok(())
}

fn process_initialize_authorized_echo(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    buffer_size: usize,
    buffer_seed: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let authorized_buffer = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let seeds_without_bump = &[
        b"authority",
        authority.key.as_ref(),
        &buffer_seed.to_le_bytes()
    ];
    let (found_key, found_bump) = Pubkey::find_program_address(
        seeds_without_bump, 
        program_id
    );

    if found_key.to_bytes() != authorized_buffer.key.to_bytes() {
        return Err(ProgramError::InvalidAccountData)
    }

    let seeds_with_bump = &[
        b"authority",
        authority.key.as_ref(),
        &buffer_seed.to_le_bytes(),
        &[found_bump]
    ];

    invoke_signed(
        &system_instruction::create_account(
            authority.key,
            authorized_buffer.key,
            Rent::get()?.minimum_balance(buffer_size),
            buffer_size as u64,
            program_id,
        ),
        &[
            authority.clone(),
            authorized_buffer.clone(),
            system_program.clone(),
        ],
        &[seeds_with_bump],
    )?;

    let mut mutable_authorized_buffer = authorized_buffer.try_borrow_mut_data()?;

    mutable_authorized_buffer[0] = found_bump as u8;
    mutable_authorized_buffer[1..9].clone_from_slice(&buffer_seed.to_le_bytes());

    // let mutable_echo_account = &mut echo_buffer_account.try_borrow_mut_data()?;
    // mutable_echo_account.clone_from_slice(echo_buffer);
    Ok(())
}

fn process_authorized_echo(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input_echo_buffer: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let authorized_buffer = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    let mut buffer = authorized_buffer.try_borrow_mut_data()?;
    let buffer_len = buffer.len();
    let seeds = &[
        b"authority",
        authority.key.as_ref(),
        &buffer[1..9],
        &[buffer[0]]
    ];
    let found_key= Pubkey::create_program_address(
        seeds, 
        program_id
    )?;

    if found_key != *authorized_buffer.key {
        return Err(ProgramError::InvalidAccountData)
    }

    let end = (input_echo_buffer.len() + 9).min(buffer_len);
    buffer[9..end].clone_from_slice(&input_echo_buffer[..end - 9]);
    Ok(())
}

fn process_initialize_vending_machine_echo(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    price: u64,
    buffer_size: usize
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let vending_machine_buffer = next_account_info(account_info_iter)?;
    let vending_machine_mint = next_account_info(account_info_iter)?;
    let payer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (authorized_buffer_key, bump_seed) = Pubkey::find_program_address(
        &[
            b"vending_machine",
            vending_machine_mint.key.as_ref(),
            &price.to_le_bytes(),
        ],
        program_id
    );

    let seeds_with_bump = &[
        b"vending_machine",
        vending_machine_mint.key.as_ref(),
        &price.to_le_bytes(),
        &[bump_seed]
    ];
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            vending_machine_buffer.key,
            Rent::get()?.minimum_balance(buffer_size),
            buffer_size as u64,
            program_id,
        ),
        &[
            payer.clone(),
            vending_machine_buffer.clone(),
            system_program.clone(),
        ],
        &[seeds_with_bump],
    )?;

    let mut mutable_vending_machine_buffer = vending_machine_buffer.try_borrow_mut_data()?;

    mutable_vending_machine_buffer[0] = bump_seed as u8;
    mutable_vending_machine_buffer[1..9].clone_from_slice(&price.to_le_bytes());

    Ok(())
}