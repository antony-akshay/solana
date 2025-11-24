#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

use anchor_spl::metadata::{
    create_master_edition_v3,
    mpl_token_metadata::types::{CollectionDetails, Creator, DataV2},
    set_and_verify_sized_collection_item, sign_metadata, CreateMasterEditionV3,
    SetAndVerifySizedCollectionItem, SignMetadata,
};

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[constant]
pub const SYMBOL: &str = "TLT";

#[program]
pub mod counter {
    use super::*;

    pub fn initialize_event(
        ctx: Context<InitializeEvent>,
        name: String,
        description: String,
        url: String,
        attentance_code: [u8; 32],
        start_time: i64,
        end_time: i64,
        total_attentees: u32,
        collection_mint: Pubkey,
    ) -> Result<()> {


        if start_time <= end_time {
            return Err(ErrorCode::InvalidEventTime.into());
        }

        *ctx.accounts.event_account = Event {
            creator: *ctx.accounts.payer.key,
            name: name,
            description: description,
            url: url,
            attentance_code: attentance_code,
            start_time: start_time,
            end_time: end_time,
            total_attentees: total_attentees,
            registered_attentees: 0,
            collection_mint: collection_mint,
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection_mint".as_ref(),
            ctx.accounts.event_account.name.as_bytes(),
            &[ctx.bumps.collection_mint],
        ]];

        msg!("creating mint account...");

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    to: ctx.accounts.collection_token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;

