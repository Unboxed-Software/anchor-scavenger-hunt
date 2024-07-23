use anchor_lang::{prelude::*, solana_program::pubkey};

declare_id!("Di2e9PAgcwGHdAJF6d5zxEXTj9KdDm8dWpmc6FcMoK1J");

#[constant]
pub const EVENT_ORGANIZER: Pubkey = pubkey!("fun8eenPrVMJtiQNE7q1iBVDNuY2Lbnc3x8FFgCt43N");

#[program]
pub mod scavenger_hunt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, game_id: Pubkey) -> Result<()> {
        ctx.accounts.user_state.set_inner(UserState {
            user: *ctx.accounts.user.key,
            game_id,
            last_location: Pubkey::default(),
            bump: ctx.bumps.user_state,
        });

        Ok(())
    }

    pub fn check_in(ctx: Context<CheckIn>, _game_id: Pubkey, location: Pubkey) -> Result<()> {
        ctx.accounts.user_state.last_location = location;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(game_id: Pubkey)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [game_id.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + UserState::INIT_SPACE,

    )]
    pub user_state: Account<'info, UserState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(game_id: Pubkey)]
pub struct CheckIn<'info> {
    #[account(
        mut,
        seeds = [game_id.key().as_ref(), user.key().as_ref()],
        bump = user_state.bump,
    )]
    pub user_state: Account<'info, UserState>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = EVENT_ORGANIZER)]
    pub event_organizer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub user: Pubkey,
    pub game_id: Pubkey,
    pub last_location: Pubkey,
    pub bump: u8,
}
