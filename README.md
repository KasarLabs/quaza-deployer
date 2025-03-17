# Quaza Deployment Tool

A Rust-based deployment tool for Cairo contracts on StarkNet, specifically designed for deploying and managing L3 network contracts.

## Overview

This tool automates the deployment process for a suite of Cairo contracts to StarkNet, including:

- Account contracts (OpenZeppelin implementation)
- Token contracts (ERC20)
- Upgrade mechanisms (EIC - External Initialization Contract)
- Counter contract (for testing purposes)
- StarkNet core contract

## Prerequisites

- Rust 1.70+ 
- [Scarb](https://github.com/software-mansion/scarb) 2.8.4 for Cairo contracts
- Access to a StarkNet RPC node

## Configuration

1. Copy `exemple.env` to `.env` and configure your environment:

```
# RPC Endpoints
RPC_URL=https://your-rpc-endpoint.com/
RPC_ADMIN_URL=https://your-admin-rpc-endpoint.com/
RPC_STARKNET_URL=https://starknet-sepolia.g.alchemy.com/starknet/version/rpc/v0_7/your_api_key

# Account Configuration
DEPLOYER_SECRET_KEY=0xYOUR_SECRET_KEY
STARKNET_ACCOUNT_ADDRESS=0xYOUR_ACCOUNT_ADDRESS
```

## Key Features

### Contract Deployment
- Automatic declaration and deployment of Cairo contracts
- Support for UDC (Universal Deployer Contract)
- Fee-free deployments for testnets

### Token Management
- STRK token deployment with upgrade capability
- Custom token deployment with minting capabilities
- ERC20 standard compliance

### Account Management
- OpenZeppelin Account contract support
- Account deployment and management

### Upgrade Mechanisms
- External Initialization Contract (EIC) pattern
- Storage migration support

## Usage

### Building the Project

```bash
# Build the Rust project
cargo build --release

# Build Cairo contracts (if they haven't been compiled)
cd cairo && scarb build
```

### Running the Deployment

```bash
cargo run --release
```

This will:
1. Declare all necessary contracts
2. Deploy the UDC (Universal Deployer Contract)
3. Deploy an OpenZeppelin account
4. Deploy STRK and QUAZA tokens
5. Set up the token contracts with proper permissions
6. Mint initial token supplies
7. Deploy a sample counter contract
8. Optionally deploy the StarkNet core contract

## Development

### Adding New Contracts

1. Add your Cairo contract to `cairo/src/`
2. Add it to the exports in `cairo/src/lib.cairo`
3. Compile using `scarb build`
4. Add deployment logic to the Rust code in `src/main.rs`

### Customizing Deployment

The main deployment sequence is in `src/main.rs`. You can modify this file to change the deployment order or add additional contracts.
