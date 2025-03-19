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

  pub fn create(){}

  pub fn join(){}

  pub fn make(){}

  pub fn claim_victory(){}

  pub fn claim_draw(){}

  pub fn abort(){}

  pub fn withdraw(){}
}

