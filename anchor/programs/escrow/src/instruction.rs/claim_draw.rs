use anchor_lang::prelude::*;

pub mod claim_draw{
    use super::*;

   pub fn claim_draw(ctx:Context<ClaimDraw>,
        status:u8,
        end_reason:Option<EndReason>)->Result<()>{
let claim_draw = &mut ctx.accounts.game;

game.status = Status::Completed;
game.winner = None;
game.end_reason = Some(reason);

Ok(())
 }
}

#[derive(Accounts)]
pub struct ClaimDraw<'info>{
     #[account(mut)]
    pub host:Signer<'info>,
    #[account(mut)]
    pub game_account: Account<'info , Game>,
    #[account(
        mut,
        seeds=[b"game",game_pda.key.as_ref()],
        bump
    )]
    pub game_pda:Account<'info,GamePda>,
    pub system_program: Program<'info, System>
}