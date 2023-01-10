use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn crank3(
    ctx: Context<Crank3>,
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    ccb1_mint_bump: u8,
    // ccs0_mint_bump: u8,
    // ccs1_mint_bump: u8,
) -> Result<()> {
    assert_eq!(ctx.accounts.mint_authority.maturity_state, 3);
    let mut rmod: f64 = 2.0 - (ctx.accounts.mint_authority.isum - 1.0) / 2.0;
    if rmod < 1.0 { rmod = 1.0; }
    let x0 = (rmod * ctx.accounts.mint_authority.ccb_amount) as u64;
    let x1 = 2 * (ctx.accounts.mint_authority.ccb_amount as u64) - x0;
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.ccb0_mint_account.to_account_info(),
                to: ctx.accounts.ccb0_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), x0,
    )?;
    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.cc_mint_account.to_account_info(),
                from: ctx.accounts.cc_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), x1,
    )?;
    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.ccb1_mint_account.to_account_info(),
                from: ctx.accounts.ccb1_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), ctx.accounts.mint_authority.ccb_amount as u64,
    )?;
    let pool0 = &mut ctx.accounts.mint_authority;
    pool0.cc0_amount -= x1 as f64;
    pool0.ccb_amount = x0 as f64;
    // pool0.cc1_amount *= 2.0;
    pool0.imod /= 2.0;
    pool0.rmod = rmod;
    pool0.isum = 1.0;
    pool0.maturity_state = 0;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    ccb1_mint_bump: u8,
    // ccs0_mint_bump: u8,
    // ccs1_mint_bump: u8,
)]
pub struct Crank3<'info> {
    #[account(mut,
        seeds = [ b"mint_auth_" ],
        bump = mint_auth_bump
    )]
    pub mint_authority: Account<'info, MintAuth>,

    #[account(mut,
        seeds = [ b"cc_mint_" ],
        bump = cc_mint_bump
    )]
    pub cc_mint_account: Account<'info, token::Mint>,
    #[account(mut,
        seeds = [ b"ccb0_mint_" ],
        bump = ccb0_mint_bump
    )]
    pub ccb0_mint_account: Account<'info, token::Mint>,
    #[account(mut,
        seeds = [ b"ccb1_mint_" ],
        bump = ccb1_mint_bump
    )]
    pub ccb1_mint_account: Account<'info, token::Mint>,
    /*
    #[account(mut,
        seeds = [ b"ccs0_mint_" ],
        bump = ccs0_mint_bump
    )]
    pub ccs0_mint_account: Account<'info, token::Mint>,
    #[account(mut,
        seeds = [ b"ccs1_mint_" ],
        bump = ccs1_mint_bump
    )]
    pub ccs1_mint_account: Account<'info, token::Mint>,
    */

    #[account(mut,
        associated_token::mint = cc_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub cc_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccb0_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccb0_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccb1_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccb1_token_account: Box<Account<'info, token::TokenAccount>>,
    /*
    #[account(mut,
        associated_token::mint = ccs0_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccs0_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccs1_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccs1_token_account: Box<Account<'info, token::TokenAccount>>,
    */
    pub token_program: Program<'info, token::Token>,
}
