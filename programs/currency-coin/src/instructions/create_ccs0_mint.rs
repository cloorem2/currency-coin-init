use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn create_ccs0_mint(
    ctx: Context<CreateCcs0Mint>,
    mint_auth_bump: u8,
) -> Result<()> {
    token::set_authority(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::SetAuthority {
                account_or_mint: ctx.accounts.mint_account.to_account_info(),
                current_authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ),
        token::spl_token::instruction::AuthorityType::MintTokens,
        Some(ctx.accounts.mint_authority.key())
    )?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(mint_auth_bump: u8)]
pub struct CreateCcs0Mint<'info> {
    #[account(mut,
        seeds = [ b"mint_auth_" ],
        bump = mint_auth_bump
    )]
    pub mint_authority: Account<'info, MintAuth>,
    #[account(init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
        seeds = [ b"ccs0_mint_" ], bump
    )]
    pub mint_account: Account<'info, token::Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}
