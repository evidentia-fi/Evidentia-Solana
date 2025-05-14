# Evidentia Bond-Backed Stablecoin Platform (Solana)

## Overview
Evidentia is a decentralized finance (DeFi) platform on Solana that enables users to tokenize bonds, use them as collateral to borrow stablecoins, and earn yield by staking stablecoins. This implementation adapts the EVM-based Evidentia protocol to Solana’s high-performance blockchain, leveraging Solana Program Library (SPL) tokens and the Anchor framework for robust smart contract development.

## Core Workflow
1. **Tokenize Bonds**: Bonds are represented as semi-fungible SPL tokens via the BondNFT program, embedding metadata (value, coupon rate, timestamps, ISIN).
2. **Stake Bonds as Collateral**: Users deposit BondNFTs into the NFTStakingAndBorrowing program.
3. **Borrow Stablecoins**: Users borrow StableBondCoins (SBC), a fungible SPL token, against staked BondNFTs. Loans accrue interest over time.
4. **Earn Yield via Staking**: SBC holders stake tokens in the StableCoinsStaking program to earn rewards from interest revenue.

## Key Features
- **Tokenized Bonds (SPL Token)**: The BondNFT program manages semi-fungible tokens with metadata, analogous to ERC-1155.
- **Collateralized Debt Position (CDP)**: The NFTStakingAndBorrowing program enables borrowing SBC against BondNFT collateral.
- **Native Stablecoin (SPL Token)**: StableBondCoins (SBC) is a mintable/burnable token used for borrowing and staking.
- **NFT Staking & Borrowing**: Users lock BondNFTs to borrow SBC, subject to collateralization ratios and interest rates.
- **Stablecoin Staking & Yield**: The StableCoinsStaking program allows SBC staking for yield generation.
- **Interest Accrual & Reward Distribution**: Manages loan interest and distributes rewards to stakers.
- **Modular Architecture**: Separate programs for BondNFT, StableBondCoins, NFTStakingAndBorrowing, and StableCoinsStaking.
- **Access Control & Security**: Employs Program Derived Address (PDA)-based ownership, signer checks, and reentrancy protections.

## Architecture
### Design Considerations
- **Solana's Account Model**: Programs are stateless; data is stored in accounts owned by programs.
- **SPL Tokens**: Used for BondNFT (semi-fungible) and StableBondCoins (fungible).
- **Upgradeability**: Supports program upgrades via `bpf_loader_upgradeable`.
- **Security**: PDA ownership, signer checks, and state-based reentrancy protection.
- **Development**: Built with Rust and Anchor for simplified serialization and security.

### Core Programs

#### BondNFT Program
- **Purpose**: Manages tokenized bonds as semi-fungible SPL tokens.
- **Key Accounts**:
  - `BondMetadataAccount`: Stores bond metadata (value, coupon rate, timestamps, ISIN).
  - `MintAccount`: SPL token mint for each bond ID.
  - `UserTokenAccount`: Tracks user bond token balances.
- **Instructions**:
  - `mint(to: Pubkey, id: u64, amount: u64, metadata: BondMetadata)`: Mints bond tokens (admin-only).
  - `burn(id: u64, amount: u64)`: Burns bond tokens.
  - `set_metadata(id: u64, metadata: BondMetadata)`: Updates bond metadata (admin-only).
  - `get_metadata(id: u64)`: Retrieves bond metadata.
  - `transfer(from: Pubkey, to: Pubkey, id: u64, amount: u64)`: Transfers bond tokens.
- **Test Version (BondNFTV2)**: Includes `initialize_v2` and `new_feature` for upgrade testing.

#### StableBondCoins Program
- **Purpose**: Manages StableBondCoins (SBC) as a fungible SPL token.
- **Key Accounts**:
  - `MintAccount`: SPL token mint for SBC.
  - `AuthorityAccount`: PDA controlling minting/burning.
  - `UserTokenAccount`: Tracks user SBC balances.
- **Instructions**:
  - `mint(to: Pubkey, amount: u64)`: Mints SBC (minter authority only).
  - `burn(from: Pubkey, amount: u64)`: Burns SBC.
  - `transfer(to: Pubkey, amount: u64)`: Transfers SBC.
  - `approve(spender: Pubkey, amount: u64)`: Sets delegate for SPL token approvals.
- **Test Version (StableBondCoinsV2)**: Includes `initialize_v2` and `new_feature` for upgrade testing.

#### NFTStakingAndBorrowing Program
- **Purpose**: Handles staking of BondNFTs and borrowing SBC.
- **Key Accounts**:
  - `UserStakeAccount`: PDA storing staked BondNFTs (bond ID, amount).
  - `LoanAccount`: PDA tracking loans (borrowed amount, interest, timestamp).
  - `RewardPoolAccount`: Stores accumulated interest for rewards.
- **Instructions**:
  - `stake_nft(nft_mint: Pubkey, token_id: u64, amount: u64)`: Stakes BondNFTs.
  - `unstake_nft(nft_mint: Pubkey, token_id: u64, amount: u64)`: Unstakes BondNFTs.
  - `borrow(amount: u64)`: Borrows SBC against collateral.
  - `repay(amount: u64)`: Repays loan with interest.
  - `claim_rewards()`: Claims interest-based rewards.
- **Test Version (NFTStakingAndBorrowingV2)**: Includes `initialize_v2` and `new_feature` for upgrade testing.

#### StableCoinsStaking Program
- **Purpose**: Enables SBC staking for yield generation.
- **Key Accounts**:
  - `StakeAccount`: PDA storing user’s staked SBC and reward data.
  - `RewardPoolAccount`: Tracks rewards for distribution.
- **Instructions**:
  - `stake(amount: u64)`: Stakes SBC.
  - `withdraw(amount: u64)`: Withdraws staked SBC.
  - `claim_rewards()`: Claims accrued rewards.
  - `earned(staker: Pubkey)`: Calculates unclaimed rewards.
- **Test Version (StableCoinsStakingV2)**: Includes `initialize_v2` and `new_feature` for upgrade testing.

### Data Structures
```rust
#[account]
pub struct BondMetadata {
    pub value: u64,
    pub coupon_value: u64,
    pub issue_timestamp: i64,
    pub expiration_timestamp: i64,
    pub isin: String,
}

#[account]
pub struct UserStake {
    pub nft_mint: Pubkey,
    pub token_id: u64,
    pub amount: u64,
    pub timestamp: i64,
}

#[account]
pub struct Loan {
    pub borrower: Pubkey,
    pub amount: u64,
    pub interest_rate: u64,
    pub start_timestamp: i64,
}
