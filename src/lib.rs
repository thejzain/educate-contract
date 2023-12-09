#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use crate::erc20::Erc20;
use crate::erc20::Erc20Params;
use alloc::{borrow, string::String, vec::Vec};
use alloy_primitives::Function;
use stylus_sdk::{alloy_primitives::U256, call, msg, prelude::*};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod erc20;

struct FungusParams;

impl Erc20Params for FungusParams {
    const NAME: &'static str = "Fungus";
    const SYMBOL: &'static str = "FETH";
    const DECIMALS: u8 = 18;
}

sol_storage! {
#[entrypoint]
struct Fungus{
    #[borrow]
    Erc20<FungusParams> erc20;
    }
}

sol_interface! {
    interface IMath {
        function sum(uint256[] values) pure returns (string, uint256);
    }
}

#[external]
#[inherit(Erc20<FungusParams>)]
impl Fungus {
    #[payable]
    pub fn deposit(&mut self) -> Result<(), Vec<u8>> {
        self.erc20.mint(msg::sender(), msg::value());
        Ok(())
    }

    pub fn withdraw(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        self.erc20.burn(msg::sender(), amount)?;

        // send the user their funds
        call::transfer_eth(msg::sender(), amount)
    }

    // sums numbers
    pub fn sum(values: Vec<U256>) -> Result<(String, U256), Vec<u8>> {
        Ok(("sum".into(), values.iter().sum()))
    }

    // calls the sum() method from the interface
    pub fn sum_with_helper(&self, helper: IMath, values: Vec<U256>) -> Result<U256, Vec<u8>> {
        let (text, sum) = helper.sum(self, values)?;
        assert_eq!(&text, "sum");
        Ok(sum)
    }
    
}
