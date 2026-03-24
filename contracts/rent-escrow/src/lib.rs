#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Landlord,
    TotalAmount,
    Roommates,     // Map<Address, i128> - roommate address to required share
    Contributions, // Map<Address, i128> - roommate address to paid share
    Deadline,
    IsReleased,
}

#[contract]
pub struct RentEscrow;

#[contractimpl]
impl RentEscrow {
    /// Initialize the rent escrow agreement
    pub fn initialize(
        env: Env,
        landlord: Address,
        total_amount: i128,
        roommate_shares: Map<Address, i128>,
        deadline: u64,
    ) {
        // TODO: Implement initialization logic
        // Verify that the sum of roommate_shares equals total_amount
    }

    /// Roommates call this to contribute their share of the rent
    pub fn contribute(env: Env, from: Address, amount: i128) {
        // TODO: Implement contribution logic
        // 1. Verify 'from' is a valid roommate
        // 2. Transfer tokens from 'from' to the contract
        // 3. Update contributions map
    }

    /// Release the total rent to the landlord if fully funded
    pub fn release(env: Env) {
        // TODO: Implement release logic
        // 1. Verify total_amount is reached
        // 2. Transfer everything to the landlord
        // 3. Mark as released
    }

    /// Refund roommates if the deadline has passed and rent is not fully funded
    pub fn refund(env: Env, to: Address) {
        // TODO: Implement refund logic
        // 1. Verify deadline has passed
        // 2. Verify total_amount was NOT reached
        // 3. Refund 'to' their exact contribution
    }
}

mod test;
