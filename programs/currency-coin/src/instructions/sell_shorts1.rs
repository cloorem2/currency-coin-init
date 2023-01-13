use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn sell_shorts1(
    ctx: Context<SellShorts1>,
    // the most s0 seller is sending, we leave the dust behind
    amount: u64,
    mint_auth_bump: u8,
    _cc_mint_bump: u8,
    _ccb1_mint_bump: u8,
    _ccs0_mint_bump: u8,
) -> Result<()> {
    assert_eq!(ctx.accounts.mint_authority.maturity_state, 2);
    // let r0: u64 = ctx.accounts.mint_authority.cc1_amount;
    // let r1: u64 = ctx.accounts.mint_authority.ccs_amount;
    // let k: u64 = r0 * r1;
    // let r11: u64 = r1 + amount;
    // let cc_amount: u64 = (r0 * r11 - k) / r11;
    // let r00: u64 = r0 - cc_amount;
    // let ccs_amount: u64 = (k - r1 * r00) / r00;

    let mut cc_to_owner = amount as f64;
    cc_to_owner += ctx.accounts.mint_authority.ccs_amount;
    cc_to_owner *= ctx.accounts.mint_authority.cc1_amount;
    cc_to_owner -= ctx.accounts.mint_authority.ccs_amount
      * ctx.accounts.mint_authority.cc1_amount;
    cc_to_owner /= ctx.accounts.mint_authority.ccs_amount + amount as f64;
    cc_to_owner = cc_to_owner.floor();
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.cc_account.to_account_info(),
                to: ctx.accounts.owner_cc_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), cc_to_owner as u64,
    )?;

    let mut s0_from_owner = ctx.accounts.mint_authority.cc1_amount
      - cc_to_owner;
    s0_from_owner *= ctx.accounts.mint_authority.ccs_amount;
    s0_from_owner = ctx.accounts.mint_authority.ccs_amount
      * ctx.accounts.mint_authority.cc1_amount - s0_from_owner;
    s0_from_owner /= ctx.accounts.mint_authority.cc1_amount - cc_to_owner;
    s0_from_owner = s0_from_owner.ceil();
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_ccs0_account.to_account_info(),
                to: ctx.accounts.ccs0_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ), s0_from_owner as u64,
    )?;

    // b1_to_burn = cc_to_owner
    // let mut b1_to_burn = cc_to_owner;
    // b1_to_burn *= ctx.accounts.mint_authority.ccb_amount;
    // b1_to_burn /= ctx.accounts.mint_authority.cc1_amount;
    // b1_to_burn = b1_to_burn.floor();
    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.ccb1_mint_account.to_account_info(),
                from: ctx.accounts.ccb1_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), cc_to_owner as u64,
    )?;

    let mut cc_to_mint = ctx.accounts.mint_authority.ccb_amount - cc_to_owner;
    cc_to_mint *= ctx.accounts.mint_authority.cc0_amount;
    cc_to_mint = ctx.accounts.mint_authority.ccb_amount
      * ctx.accounts.mint_authority.cc0_amount - cc_to_mint;
    cc_to_mint /= ctx.accounts.mint_authority.ccb_amount - cc_to_owner;
    cc_to_mint = cc_to_mint.floor();
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.cc_mint_account.to_account_info(),
                to: ctx.accounts.cc_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), cc_to_mint as u64,
    )?;

    ctx.accounts.mint_authority.cc0_amount += cc_to_mint;
    ctx.accounts.mint_authority.ccb_amount -= cc_to_owner;
    ctx.accounts.mint_authority.cc1_amount -= cc_to_owner;
    ctx.accounts.mint_authority.ccs_amount += s0_from_owner;
    Ok(())
}


#[derive(Accounts)]
#[instruction(
    amount: u64,
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb1_mint_bump: u8,
    ccs0_mint_bump: u8,
)]
pub struct SellShorts1<'info> {
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
        seeds = [ b"ccb1_mint_" ],
        bump = ccb1_mint_bump
    )]
    pub ccb1_mint_account: Account<'info, token::Mint>,
    #[account(mut,
        seeds = [ b"ccs0_mint_" ],
        bump = ccs0_mint_bump
    )]
    pub ccs0_mint_account: Account<'info, token::Mint>,

    #[account(mut,
        associated_token::mint = cc_mint_account,
        associated_token::authority = owner,
    )]
    pub owner_cc_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccs0_mint_account,
        associated_token::authority = owner,
    )]
    pub owner_ccs0_account: Box<Account<'info, token::TokenAccount>>,

    #[account(mut,
        associated_token::mint = cc_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub cc_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccb1_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccb1_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccs0_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccs0_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    // pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    // pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
