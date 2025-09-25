# CapsuleUnlock

A Solana-based time-locked cause capsule app for the 100xSchool hackathon.

## Description

CapsuleUnlock is a decentralized application that allows users to create time-locked capsules containing messages or causes that can only be unlocked after a specified time period.

## Project Structure

- `programs/` - Solana programs (smart contracts)
- `frontend/` - Next.js frontend application
- `tests/` - Test files
- `migrations/` - Anchor migrations

## Setup

1. Install dependencies:
   - Node.js
   - Solana CLI
   - Anchor CLI
   - Rust

2. Install Solana CLI:
   - Download from https://docs.solana.com/cli/install-solana-cli-tools
   - Or run: `sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"`

3. Install Anchor CLI:
   - `cargo install anchor-cli`

4. Configure Solana for devnet:
   - `solana config set --url https://api.devnet.solana.com`

5. Install Phantom wallet: https://phantom.app/

## Development

1. Clone the repository
2. `cd frontend && npm install`
3. `anchor build` in the root
4. `anchor deploy` to deploy programs
5. `npm run dev` in frontend

## License

MIT