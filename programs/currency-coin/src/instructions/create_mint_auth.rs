use {
    anchor_lang::{
        prelude::*,
    },
};

pub fn create_mint_auth(
    _ctx: Context<CreateMintAuth>,
) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMintAuth<'info> {
    #[account(init,
        payer = payer,
        space = 8 + 56 + 1,
        seeds = [ b"mint_auth_" ], bump
    )]
    pub mint_authority: Account<'info, MintAuth>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MintAuth {
    pub timestamp: i64,

    pub cmod: f64,
    pub rmod: f64,
    pub imod: f64,
    pub isum: f64,
    pub smod: f64,

    pub ima0: f32,
    pub ima1: f32,

        // maturity_state 0 -- steady state
        //     b0 is accumulating interest, s0 is loosing interest
        //     b0 and s0 are trading, b1 and s1 can be redeemed
        // maturity state 1 -- mint b1, burn b0, mint cc, mint s1, burn s0
        // maturity state 2 -- steady state
        //     b1 is accumulating interest, s1 is loosing interest
        //     b1 and s1 are trading, b0 and s0 can be redeemed
        // maturity state 3 -- mint b0, burn b1, mint cc, mint s0, burn s1
    pub maturity_state: u8,
}
