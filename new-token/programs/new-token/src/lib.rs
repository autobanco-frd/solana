use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("BsWpHHhK7tZgu3hJM5EPLK2TMiUQ7kMWn4mwEYx9fJ87");

#[program]
pub mod new_token {
    use super::*;

    pub fn create_token(_ctx: Context<CreateToken>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateToken <'info>{
 
#[account(mut)]
pub payer: Signer<'info>,

    
#[account(
    init,
    payer = payer,
    mint::decimals = 9,
    mint::authority = payer.key(),
    mint::freeze_authority = payer.key()

)]    
pub mint_account: Account<'info,Mint>

pub token_program: Program<'info, Token>,
pub system_program: Program<'info, System>,
pub rent: Sysvar<'info, Rent>


}


