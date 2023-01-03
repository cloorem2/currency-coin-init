use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke_signed,
    },
    anchor_spl::token,
    mpl_token_metadata::instruction as mpl_instruction,
};

pub fn create_mint_auth(
    ctx: Context<CreateMintAuth>,
) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMintAuth<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32,
        seeds = [ b"mint_auth_" ], bump
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

#[account]
pub struct MintAuthorityPda {}
