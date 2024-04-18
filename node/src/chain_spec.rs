use foucoco_standalone_runtime::{GenesisConfig, Runtime, SS58Prefix, WASM_BINARY};
use sc_service::ChainType;
use sp_core::{crypto::Ss58Codec, sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	FixedPointNumber, FixedU128,
};

use runtime_common::{AccountId, Balance, Signature, UNIT};

use spacewalk_primitives::{oracle::Key, Asset, CurrencyId, CurrencyId::XCM, VaultCurrencyPair};

pub const INITIAL_ISSUANCE: Balance = 200_000_000 * UNIT;

pub const INITIAL_ISSUANCE_PER_SIGNATORY: Balance = 200 * UNIT;

pub const ENDOWED_ACCOUNTS_BALANCE: Balance = 1_000_000 * UNIT;

pub const OFF_CHAIN_WORKER_ADDRESS: &str = "6m69vWMouLarYCbJGJisVaDDpfNGETkD5hsDWf2T7osW4Cn1";

pub const TOKEN_DECIMALS: u32 = 12;

pub const INITIAL_SUDO_SIGNATORIES: [&str; 5] = [
	"6mSy3qQKgAez9zpqY1JSnYW7d1njMNX93P4mkkQvsmPXmehB",
	"6mrdgs7NsHwceSPQRcXCagYzZiB4hoMBGmpMPLA4rS4BGyo7",
	"6jBUR27UemaZBF2aYrEbMuN3u76aANEpA3uxLrQcWP8jNDtf",
	"6hcDDb1nV6zrqfiB7dgQ5DbzuLkPmxkvSZ5LSA9kcE3gxNs8",
	"6k4NQX2KepBkeexrWVNabnWG9GZxvQTYi4ytHHCNwPhLZMnE",
];

// For Mainnet USDC issued by the testnet issuer
pub const MAINNET_USDC_CURRENCY_ID: CurrencyId = CurrencyId::Stellar(Asset::AlphaNum4 {
	code: *b"USDC",
	issuer: [
		59, 153, 17, 56, 14, 254, 152, 139, 160, 168, 144, 14, 177, 207, 228, 79, 54, 111, 125,
		190, 148, 107, 237, 7, 114, 64, 247, 246, 36, 223, 21, 197,
	],
});

// For Mainnet BRL issued by the testnet issuer
pub const MAINNET_BRL_CURRENCY_ID: CurrencyId = CurrencyId::Stellar(Asset::AlphaNum4 {
	code: *b"BRL\0",
	issuer: [
		234, 172, 104, 212, 208, 227, 123, 76, 36, 194, 83, 105, 22, 232, 48, 115, 95, 3, 45, 13,
		107, 42, 28, 143, 202, 59, 197, 162, 94, 8, 62, 58,
	],
});

// For Mainnet TZS issued by the testnet issuer
pub const MAINNET_TZS_CURRENCY_ID: CurrencyId = CurrencyId::Stellar(Asset::AlphaNum4 {
	code: *b"TZS\0",
	issuer: [
		52, 201, 75, 42, 75, 169, 232, 181, 123, 34, 84, 125, 203, 179, 15, 68, 60, 76, 176, 45,
		163, 130, 154, 137, 170, 27, 212, 120, 14, 68, 102, 186,
	],
});

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	<TPublic::Pair as Pair>::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config() -> ChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "AMPE".into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());

	let mut signatories: Vec<_> = INITIAL_SUDO_SIGNATORIES
		.iter()
		.map(|ss58| AccountId::from_ss58check(ss58).unwrap())
		.collect();
	signatories.sort();

	let sudo_account = get_account_id_from_seed::<sr25519::Public>("AltoParaíso"); //pallet_multisig::Pallet::<Runtime>::multi_account_id(&signatories[..], 3);

	let offchain_worker_price_feeder = AccountId::from_ss58check(OFF_CHAIN_WORKER_ADDRESS).unwrap();

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				signatories.clone(),
				vec![sudo_account.clone(), offchain_worker_price_feeder.clone()],
				// Sudo account
				sudo_account.clone(),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				false,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("rpc"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	)
}

