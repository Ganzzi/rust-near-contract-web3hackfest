[Ignite Hack](https://devfolio.co/projects/ignite-hack-bc90)
# rust-near-contract-web3hackfest

Welcome to the README for Rust smart contract project. This document will guide you through the installation process and deployment steps for your project.

# Installation
Before you can deploy your Rust smart contract, you need to set up the necessary tools and dependencies. Follow these steps to get started:

1. Install Rust: Make sure you have Rust installed on your system. You can install Rust using rustup, a toolchain manager for Rust programming language. Follow the instructions on the website to install rustup and Rust.

2. Install Near CLI: Install the NEAR Command Line Interface (CLI) by running the following command in your terminal:
```
npm install -g near-cli
```
3. Clone the Repository: Clone your smart contract repository to your local machine using Git:

```
git clone https://github.com/Ganzzi/rust-near-contract-web3hackfest.git
cd rust-near-contract-web3hackfest
```
4. Build the Smart Contract: Build your Rust smart contract using the following command:

```
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```

# Deployment
Now that you have built your smart contract, it's time to deploy it to the NEAR blockchain. Follow these steps to deploy your smart contract:

1. Create an Account: Create an account on the NEAR blockchain to deploy your smart contract. Run the following command:

```
near create-account <subname>.<account-name> --masterAccount <account-name> --initialBalance 5
```

2. Deploy the Smart Contract: Deploy your compiled smart contract to the NEAR blockchain using the following command:

```
near deploy --accountId=<sub-acccount> --wasmFile=target/wasm32-unknown-unknown/release/web3hackfest.wasm
```

3. Initialize the Smart Contract: After deploying the smart contract, you need to initialize it using the following command:

```
near call <sub-acccount> init --accountId=<account-name>
```

# Usage
Your Rust smart contract is now deployed and ready to be used on the NEAR blockchain. You can interact with your smart contract using the NEAR CLI or by integrating it into your dApp.

# Additional Resources
If you need more information about Rust smart contract development on the NEAR blockchain, refer to the official NEAR documentation and resources:

-  NEAR Documentation
-  NEAR Rust Smart Contracts
-  NEAR CLI GitHub Repository
  
For any issues, questions, or discussions, consider joining the NEAR community on Discord or their official forum.

Happy coding and building on the NEAR blockchain!
