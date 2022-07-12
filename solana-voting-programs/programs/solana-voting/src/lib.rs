use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::system_program;
use borsh::{BorshDeserialize, BorshSerialize};
use std::iter::repeat;

declare_id!("85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ");

// https://docs.rs/anchor-lang/latest/anchor_lang/error/enum.ErrorCode.html
#[error_code]
// #[error]
pub enum ErrorCode {
    #[msg("Need to be more than one option")]
    TooLittleOptions,
}

// // #[derive(Error, Debug, Copy, Clone)]
// pub enum ErrorCode {
//     #[msg("The provided topic should be 50 characters long maximum.")]
//     TopicTooLong,
//     #[msg("The provided content should be 280 characters long maximum.")]
//     ContentTooLong,
// }

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        msg!("{}", e);
        ProgramError::Custom(e as u32)
    }
}

#[program]
pub mod solana_voting {
    use super::*;
        pub fn create_votetopic(ctx: Context<InitializeVoteTopic>, topic: String, options: String) -> ProgramResult {
        msg!("create_votetopic to this topic now: {:#?} with options: {:#?}", topic, options);
        let splitOptions: Vec<&str>  = options.split(',').collect();
        msg!("options are: {:#?}, with length: {:#?}", splitOptions, splitOptions.len());
        if splitOptions.len() <= 1 {
            return Err(ErrorCode::TooLittleOptions.into())
        }

        let votetopic: &mut Account<Votetopic> = &mut ctx.accounts.votetopic;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        votetopic.author = *author.key;
        votetopic.timestamp = clock.unix_timestamp;
        votetopic.topic = topic;
        votetopic.options = options;
        // votetopic.voters = vec![*author.key];
        votetopic.voters = vec![(*author.key).to_string()];
        // votetopic.voters = vec![];
        msg!("votetopic with 0 voters: {:#?}", votetopic.voters);

        Ok(())
    }


    pub fn register_voter(ctx: Context<RegisterVoter>, random: String) -> ProgramResult {
        msg!("entered create_votetopic methodddd");

        let votetopic: &mut Account<Votetopic> = &mut ctx.accounts.votetopic;
        votetopic.voters.push(random);
        msg!("votetopic with 0 voters: {:#?}", votetopic.voters);

        Ok(())
    }

}


#[derive(Accounts)]
pub struct InitializeVoteTopic<'info> {
    #[account(init, payer = author, space = Votetopic::LEN)]
    pub votetopic: Account<'info, Votetopic>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RegisterVoter<'info> {
    #[account(mut)]
    pub votetopic: Account<'info, Votetopic>,
    // #[account(mut)]
    // // pub voter: AccountInfo<'info>,
    // pub voter: Signer<'info>,
    // pub voter: Pubkey,

}


#[account]
pub struct Votetopic {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub options: String,
    pub voters: Vec<String>,
    // pub voters: Vec<Pubkey>,
    // pub voters: Vec<Voter>,
}


// // https://gist.github.com/FrankC01/b03937c5e8c74753eb552ca1e15ba8f8
// // #[account]
// // #[derive(Debug)]
// // make it public unless its anonymous voting:
// #[derive(Debug, BorshDeserialize, BorshSerialize, Default, Clone)]
// pub struct Voter {
//     account: Pubkey,
//     vote: u32,
// }
// // ----didnt work since IdlError: Type not found: {"type":{"defined":"Voter"}} kept appearing during testing:

// impl Voter {
//     fn new(account: Pubkey, vote: u32) -> Self {
//         Voter { account: account, vote: vote, }
//     }
// }

// 2. Add some useful constants for sizing propeties.
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.

impl Votetopic {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Admin for votetopic.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Voting Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // length of voters.
}


