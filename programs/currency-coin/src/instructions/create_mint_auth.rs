use {
    anchor_lang::{
        prelude::*,
    },
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
}

#[account]
pub struct MintAuthorityPda {}
