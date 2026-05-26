#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    VaultInfo,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultAgreement {
    pub client: Address,
    pub freelancer: Address,
    pub usdc_token: Address,
    pub escrow_amount: i128,
    pub milestone_hash: Symbol,
    pub is_funded: bool,
    pub is_released: bool,
}

#[contract]
pub struct TrabahoVaultContract;

#[contractimpl]
impl TrabahoVaultContract {
    /// Nag-i-initialize ng kasunduan sa pagitan ng foreign client at lokal na freelancer.
    pub fn initialize(
        env: Env,
        client: Address,
        freelancer: Address,
        usdc_token: Address,
        escrow_amount: i128,
        milestone_hash: Symbol,
    ) {
        if env.storage().instance().has(&DataKey::VaultInfo) {
            panic!("Vault escrow agreement already initialized");
        }

        let agreement = VaultAgreement {
            client,
            freelancer,
            usdc_token,
            escrow_amount,
            milestone_hash,
            is_funded: false,
            is_released: false,
        };

        env.storage().instance().set(&DataKey::VaultInfo, &agreement);
    }

    /// Hinahatak ang USDC mula sa wallet ng client patungo sa proteksyon ng contract escrow.
    pub fn fund_vault(env: Env) {
        let mut agreement: VaultAgreement = env.storage().instance().get(&DataKey::VaultInfo).unwrap();
        if agreement.is_funded {
            panic!("Vault is already funded");
        }

        // Pinapatunayan ang cryptographic identity ng nagpopondo
        agreement.client.require_auth();
        let token_client = token::Client::new(&env, &agreement.usdc_token);
        
        token_client.transfer(&agreement.client, &env.current_contract_address(), &agreement.escrow_amount);

        agreement.is_funded = true;
        env.storage().instance().set(&DataKey::VaultInfo, &agreement);
    }

    /// Ipinapadala ang naka-lock na pondo direkta sa wallet ng freelancer kapag aprubado ang trabaho.
    pub fn release_payment(env: Env) {
        let mut agreement: VaultAgreement = env.storage().instance().get(&DataKey::VaultInfo).unwrap();
        if !agreement.is_funded {
            panic!("Cannot release funds from an unfunded vault");
        }
        if agreement.is_released {
            panic!("Payment already disbursed to the freelancer");
        }

        // Tanging ang nagpapasahod na client ang may pahintulot na mag-approve
        agreement.client.require_auth();

        let token_client = token::Client::new(&env, &agreement.usdc_token);
        token_client.transfer(&env.current_contract_address(), &agreement.freelancer, &agreement.escrow_amount);

        agreement.is_released = true;
        env.storage().instance().set(&DataKey::VaultInfo, &agreement);
    }

    /// Kumukuha ng kasalukuyang detalye at on-chain status ng escrow.
    pub fn get_vault_details(env: Env) -> VaultAgreement {
        env.storage().instance().get(&DataKey::VaultInfo).unwrap()
    }
}