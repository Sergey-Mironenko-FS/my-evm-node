use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig, EVMConfig,
	SystemConfig, WASM_BINARY, GenesisAccount, EthereumConfig,
};
use sc_service::ChainType;
//use hex_literal::hex;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, H160, U256, ByteArray};
use sp_core::crypto::Ss58Codec;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};
use hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

pub const FIRST_ACCOUNT_AURA: &str = "0xa08dcd654b785edbacf3de8a81022ec1d79ee4e9e226eaf8c3e0177370380256";
pub const FIRST_ACCOUNT_GRANDPA: &str = "0x19d86e38c784910ab80935d6688811858058cc8ab53cd85dcd0b051b6f2c0e09";
pub const SECOND_ACCOUNT_AURA: &str = "0x7e8fb760df4284750830deaef431e8a6374771a30f88ffd5072ea637bd32780b";
pub const SECOND_ACCOUNT_GRANDPA: &str = "0x9f00f75e786d0d781259ef67ecd673747be94df8e6cad30d1df7ed0040559c83";

pub fn get_ss58_from_key<TPublic: Public>(hex_key: &str) -> Result<String, &'static str> {
    // Decode the hex string and handle potential errors
    let decoded = hex::decode(&hex_key[2..]).map_err(|_| "Invalid hex")?;
    
    // Create the public key from the decoded bytes
    let public_key = sr25519::Public::from_slice(&decoded)
        .map_err(|_| "Failed to create public key")?;

    // Convert public key to SS58 address
    let ss58_address = public_key.to_ss58check(); // Use the correct network prefix

    Ok(ss58_address)
}

/// Generate an account ID from a key string.
pub fn get_account_id_from_key<TPublic: Public>(key: &str) -> Result<AccountId, &'static str> {
    // Get the SS58 address from the key
    let ss58_address = get_ss58_from_key::<TPublic>(key)?;

    // Convert the SS58 address to AccountId
    let account_id = AccountId::from_str(&ss58_address)
        .map_err(|_| "Failed to convert SS58 address to AccountId")?;

    Ok(account_id)
}

/// Generate an Aura and Grandpa authority keys from a keys string.
pub fn authority_keys_from_key(aura_key: &str, grandpa_key: &str) -> (AuraId, GrandpaId) {
    let aura_public = get_ss58_from_key::<AuraId>(aura_key)
        .expect("Invalid Aura key"); // Получаем SS58 строку

    let grandpa_public = get_ss58_from_key::<GrandpaId>(grandpa_key)
        .expect("Invalid Grandpa key"); // Получаем SS58 строку

    // Преобразуем SS58 строки в соответствующие Public
    let aura_id = AuraId::from_ss58check(&aura_public)
        .expect("Failed to convert Aura SS58 to Public");
    let grandpa_id = GrandpaId::from_ss58check(&grandpa_public)
        .expect("Failed to convert Grandpa SS58 to Public");

    (aura_id, grandpa_id)
}

//pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
//	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
//}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let first_keys = authority_keys_from_key(FIRST_ACCOUNT_AURA, FIRST_ACCOUNT_GRANDPA);
	let root_key = get_account_id_from_key::<sr25519::Public>(FIRST_ACCOUNT_AURA)?;
    let first_id = get_account_id_from_key::<sr25519::Public>(FIRST_ACCOUNT_AURA)?;
    let second_id = get_account_id_from_key::<sr25519::Public>(SECOND_ACCOUNT_AURA)?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				vec![first_keys.clone()],
				root_key.clone(),
				vec![first_id.clone(), second_id.clone()],
				true,
			)
		},
		// Bootnodes
		vec![
			"/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEYhMgA4mfNcG1fcmyduTpL7nTENqD9KVds5NB6uZ3fWh"
            .parse()
            .unwrap()
		],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let first_keys = authority_keys_from_key(FIRST_ACCOUNT_AURA, FIRST_ACCOUNT_GRANDPA);
	let root_key = get_account_id_from_key::<sr25519::Public>(FIRST_ACCOUNT_AURA)?;
    let first_id = get_account_id_from_key::<sr25519::Public>(FIRST_ACCOUNT_AURA)?;
    let second_id = get_account_id_from_key::<sr25519::Public>(SECOND_ACCOUNT_AURA)?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				vec![first_keys.clone()],
				root_key.clone(),
				vec![first_id.clone(), second_id.clone()],
				true,
			)
		},
		// Bootnodes
		vec![
			"/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEYhMgA4mfNcG1fcmyduTpL7nTENqD9KVds5NB6uZ3fWh"
            .parse()
            .unwrap()
		],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1000000000000u128)).collect::<Vec<_>>(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		evm: EVMConfig {
			accounts: {		
				let mut accounts = BTreeMap::new();

				accounts.insert(
					//Пример первого адреса 0xc22D1398dBD629A80aE75b7320c0d850e847D3b0
					H160::from_str("c22D1398dBD629A80aE75b7320c0d850e847D3b0")
						.expect("internal H160 is valid; qed"),
					GenesisAccount {
						balance: U256::from_str("0xDE0B6B3A7640000")//  1 ETH
							.expect("internal U256 is valid; qed"),
						code: vec![],
						nonce: U256::zero(),
						storage: BTreeMap::new(),
					},
				);

				accounts.insert(
					// //Пример второго адреса 0xB30B34CEB15DeC3C330D4A0BdBf14CB835220F73
					H160::from_str("B30B34CEB15DeC3C330D4A0BdBf14CB835220F73")
						.expect("internal H160 is valid; qed"),
					GenesisAccount {
						balance: U256::from_str("0x56BC75E2D63100000") // 100 ETH
							.expect("internal U256 is valid; qed"),
						code: vec![],
						nonce: U256::zero(),
						storage: BTreeMap::new(),
					},
				);

				accounts
			}
		},
		ethereum: EthereumConfig {},
		base_fee: Default::default(),
	}
}