        msg!("creating metadata account");

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer_seeds,
            ),
            DataV2 {
                name: ctx.accounts.event_account.name.clone(),
                symbol: SYMBOL.to_string(),
                uri: ctx.accounts.event_account.url.clone(),
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: ctx.accounts.collection_mint.key(),
                    verified: false,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            true,
            true,
            Some(CollectionDetails::V1 { size: 0 }),
        )?;

        msg!("creating master edition account");

        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.master_edition.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    metadata: ctx.accounts.metadata.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer_seeds,
            ),
            Some(0),
        )?;

        msg!("verifying the collection...");

        sign_metadata(CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            SignMetadata {
                creator: ctx.accounts.collection_mint.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
            },
            signer_seeds,
        ))?;

        Ok(())
    }

    pub fn edit_event(
        ctx: Context<EditEvent>,
        _name: String,
        attentance_code: [u8; 32],
        start_time: i64,
        end_time: i64,
        total_attentees: u32,
    ) -> Result<()> {
        let event_account = &mut ctx.accounts.event_account;
        event_account.attentance_code = attentance_code;
        event_account.start_time = start_time;
        event_account.end_time = end_time;
        event_account.total_attentees = total_attentees;
        Ok(())
    }

    pub fn close_event(ctx: Context<CloseEvent>) -> Result<()> {
        msg!("closing account: {:?}", ctx.accounts.event_account.key());
        Ok(())
    }

    pub fn register_event(ctx: Context<RegisterEvent>) -> Result<()> {
        // let clock = Clock::get()?;

        // if clock.slot > ctx.accounts.event_account.start_time as u64
        //     && clock.slot < ctx.accounts.event_account.end_time as u64
        // {
        //     return Err(ErrorCode::RegistrationNotOpenYet.into());
        // }

        if ctx.accounts.event_account.registered_attentees
            == ctx.accounts.event_account.total_attentees
        {
            return Err(ErrorCode::RegistrationCompleted.into());
        }

        *ctx.accounts.registration_account = EventRegistration {
            event: ctx.accounts.event_account.key(),
            attentee: ctx.accounts.attentee.key(),
            registered: true,
            attented: false,
            attentence_nft_minted: false,
        };

        let event_account: &mut Account<'_, Event> = &mut ctx.accounts.event_account;
        event_account.registered_attentees = event_account
            .registered_attentees
            .checked_add(1)
            .ok_or(ErrorCode::OverflowError)?;
        Ok(())
    }

    pub fn cancel_registration(ctx: Context<CancelRegistration>) -> Result<()> {
        let event_account = &mut ctx.accounts.event_account;
        event_account.registered_attentees = event_account
            .registered_attentees
            .checked_sub(1)
            .ok_or(ErrorCode::OverflowError)?;
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNft>, attentance_code: [u8; 32]) -> Result<()> {
        let clock = Clock::get()?;

        if attentance_code != ctx.accounts.event_account.attentance_code {
            return Err(ErrorCode::InvalidAttentanceCode.into());
        }

        if ctx.accounts.registration_account.attentence_nft_minted {
            return Err(ErrorCode::NftAlreadyMinted.into());
        }

        if clock.unix_timestamp < ctx.accounts.event_account.start_time as i64
            || clock.unix_timestamp > ctx.accounts.event_account.end_time as i64
        {
            return Err(ErrorCode::NotMinitingTime.into());
        }

        ctx.accounts.registration_account.attentence_nft_minted = true;

        let nft_name = ctx.accounts.event_account.name.to_owned()
            + ctx
                .accounts
                .event_account
                .registered_attentees
                .to_string()
                .as_str();
        let nft_uri = ctx.accounts.event_account.url.to_owned();

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection_mint".as_ref(),
            ctx.accounts.event_account.name.as_bytes(),
            &[ctx.bumps.collection_mint],
        ]];

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;

        msg!("creating metadata account");

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.child_nft_metadata.to_account_info(),
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.attentee.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer_seeds,
            ),
            DataV2 {
                name: nft_name,
                symbol: SYMBOL.to_string(),
                uri: nft_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;

        msg!("creating master edition account");

        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.child_nft_master_edition.to_account_info(),
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.attentee.to_account_info(),
                    metadata: ctx.accounts.child_nft_metadata.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer_seeds,
            ),
            Some(0),
        )?;

        msg!("Verifying collection");
        set_and_verify_sized_collection_item(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                SetAndVerifySizedCollectionItem {
                    metadata: ctx.accounts.child_nft_metadata.to_account_info(),
                    collection_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.attentee.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    collection_mint: ctx.accounts.collection_mint.to_account_info(),
                    collection_metadata: ctx.accounts.metadata.to_account_info(),
                    collection_master_edition: ctx.accounts.master_edition.to_account_info(),
                },
                signer_seeds,
            ),
            None,
        )?;

        ctx.accounts.registration_account.attentence_nft_minted = true;
        ctx.accounts.registration_account.attented = true;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct InitializeEvent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [b"event",payer.key().as_ref(),name.as_bytes()],
        space = ANCHOR_DISCRIMINATOR_SIZE + Event::INIT_SPACE,
        payer=payer,
        bump
    )]
    pub event_account: Account<'info, Event>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint,
        seeds = [b"collection_mint".as_ref(),name.as_bytes()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=payer,
        token::mint = collection_mint,
        token::authority = collection_token_account,
        seeds = [b"collection_associated_token".as_ref(),name.as_bytes()],
        bump
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK:this account is checked by metadata smart contract
    #[account(
        mut,
        seeds=[
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK:this account is checked by metadata smart contract
    #[account(
        mut,
        seeds=[
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub master_edition: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct EditEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"event",creator.key().as_ref(),name.as_bytes()],
        bump,
        has_one=creator
    )]
    pub event_account: Account<'info, Event>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseEvent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        close = payer,
        seeds = [b"event", event_account.creator.as_ref(), event_account.name.as_bytes()],
        bump
    )]
    pub event_account: Account<'info, Event>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterEvent<'info> {
    #[account(mut)]
    pub attentee: Signer<'info>,

    #[account(
        mut,
        seeds = [b"event", event_account.creator.as_ref(), event_account.name.as_bytes()],
        bump
    )]
    pub event_account: Account<'info, Event>,

    #[account(
        init,
        payer= attentee,
        space = ANCHOR_DISCRIMINATOR_SIZE + EventRegistration::INIT_SPACE,
        seeds=[b"attentee",event_account.key().as_ref(),attentee.key().as_ref()],
        bump,
    )]
    pub registration_account: Account<'info, EventRegistration>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelRegistration<'info> {
    #[account(mut)]
    pub attentee: Signer<'info>,

    #[account(
        mut,
        seeds = [b"event", event_account.creator.as_ref(), event_account.name.as_bytes()],
        bump
    )]
    pub event_account: Account<'info, Event>,

    #[account(
        mut,
        close = attentee,
        seeds=[b"attentee",event_account.key().as_ref(),attentee.key().as_ref()],
        bump,
    )]
    pub registration_account: Account<'info, EventRegistration>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub attentee: Signer<'info>,

    #[account(
        mut,
        seeds = [b"event", event_account.creator.as_ref(), event_account.name.as_bytes()],
        bump
    )]
    pub event_account: Account<'info, Event>,

    #[account(
        mut,
        seeds=[b"attentee",event_account.key().as_ref(),attentee.key().as_ref()],
        bump,
        has_one = attentee,
    )]
    pub registration_account: Account<'info, EventRegistration>,

    #[account(
        mut,
        seeds = [b"collection_mint".as_ref(),event_account.name.as_bytes()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = attentee,
        seeds = [
            b"nft_mint".as_ref(),
            event_account.key().as_ref(),
            attentee.key().as_ref()
        ],
        bump,
        mint::decimals = 0,
        mint::authority=collection_mint,
        mint::token_program=token_program,
        mint::freeze_authority=collection_mint
    )]
    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds=[
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    ///CHECK:this account is checked by metadata smart contract
    pub child_nft_metadata: UncheckedAccount<'info>,

    ///CHECK:this account is checked by metadata smart contract
    #[account(
        mut,
        seeds=[
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub child_nft_master_edition: UncheckedAccount<'info>,

    /// CHECK:this account is checked by metadata smart contract
    #[account(
        mut,
        seeds=[
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK:this account is checked by metadata smart contract
    #[account(
        mut,
        seeds=[
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub master_edition: UncheckedAccount<'info>,

    #[account(
        init,
        payer=attentee,
        associated_token::mint = nft_mint,
        associated_token::authority=attentee,
        associated_token::token_program=token_program
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(InitSpace)]
#[account]
pub struct Event {
    pub creator: Pubkey,
    #[max_len(32)]
    pub name: String,
    #[max_len(64)]
    pub description: String,
    #[max_len(64)]
    pub url: String,
    pub attentance_code: [u8; 32],
    pub start_time: i64,
    pub end_time: i64,
    pub total_attentees: u32,
    pub registered_attentees: u32,
    pub collection_mint: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct EventRegistration {
    pub event: Pubkey,
    pub attentee: Pubkey,
    pub registered: bool,
    pub attented: bool,
    pub attentence_nft_minted: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("registration not open yet")]
    RegistrationNotOpenYet,
    #[msg("registration completed")]
    RegistrationCompleted,
    #[msg("invalid attentance code")]
    InvalidAttentanceCode,
    #[msg("minting not reached")]
    NotMinitingTime,
    #[msg("already minted")]
    NftAlreadyMinted,
    #[msg("overflow error")]
    OverflowError,
    #[msg("invalid event start or end time")]
    InvalidEventTime,
}
