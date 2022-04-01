// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

pub use self::charge::GasCharge;
pub(crate) use self::outputs::GasOutputs;
pub use self::price_list::{price_list_by_network_version, PriceList};
use crate::kernel::{ExecutionError, Result};

mod charge;
mod outputs;
mod price_list;

pub const GAS_TO_FUEL_MULTIPLIER: u64 = 1;

pub fn gas_to_fuel(gas: i64) -> u64 {
    return (gas * GAS_TO_FUEL_MULTIPLIER) as u64;
}

pub fn fuel_to_gas(fuel: u64) -> i64 {
    return (fuel / GAS_TO_FUEL_MULTIPLIER) as i64;
}

pub struct GasTracker {
    gas_available: i64,
    gas_used: i64,
}

impl GasTracker {
    pub fn new(gas_available: i64, gas_used: i64) -> Self {
        Self {
            gas_available,
            gas_used,
        }
    }

    /// Safely consumes gas and returns an out of gas error if there is not sufficient
    /// enough gas remaining for charge.
    pub fn charge_gas(&mut self, charge: GasCharge) -> Result<()> {
        let to_use = charge.total();
        match self.gas_used.checked_add(to_use) {
            None => {
                log::trace!("gas overflow: {}", charge.name);
                self.gas_used = self.gas_available;
                Err(ExecutionError::OutOfGas)
            }
            Some(used) => {
                log::trace!("charged {} gas: {}", to_use, charge.name);
                if used > self.gas_available {
                    log::trace!("out of gas: {}", charge.name);
                    self.gas_used = self.gas_available;
                    Err(ExecutionError::OutOfGas)
                } else {
                    self.gas_used = used;
                    Ok(())
                }
            }
        }
    }

    /// Getter for gas available.
    pub fn gas_available(&self) -> i64 {
        self.gas_available
    }

    /// Getter for gas used.
    pub fn gas_used(&self) -> i64 {
        self.gas_used
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_gas_tracker() {
        let mut t = GasTracker::new(20, 10);
        t.charge_gas(GasCharge::new("", 5, 0)).unwrap();
        assert_eq!(t.gas_used(), 15);
        t.charge_gas(GasCharge::new("", 5, 0)).unwrap();
        assert_eq!(t.gas_used(), 20);
        assert!(t.charge_gas(GasCharge::new("", 1, 0)).is_err())
    }
}
