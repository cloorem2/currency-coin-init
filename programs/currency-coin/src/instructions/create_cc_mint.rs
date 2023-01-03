use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke_signed,
    },
    anchor_spl::token,
    mpl_token_metadata::instruction as mpl_instruction,
};
use crate::create_mint_auth::MintAuthorityPda;

pub fn create_cc_mint(
    ctx: Context<CreateCcMint>,
    metadata_title: String,
    metadata_symbol: String,
    metadata_uri: String,
    mint_auth_bump: u8,
) -> Result<()> {
    // msg!("Metadata account address: {}", &ctx.accounts.metadata_account.key());
    invoke_signed(
        &mpl_instruction::create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),      // Program ID (the Token Metadata Program)
            ctx.accounts.metadata_account.key(),            // Metadata account
            ctx.accounts.mint_account.key(),                // Mint account
            ctx.accounts.mint_authority.key(),              // Mint authority
            ctx.accounts.payer.key(),                       // Payer
            ctx.accounts.mint_authority.key(),              // Update authority
            metadata_title,                                 // Name
            metadata_symbol,                                // Symbol
            metadata_uri,                                   // URI
            None,                                           // Creators
            0,                                              // Seller fee basis points
            true,                                           // Update authority is signer
            false,                                          // Is mutable
            None,                                           // Collection
            None,                                           // Uses
        ),
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[
            &[
                b"mint_auth_",
                &[mint_auth_bump],
            ]
        ]
    )?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    metadata_title: String,
    metadata_symbol: String,
    metadata_uri: String,
    mint_auth_bump: u8,
)]
pub struct CreateCcMint<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_authority.key(),
        seeds = [ b"cc_mint_" ], bump
    )]
    pub mint_account: Account<'info, token::Mint>,
    #[account(mut,
        seeds = [ b"mint_auth_" ],
        bump = mint_auth_bump
    )]
    pub mint_authority: Account<'info, MintAuthorityPda>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}
