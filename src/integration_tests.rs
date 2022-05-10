// Passport is currently being built on Terra. As part of this, we need to create smart contracts that will be deployed to Terra mainnet.
//
// We want you to build a simple Terra smart contract. Please create a terra smart contract with the following requirements:
// - you should be able to instantiate the contract and set the owner
// - you should support a read query to get the owner of the smart contract
// - you should store the scores for multiple addresses in the smart contract state (ex. {address_1: 10, address_2: 20})
// - you should support an execute message where only the owner of the smart contract can set the score of an address (ex. so the owner, who is address_1, can set the score for address_2)
// - you should support a read query to get the score for a particular address
// - you should write unit tests for all of these scenarios (we should be able to run `cargo test` and all of the unit tests should pass)
//
// Bonus Questions:
// 1. Let's say now we want to break down an address's score by token type, can you update your smart contract to support this?
// Example: address_1 & token Mirror -> score 95 address_1 & token TerraUST -> score 54
//
// 2. Can you deploy the smart contract to the Terra testnet, write tests to set and get address scores, and print out your test results?
//
// Please send us a public github link with the entire project. We should be able to clone the project, change directories, and then run `cargo test` to have the code compile + have the unit tests run

// https://academy.terra.money/enrollments
// https://docs.terra.money/Tutorials/Smart-contracts/Build-Terra-dApp.html
