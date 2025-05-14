# Evidentia Solana Program

This repository contains the Solana implementation of the Evidentia smart contracts, originally designed for the Ethereum Virtual Machine (EVM). The program is written in Rust using the [Anchor framework](https://www.anchor-lang.com/) and runs on the Solana blockchain, leveraging its high throughput, low fees, and parallel execution capabilities.

## Overview

Evidentia on Solana provides a decentralized platform for managing data records, user permissions, or tokenized operations (e.g., storing verified data, controlling access, or issuing tokens). Unlike EVM-based contracts, Solana programs are stateless, with data stored in separate accounts. This implementation uses the Anchor framework to simplify development, program-derived addresses (PDAs) for state management, and Solana’s Sealevel runtime for parallel transaction processing.

## Architecture

The Evidentia Solana program adapts the EVM-based smart contract logic to Solana’s stateless program model. Below is the architecture, highlighting key components, account structures, and differences from the EVM implementation.

### Key Components

1. **Programs (Smart Contracts)**:
   - A single Solana program, written in Rust with Anchor, handles core logic, including:
     - Initializing data accounts (e.g., for records or configurations).
     - Updating records (e.g., storing verified data).
     - Managing permissions (e.g., granting user access).
     - Token operations (if applicable, using Solana’s SPL Token program).
   - **Program ID**: A unique public key identifies the program on Solana.
   - **Anchor Framework**: Simplifies program development with declarative macros for serialization and account validation.

2. **Accounts**:
   - **Program-Derived Addresses (PDAs)**:
     - **Config Account**: Stores global settings (e.g., admin public key or system parameters).
     - **Record Account**: Holds individual data records (e.g., content, owner, verification status).
     - **User Account**: Tracks user-specific data (e.g., permissions or token balances).
   - **Executable Account**: The program itself, containing compiled BPF bytecode.
   - **Token Accounts**: Managed by the SPL Token program for token-related operations (if applicable).

3. **Instructions**:
   - Instructions are entry points to the program, similar to EVM contract functions:
     - `initialize`: Creates PDAs and sets initial state (e.g., admin settings).
     - `update_record`: Modifies data in a record account (requires authorization).
     - `grant_access`: Updates user permissions (admin-only).
     - `transfer_token`: Handles token transfers (if applicable).
   - Each instruction declares accounts to read/write, enabling parallel execution via Solana’s Sealevel runtime.

4. **Client Interaction**:
   - Clients (e.g., web apps or scripts) interact with the program using Solana’s JSON RPC API or libraries like `@solana/web3.js`.
   - Transactions include instruction data, account lists, and signatures from relevant keypairs (e.g., user or admin).

5. **Parallel Execution**:
   - Solana’s Sealevel runtime processes transactions in parallel when they access non-overlapping accounts. The program specifies account access (read/write) upfront, maximizing throughput.
   - Example: Two `update_record` calls for different PDAs can execute concurrently.

### Data Flow

1. **Initialization**: A client calls `initialize` to create PDAs for configuration and data storage, setting initial state (e.g., admin key).
2. **Data Operations**: Users call `update_record` to store or modify data in a PDA, with the program validating permissions.
3. **Permission Management**: Admins call `grant_access` to update user permissions in a PDA.
4. **Token Operations** (if applicable): Users call `transfer_token` to interact with SPL Token accounts.

### Security Considerations

- **Account Ownership**: PDAs are owned by the program, ensuring only the program can modify them.
- **Signature Verification**: Instructions validate signers (e.g., user or admin keypairs).
- **Rent Exemption**: Accounts maintain a minimum balance (lamports) to persist.
- **Anchor Constraints**: Use `@account` and `@constraint` macros to enforce validation (e.g., correct PDA derivation).

### Differences from EVM

- **State Management**:
  - EVM: State is stored in the contract’s storage (e.g., mappings or arrays).
  - Solana: State is stored in separate accounts (PDAs), and programs are stateless.
- **Execution**:
  - EVM: Sequential execution, gas-based cost.
  - Solana: Parallel execution via Sealevel, cost measured in compute units.
- **Programming**:
  - EVM: Solidity/Vyper, compiled to EVM bytecode.
  - Solana: Rust with Anchor, compiled to BPF bytecode.
- **Account Model**:
  - EVM: Externally Owned Accounts (EOAs) and Contract Accounts (CAs).
  - Solana: All accounts can store data; executable (programs) vs. non-executable (data).

### Example Program Structure

```rust
use anchor_lang::prelude::*;

declare_id!("EvidentiaProgramID...");

#[program]
mod evidentia {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, config_data: ConfigData) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.admin = *ctx.accounts.signer.key;
        config.data = config_data;
        Ok(())
    }

    pub fn update_record(ctx: Context<UpdateRecord>, record_id: u64, data: Vec<u8>) -> Result<()> {
        let record = &mut ctx.accounts.record;
        require!(record.owner == *ctx.accounts.signer.key, ErrorCode::Unauthorized);
        record.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + Config::LEN)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub data: ConfigData,
}

#[derive(Accounts)]
pub struct UpdateRecord<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
pub struct Record {
    pub owner: Pubkey,
    pub data: Vec<u8>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access")]
    Unauthorized,
}
