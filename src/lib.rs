pub mod config;
use alloy::primitives::U256;
use rust_decimal::Decimal;

pub fn to_wei(value: Decimal) -> U256 {
    let mantissa = value.mantissa();
    let base = 10_usize.pow(18 - value.scale());
    U256::from(mantissa) * U256::from(base)
}

pub fn to_ether(value: U256) -> Decimal {
    Decimal::from_i128_with_scale(value.to(), 18)
}
