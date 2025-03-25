#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod instruction;
pub mod state;
pub mod constant;
pub mod error;

pub use instruction::*;
pub use state::*;
pub use constant::*;
pub use error::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod escrow{
  use super::*;

  pub fn create(ctx:Context<CreateGame>, 
                stake:u64, 
                time_control:u64,
                status:u8)->Result<()>{
    _create(ctx,stake,time,status)
  }

  pub fn join(ctx:Context<JoinGame>,
              id:Pubkey,
              status:u8,
              opponent:Pubkey)->Result<()>{
    _join(ctx,id,opponent,status)
  }

  pub fn make(ctx:Context<MakeMove>,
              moves:String,
              status:u8)-> Result<()>{
    _make(ctx,moves,status)
  }

  pub fn claim_victory(ctx:Context<ClaimVictory>,
                       status:u8,
                       last_white_move:i64,
                       last_black_move:i64 ,
                       end_reason:String,
                       winner:Pubkey)->Result<()>{
  _claim_victory(ctx,
                status,
                last_black_move,
                last_white_move,
                end_reason,
                winner)
                       }

  pub fn claim_draw(ctx:Context<ClaimDraw>,
                     status:u8,
                     end_reason:String)->Result<()>{
    _claim_draw(ctx, status, end_reason)
  }

  pub fn abort(ctx:Context<AbortGame>,
               status:u8, 
               end_reason:Option<EndReason3>,
               time_control:u64,
               moves:Vec<String>)-> Result<()>{
    _abort(ctx,status, moves,end_reason)
  }

  pub fn withdraw(ctx:Context<WithdrawFunds>,
                 status:u8,
                 id:Pubkey , 
                 winner:Pubkey)->Result<()>{
    _withdraw(ctx,id,winner,status)
  }
}

