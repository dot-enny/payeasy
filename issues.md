# PayEasy: Smart Contract Development Roadmap (40 Granular Issues)

> [!IMPORTANT]
> **4-Hour Resolution Rule**: Once an issue is assigned to you, it must be resolved and a Pull Request submitted within **4 hours**. If the deadline passes without a PR, the issue will be unassigned and given to another contributor to keep the project moving at speed.

---

## Stage 1: Infrastructure & Contract Setup (1-5)

### [Issue #1] Setup: Workspace Configuration
- **Points**: 100 (Medium)
- **Description**: Configure the root `Cargo.toml` to manage the `contracts/rent-escrow` contract.
- **Criteria**: `cargo check` at the root recognizes the contract member.

### [Issue #2] Setup: Soroban SDK Optimization
- **Points**: 100 (Medium)
- **Description**: Add build profiles to `contracts/rent-escrow/Cargo.toml` for size optimization (`opt-level = "z"`).
- **Criteria**: Contract compiles to a smaller `.wasm` file.

### [Issue #3] Setup: Define Base Contract Struct
- **Points**: 100 (Medium)
- **Description**: Implement the `RentEscrow` struct with the `#[contract]` attribute.
- **Criteria**: Struct is defined in `lib.rs`.

### [Issue #4] Setup: Implement No-Standard Flag
- **Points**: 100 (Medium)
- **Description**: Ensure the contract environment is properly set to `#![no_std]`.
- **Criteria**: Contract builds for the `wasm32` target.

### [Issue #5] Setup: Test Environment Boilerplate
- **Points**: 100 (Medium)
- **Description**: Create a basic `test.rs` file that can register the contract in a test environment.
- **Criteria**: `cargo test` runs without errors.

---

## Stage 2: Error Handling & Constants (6-10)

### [Issue #6] Errors: Define Initialization Guard Errors
- **Points**: 100 (Medium)
- **Description**: Add `AlreadyInitialized` and `NotInitialized` variants to a `#[contracterror]` enum.
- **Criteria**: Enum compiles in `lib.rs`.

### [Issue #7] Errors: Define Payment Validation Errors
- **Points**: 100 (Medium)
- **Description**: Add `InvalidAmount` and `InsufficientFunding` to the `Error` enum.
- **Criteria**: Errors can be used in `Result` returns.

### [Issue #8] Errors: Define Access Control Errors
- **Points**: 100 (Medium)
- **Description**: Add `Unauthorized` and `Expired` variants to the `Error` enum.
- **Criteria**: Errors are defined and mapped to unique codes.

### [Issue #9] Constants: Network Time Constants
- **Points**: 100 (Medium)
- **Description**: Define `const DAY_IN_LEDGERS: u32 = 17280` for time-based logic.
- **Criteria**: Constant is available for contract use.

### [Issue #10] Constants: Minimum Rent Threshold
- **Points**: 100 (Medium)
- **Description**: Define `const MIN_RENT: i128 = 100` to prevent micro-escrow spam.
- **Criteria**: Constant is enforced in future logic.

---

## Stage 3: Data Structures & Storage Keys (11-15)

### [Issue #11] Storage: Define Base Keys
- **Points**: 100 (Medium)
- **Description**: Add `Landlord` and `RentAmount` keys to the `DataKey` enum.
- **Criteria**: Enum works with storage accessors.

### [Issue #12] Storage: Define Roommate Keys
- **Points**: 100 (Medium)
- **Description**: Add `Shares` and `Contributions` keys to the `DataKey` enum.
- **Criteria**: Keys are ready for `Map` storage.

### [Issue #13] Storage: Define Metadata Keys
- **Points**: 100 (Medium)
- **Description**: Add `Deadline` and `RentToken` keys to the `DataKey` enum.
- **Criteria**: Keys are ready for persistence.

### [Issue #14] Types: Define Roommate State Struct
- **Points**: 100 (Medium)
- **Description**: Create a `RoommateState` struct with `expected` and `paid` fields.
- **Criteria**: Struct is marked with `#[contracttype]`.

### [Issue #15] Types: Define Escrow Status Enum
- **Points**: 100 (Medium)
- **Description**: Create a `Status` enum (Open, Funded, Released, Refunded).
- **Criteria**: Enum is marked with `#[contracttype]`.

---

## Stage 4: Initialization Logic (16-20)

### [Issue #16] Init: `initialize` Signature
- **Points**: 100 (Medium)
- **Description**: Define the public `initialize` function signature with correct arguments.
- **Criteria**: Accepts `landlord`, `total`, `deadline`, and `token_address`.

### [Issue #17] Init: Validation - Zero Address Guard
- **Points**: 100 (Medium)
- **Description**: Add logic to verify the landlord address isn't the contract itself.
- **Criteria**: Reverts if check fails.

### [Issue #18] Init: Validation - Positive Rent Check
- **Points**: 100 (Medium)
- **Description**: Logic to ensure the `total_rent` is greater than `MIN_RENT`.
- **Criteria**: Fails for small or negative amounts.

