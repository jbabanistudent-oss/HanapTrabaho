# TrabahoVault

Isang secure milestone escrow contract na binuo sa Soroban upang maprotektahan ang sahod at mapababa ang bayarin sa pagpapadala ng mga freelance virtual assistant sa Pilipinas.

## Problem & Solution
Ang mga freelance virtual assistant ay nawawalan ng malaking porsyento ng kanilang kita dahil sa mataas na transaction fees at matagal na proseso ng bank wire kapag sumesweldo mula sa ibang bansa. Sa TrabahoVault, direktang magdedeposito ang kliyente ng USDC sa smart contract escrow, at awtomatiko itong mapupunta sa freelancer kapag natapos ang milestone, nang mabilis at walang kalat-kalat na charges.

## Timeline
* **Day 1:** Pagsulat ng core escrow holding contract at pagpapatakbo ng kumpletong unit tests gamit ang Soroban SDK.
* **Day 2:** Pagbuo ng interactive web client interface para sa mga foreign businesses at lokal na manggagawa.
* **Day 3:** Deployment ng WASM binary file sa Testnet network at paghanda ng pangwakas na presentasyon.

## Contract ID:
CBQFBIOLJG5X6TEGYIQWG77A22SMVIIOQEKQZFC46EHEZJEVK5NV4MUK

## Stellar Features Used
* Soroban Smart Contracts (Immutable escrow lifecycle control at security state verification)
* USDC Stablecoin Standard (Mabilis at mababang-gastos na cross-border value settlement)

## Vision and Purpose
Layunin naming alisin ang mataas na kaltas at burukrasya sa mga pandaigdigang micro-contract, upang mapanatiling buo at secure ang kita ng mga remote workers sa bansa.

## Prerequisites
* Rust `v1.75.0+`
* Stellar CLI `v25.0.0+`
* Target `wasm32-unknown-unknown`

## How to Build
Mula sa iyong root workspace directory, patakbuhin ang command na ito upang i-compile ang contract:

```bash
stellar contract build