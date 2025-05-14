# Evidentia Solana Program

This repository contains the Evidentia program, a decentralized application built for the Solana blockchain. Written in Rust using the [Anchor framework](https://www.anchor-lang.com/), the program leverages Solana’s high throughput, low transaction fees, and parallel execution capabilities to deliver a scalable and efficient platform.

## Overview

The Evidentia program enables users to manage data records, permissions, or tokenized operations, such as storing verified data, controlling access, or transferring tokens. Solana’s stateless program model stores data in separate accounts, and the Sealevel runtime processes transactions in parallel for optimal performance. The program uses the Anchor framework for streamlined development, program-derived addresses (PDAs) for state management, and optionally integrates with Solana’s SPL Token program for token functionality.

## Architecture

The Evidentia program is designed within Solana’s account-based, stateless program model. Below is the architecture, detailing the program’s components, account structures, and data flow.

### Key Components

1. **Program**:
   - A single Solana program, written in Rust with Anchor, handles core logic, including:
     - Initializing accounts for configuration or data storage.
     - Updating records (e.g., storing or modifying data).
     - Managing user permissions (e.g., granting access).
     - Token operations (if applicable, using the SPL Token program).
   - **Program ID**: A unique public key identifying the program on the Solana blockchain.
   - **Anchor Framework**: Provides declarative macros to simplify serialization, account validation, and instruction handling.

2. **Accounts**:
   - **Program-Derived Addresses (PDAs)**:
     - **Config Account**: Stores global settings, such as the admin public key or system parameters.
     - **Record Account**: Holds individual data records, including content, owner, or metadata.
     - **User Account**: Tracks user-specific data, such as permissions or token balances.
   - **Executable Account**: The program itself, containing compiled Berkeley Packet Filter (BPF) bytecode.
   - **Token Accounts**: Managed by the SPL Token program for token-related operations (if applicable).

3. **Instructions**:
   - Instructions serve as entry points to the program, defining its functionality:
     - `initialize`: Creates PDAs and sets initial state (e.g., admin settings).
     - `update_record`: Modifies data in a record account, with authorization checks.
     - `grant_access`: Updates user permissions, restricted to admins.
     - `transfer_token`: Facilitates token transfers (if applicable).
   - Each instruction specifies accounts to read or write, enabling parallel execution via Solana’s Sealevel runtime.

4. **Client Interaction**:
   - Clients (e.g., web apps or scripts) interact with the program using Solana’s JSON RPC API or libraries like `@solana/web3.js`.
   - Transactions include:
     - Instruction data (e.g., function name and parameters).
     - A list of accounts to access.
     - Signatures from relevant keypairs (e.g., user or admin).

5. **Parallel Execution**:
   - Solana’s Sealevel runtime processes transactions concurrently when they access non-overlapping accounts. The program declares account access (read/write) in each instruction, allowing the runtime to optimize transaction scheduling.
   - Example: Two `update_record` instructions targeting different PDAs can execute simultaneously.

### Data Flow

1. **Initialization**:
   - A client calls `initialize` to create PDAs for configuration and data storage, setting initial state (e.g., admin public key).
2. **Data Operations**:
   - Users call `update_record` to store or update data in a PDA, with the program enforcing permission checks.
3. **Permission Management**:
   - Admins call `grant_access` to modify user permissions in a PDA.
4. **Token Operations** (if applicable):
   - Users call `transfer_token` to manage token transfers via SPL Token accounts.

### Security Considerations

- **Account Ownership**: PDAs are owned by the program, ensuring only the program can modify their data.
- **Signature Verification**: Instructions validate signers (e.g., user or admin keypairs) to enforce access control.
- **Rent Exemption**: Accounts maintain a minimum balance of lamports to remain active on the blockchain.
- **Anchor Constraints**: Use `@account` and `@constraint` macros to enforce validation, such as correct PDA derivation.

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
