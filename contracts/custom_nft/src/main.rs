#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::{
    ckb_types::{
        bytes::Bytes, 
        packed::*, 
        prelude::*
    },
    high_level::load_script,
};
use core::result::Result;
use nft_base::*;
use nft_base::error::*;
use nft_base::helper::*;
use nft_base::extension::OnlyOwner;

pub struct Custom;

impl Custom {
    pub fn handle_creation(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }

    pub fn handle_update(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }

    pub fn handle_destroying(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }
}

define_script! { ComposedScript(Base, OnlyOwner, Custom) { } }

pub fn main() -> Result<(), Error> {
    let nft_type = load_script()?;
    ComposedScript::validate_nft_args(&nft_type)?;
    match nft_base::parse_nft_action(&nft_type)? {
        Action::Create => ComposedScript::handle_creation(&nft_type),
        Action::Update => ComposedScript::handle_update(&nft_type),
        Action::Destroy => ComposedScript::handle_destroying(&nft_type),
    }
}

use ckb_std::default_alloc;
ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry(_argc: u64, _argv: *const *const u8) -> i8 {
    match main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}