### [Issue #19] Init: Storage Persistence
- **Points**: 150 (High)
- **Description**: Implement the code to save landlord and amount into persistent storage.
- **Criteria**: State is saved successfully.

### [Issue #20] Init: Test Case - Success Path
- **Points**: 100 (Medium)
- **Description**: Write a test verifying that `initialize` sets the state correctly.
- **Criteria**: Test passes `cargo test`.

---

## Stage 5: Roommate Configuration (21-25)

### [Issue #21] Setup: `add_roommate` Interface
- **Points**: 150 (High)
- **Description**: Implement function for the landlord to add roommate addresses and shares.
- **Criteria**: Only callable by the landlord address.

### [Issue #22] Logic: Share Sum Validation
- **Points**: 150 (High)
- **Description**: Ensure the sum of roommate shares does not exceed `total_rent`.
- **Criteria**: Reverts if math is incorrect.

### [Issue #23] Getter: Landlord Lookup
- **Points**: 100 (Medium)
- **Description**: Implement `fn get_landlord(e: Env) -> Address`.
- **Criteria**: Returns correct address from storage.

### [Issue #24] Getter: Total Amount Lookup
- **Points**: 100 (Medium)
- **Description**: Implement `fn get_total(e: Env) -> i128`.
- **Criteria**: Returns correct amount from storage.

### [Issue #25] Getter: Deadline Lookup
- **Points**: 100 (Medium)
- **Description**: Implement `fn get_deadline(e: Env) -> u64`.
- **Criteria**: Returns correct timestamp from storage.

---

## Stage 6: Contribution Logic (26-30)

### [Issue #26] Deposit: `contribute` Signature
- **Points**: 150 (High)
- **Description**: Define the public `contribute(from: Address, amount: i128)` function.
- **Criteria**: properly marked with `require_auth()`.

### [Issue #27] Validation: Roommate Membership check
- **Points**: 100 (Medium)
- **Description**: Logic to verify the caller is actually a registered roommate.
- **Criteria**: Reverts for unauthorized accounts.

### [Issue #28] Token: Transfer Integration
- **Points**: 200 (High)
- **Description**: Implement the `token::Client` transfer from user to contract.
- **Criteria**: Tokens move successfully on-chain.

### [Issue #29] Logic: Update Paid Balance
- **Points**: 150 (High)
- **Description**: Increment the `paid` field in the roommate's contribution map.
- **Criteria**: State reflects the new deposit.

### [Issue #30] Event: `Contribution` Emission
- **Points**: 100 (Medium)
- **Description**: Emit an event including the roommate and amount deposited.
- **Criteria**: Event appears in transaction logs.

---

## Stage 7: Escrow & Release (31-35)

### [Issue #31] Logic: Calculate Total Funded
- **Points**: 100 (Medium)
- **Description**: Helper to sum all current roommate contributions.
- **Criteria**: Correctly identifies if the goal is met.

### [Issue #32] Release: `release` Status Guard
- **Points**: 100 (Medium)
- **Description**: Ensure `release()` only works if `is_fully_funded` is true.
- **Criteria**: prevents premature payout.

### [Issue #33] Release: Transfer to Landlord
- **Points**: 200 (High)
- **Description**: Logic to move the full contract balance to the landlord.
- **Criteria**: Landlord receives funds.

### [Issue #34] Event: `AgreementReleased`
- **Points**: 100 (Medium)
- **Description**: Publish event when the total rent is paid out.
- **Criteria**: Event shows full funding was achieved.

### [Issue #35] Test: full Flow Scenario
- **Points**: 200 (High)
- **Description**: End-to-end test: init -> contribute x3 -> release.
- **Criteria**: Full cycle completes successfully in one test.

---

## Stage 8: Expiry & Refund (36-40)

### [Issue #36] Refund: `claim_refund` Signature
- **Points**: 150 (High)
- **Description**: Implement function for individual roommates to reclaim deposits.
- **Criteria**: Enforces `require_auth()`.

### [Issue #37] Validation: Deadline Verification
- **Points**: 100 (Medium)
- **Description**: Ensure refunds are only available after the deadline.
- **Criteria**: Reverts if called too early.

### [Issue #38] Logic: Individual Token Refund
- **Points**: 150 (High)
- **Description**: Transfer the roommate's `paid` amount back to them.
- **Criteria**: User receives their money back.

### [Issue #39] State: Zeroing Balances
- **Points**: 100 (Medium)
- **Description**: Set the roommate's `paid` balance to `0` after refund.
- **Criteria**: prevents re-entrancy/double-refunds.

### [Issue #40] Polish: Storage TTL Extension
- **Points**: 150 (High)
- **Description**: implement persistent storage TTL extension so the agreement doesn't expire.
- **Criteria**: Storage state is preserved for the duration of the rent period.
