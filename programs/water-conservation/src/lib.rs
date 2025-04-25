#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("CYgCc27FKtjZBwkDjeZ6VfyiifvbdBv9e9Q8Zn872jAK");

#[program]
pub mod water_conservation {
    use super::*;
}
