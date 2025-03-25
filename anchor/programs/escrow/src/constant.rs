use anchor_lang::prelude::*;

pub fn game_pda(game_id:Pubkey)->(Pubkey,u8){
    Pubkey::find_program_address(&[b"game",game_id.as_ref()],&ID)
}

#[account]
#[derive(InitSpace)]
pub struct Game{
    pub game_id:Pubkey,
    pub host:Pubkey,
    pub opponent:Option<Pubkey>,
    pub winner: Option<Pubkey>,
    pub stake:u64,
    pub end_reason:Option<EndReason>,
    #[max_len(32)]
    pub moves:Vec<MoveData>,
    pub time_control:u64,
    pub status:u8,
    pub player_turn:Pubkey,
    pub last_black_move : i64,
    pub last_white_move: i64,
}

#[account]
#[derive(Accounts)]
pub struct MoveData{
    pub host: Pubkey,
    pub from: (u8,u8),
    pub to: (u8,u8),
}