pub fn local_testnet_config() -> ChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "AMPE".into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());

	let mut signatories: Vec<_> = INITIAL_SUDO_SIGNATORIES
		.iter()
		.map(|ss58| AccountId::from_ss58check(ss58).unwrap())
		.collect();
	signatories.sort();

	let sudo_account = pallet_multisig::Pallet::<Runtime>::multi_account_id(&signatories[..], 3);

	let offchain_worker_price_feeder = AccountId::from_ss58check(OFF_CHAIN_WORKER_ADDRESS).unwrap();

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				signatories.clone(),
				vec![sudo_account.clone(), offchain_worker_price_feeder.clone()],
				// Sudo account
				sudo_account.clone(),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				false,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("rpc"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	signatories: Vec<AccountId>,
	authorized_oracles: Vec<AccountId>,
	sudo_account: AccountId,
	endowed_accounts: Vec<AccountId>,
	start_shutdown: bool,
) -> GenesisConfig {
	fn get_vault_currency_pair(
		collateral: CurrencyId,
		wrapped: CurrencyId,
	) -> VaultCurrencyPair<CurrencyId> {
		VaultCurrencyPair { collateral, wrapped }
	}

	let mut balances: Vec<_> = signatories
		.iter()
		.cloned()
		.map(|k| (k, INITIAL_ISSUANCE_PER_SIGNATORY))
		.chain(endowed_accounts.iter().cloned().map(|k| (k, ENDOWED_ACCOUNTS_BALANCE)))
		.collect();

	balances.push((
		sudo_account.clone(),
		INITIAL_ISSUANCE
			.saturating_sub(
				INITIAL_ISSUANCE_PER_SIGNATORY
					.saturating_mul(signatories.len().try_into().unwrap()),
			)
			.saturating_sub(
				ENDOWED_ACCOUNTS_BALANCE.saturating_mul(endowed_accounts.len().try_into().unwrap()),
			),
	));

	let token_balances = balances
		.iter()
		.flat_map(|k| vec![(k.0.clone(), XCM(0), u128::pow(10, 20))])
		.collect();

	GenesisConfig {
		asset_registry: Default::default(),
		system: foucoco_standalone_runtime::SystemConfig {
			code: WASM_BINARY.expect("WASM binary was not build, please build it!").to_vec(),
		},
		balances: foucoco_standalone_runtime::BalancesConfig { balances },
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		council: foucoco_standalone_runtime::CouncilConfig {
			members: signatories.clone(),
			..Default::default()
		},
		democracy: Default::default(),
		sudo: foucoco_standalone_runtime::SudoConfig { key: Some(sudo_account.clone()) },
		technical_committee: foucoco_standalone_runtime::TechnicalCommitteeConfig {
			members: signatories.clone(),
			..Default::default()
		},
		parachain_info: foucoco_standalone_runtime::ParachainInfoConfig { parachain_id: 2124.into() },
		tokens: foucoco_standalone_runtime::TokensConfig {
			// Configure the initial token supply for the native currency and USDC asset
			balances: token_balances,
		},
		issue: foucoco_standalone_runtime::IssueConfig {
			issue_period: foucoco_standalone_runtime::DAYS,
			issue_minimum_transfer_amount: 1_000_000_000,
			limit_volume_amount: None,
			limit_volume_currency_id: XCM(0),
			current_volume_amount: 0u32.into(),
			interval_length: (60u32 * 60 * 24).into(),
			last_interval_index: 0u32.into(),
		},
		redeem: foucoco_standalone_runtime::RedeemConfig {
			redeem_period: foucoco_standalone_runtime::DAYS,
			redeem_minimum_transfer_amount: 1_000_000_000,
			limit_volume_amount: None,
			limit_volume_currency_id: XCM(0),
			current_volume_amount: 0u32.into(),
			interval_length: (60u32 * 60 * 24).into(),
			last_interval_index: 0u32.into(),
		},
		replace: foucoco_standalone_runtime::ReplaceConfig {
			replace_period: foucoco_standalone_runtime::DAYS,
			replace_minimum_transfer_amount: 1_000_000_000,
		},
		security: foucoco_standalone_runtime::SecurityConfig {
			initial_status: if start_shutdown {
				foucoco_standalone_runtime::StatusCode::Shutdown
			} else {
				foucoco_standalone_runtime::StatusCode::Error
			},
		},
		oracle: foucoco_standalone_runtime::OracleConfig {
			max_delay: 604_800_000, // 7 days
			oracle_keys: vec![
				Key::ExchangeRate(CurrencyId::XCM(0)),
				Key::ExchangeRate(CurrencyId::Native),
				Key::ExchangeRate(CurrencyId::Stellar(Asset::StellarNative)),
				Key::ExchangeRate(MAINNET_USDC_CURRENCY_ID),
				Key::ExchangeRate(MAINNET_BRL_CURRENCY_ID),
				Key::ExchangeRate(MAINNET_TZS_CURRENCY_ID),
			],
		},
		vault_registry: foucoco_standalone_runtime::VaultRegistryConfig {
			minimum_collateral_vault: vec![(XCM(0), 3_000_000_000_000)],
			punishment_delay: foucoco_standalone_runtime::DAYS * 2,
			secure_collateral_threshold: vec![
				(
					get_vault_currency_pair(XCM(0), MAINNET_USDC_CURRENCY_ID),
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
				(
					get_vault_currency_pair(XCM(0), MAINNET_BRL_CURRENCY_ID),
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
				(
					get_vault_currency_pair(XCM(0), MAINNET_TZS_CURRENCY_ID),
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
				(
					get_vault_currency_pair(XCM(0), CurrencyId::Stellar(Asset::StellarNative)),
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
			],
			/* 140% */
			premium_redeem_threshold: vec![
				(
					get_vault_currency_pair(XCM(0), MAINNET_USDC_CURRENCY_ID),
					FixedU128::checked_from_rational(140, 100).unwrap(),
				),
				(
					get_vault_currency_pair(XCM(0), MAINNET_BRL_CURRENCY_ID),
					FixedU128::checked_from_rational(140, 100).unwrap(),
				),
				(
					get_vault_currency_pair(XCM(0), MAINNET_TZS_CURRENCY_ID),
					FixedU128::checked_from_rational(140, 100).unwrap(),
				),
				(
					get_vault_currency_pair(XCM(0), CurrencyId::Stellar(Asset::StellarNative)),
					FixedU128::checked_from_rational(140, 100).unwrap(),
				),
			],
			/* 125% */
			liquidation_collateral_threshold: vec![
				(
					get_vault_currency_pair(XCM(0), MAINNET_USDC_CURRENCY_ID),
					FixedU128::checked_from_rational(125, 100).unwrap(),
				),
				(
					get_vault_currency_pair(XCM(0), MAINNET_BRL_CURRENCY_ID),
					FixedU128::checked_from_rational(125, 100).unwrap(),
				),
				(
					get_vault_currency_pair(XCM(0), MAINNET_TZS_CURRENCY_ID),
					FixedU128::checked_from_rational(125, 100).unwrap(),
				),
				(
					get_vault_currency_pair(XCM(0), CurrencyId::Stellar(Asset::StellarNative)),
					FixedU128::checked_from_rational(125, 100).unwrap(),
				),
			],
			system_collateral_ceiling: vec![
				(
					get_vault_currency_pair(XCM(0), MAINNET_USDC_CURRENCY_ID),
					50 * 10u128.pow(TOKEN_DECIMALS),
				),
				(
					get_vault_currency_pair(XCM(0), MAINNET_BRL_CURRENCY_ID),
					25 * 10u128.pow(TOKEN_DECIMALS),
				),
				(
					get_vault_currency_pair(XCM(0), MAINNET_TZS_CURRENCY_ID),
					25 * 10u128.pow(TOKEN_DECIMALS),
				),
				(
					get_vault_currency_pair(XCM(0), CurrencyId::Stellar(Asset::StellarNative)),
					50 * 10u128.pow(TOKEN_DECIMALS),
				),
			],
		},
		stellar_relay: foucoco_standalone_runtime::StellarRelayConfig::default(),
		fee: foucoco_standalone_runtime::FeeConfig {
			issue_fee: FixedU128::checked_from_rational(1, 1000).unwrap(), // 0.1%
			issue_griefing_collateral: FixedU128::checked_from_rational(5, 1000).unwrap(), // 0.5%
			redeem_fee: FixedU128::checked_from_rational(1, 1000).unwrap(), // 0.1%
			premium_redeem_fee: FixedU128::checked_from_rational(5, 100).unwrap(), // 5%
			punishment_fee: FixedU128::checked_from_rational(1, 10).unwrap(), // 10%
			replace_griefing_collateral: FixedU128::checked_from_rational(1, 10).unwrap(), // 10%
		},
		nomination: foucoco_standalone_runtime::NominationConfig { is_nomination_enabled: false },
		dia_oracle_module: foucoco_standalone_runtime::DiaOracleModuleConfig {
			authorized_accounts: authorized_oracles,
			supported_currencies: vec![
				foucoco_standalone_runtime::AssetId::new(b"Kusama".to_vec(), b"KSM".to_vec()),
				foucoco_standalone_runtime::AssetId::new(b"Stellar".to_vec(), b"XLM".to_vec()),
				foucoco_standalone_runtime::AssetId::new(b"FIAT".to_vec(), b"BRL-USD".to_vec()),
				foucoco_standalone_runtime::AssetId::new(b"FIAT".to_vec(), b"USD-USD".to_vec()),
				foucoco_standalone_runtime::AssetId::new(b"FIAT".to_vec(), b"TZS-USD".to_vec()),
				foucoco_standalone_runtime::AssetId::new(b"FIAT".to_vec(), b"MXN-USD".to_vec()),
			],
			batching_api: b"https://dia-00.pendulumchain.tech/currencies".to_vec(),
			coin_infos_map: vec![],
		},
		token_allowance: foucoco_standalone_runtime::TokenAllowanceConfig {
			allowed_currencies: vec![
				CurrencyId::Native,
				CurrencyId::XCM(0),
				CurrencyId::XCM(1),
				CurrencyId::XCM(2),
				CurrencyId::XCM(3),
				CurrencyId::XCM(4),
				CurrencyId::XCM(5),
				CurrencyId::XCM(6),
				CurrencyId::XCM(7),
				CurrencyId::XCM(8),
				CurrencyId::XCM(9),
				CurrencyId::XCM(10),
			],
		},
	}
}
