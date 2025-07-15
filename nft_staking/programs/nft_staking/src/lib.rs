pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5gAVPfjCFd5MT9Yzd8C6et8mxRuUGwmyw9vPRbZZmnrY");

#[program]
pub mod nft_staking {
    use super::*;
}
