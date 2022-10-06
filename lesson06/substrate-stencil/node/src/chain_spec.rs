use hex_literal::hex;
use node_primitives::*;
use node_template_runtime::{
	constants::currency::*, opaque::SessionKeys, wasm_binary_unwrap, BabeConfig, BalancesConfig,
	CouncilConfig, DemocracyConfig, ElectionsConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig,
	MaxNominations, NominationPoolsConfig, SessionConfig, StakerStatus, StakingConfig, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig, BABE_GENESIS_EPOCH_CONFIG,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

fn session_keys(babe: BabeId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online }
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
			)
		},
		// Bootnodes
		vec![],
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
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
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
			)
		},
		// Bootnodes
		vec![],
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

pub fn xiankun_staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Substrate Xiankun",
		"xiankun_staging",
		ChainType::Live,
		xiankun_staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn xiankun_staging_network_config_genesis() -> GenesisConfig {
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			// 5C5uRq6eC8Xu7eQ9TDXLbqCfpn2AWWxQBhF9F55wu5GTAYDw
			hex!["00ea330a63b346ff9145fba8a737d1578abd8ffb898f51e6761dc41b2a137663"].into(),
			// 5CvqXhLYrXdNgBEuLfEcmrfKthxow7JLT4ZQ2DJcx9L1zVFn
			hex!["263c19766a66f9ca1d5f3463a2c50a23db57e5b02e80d50bbc7b68b98daeaa04"].into(),
			// 5GBbMQfaLAnsLzyXcxb8SDG9QVBh8uytRyMZ8X1DW81gi4yn
			hex!["b631c16f6d7e351e73a5ba5bf932dc2631805e98164738333c1fa49b889f0551"]
				.unchecked_into(),
			// 5FSfFhU9PLjuhGTK7wD6nfngYH8fhCd9EewZERNkuRpGoHCb
			hex!["957356455a1fc364f193d137f9fc36780d12b3f2172f5d92f4591189c05042b5"]
				.unchecked_into(),
			// 5GEMX1VMhner1DonB9gyrFNQC9pX1HUz6ixstxLEa5i8Pt1C
			hex!["b84cebbb0fcc221516c659375d085feb80603901f9ee7418551103cb92870115"]
				.unchecked_into(),
		),
		(
			// 5DchxDdtBkWzNpT8UHHqVQpFYaehagDJV7fcvV1rsrV1erAB
			hex!["44a468940048dd8e4464288a50898a19e608dc7d7eb3a7ab66ce34ca903a3b34"].into(),
			// 5FX382nhyAtz1xBWz4sda1SBSuCwr6FiQXxTQyjChBMgfpRw
			hex!["98c9eea6adccac9194306a80c91b4b8b247b972fb3b46e9e7b14fcc668507172"].into(),
			// 5E9MocwW7kKQtQDfjcghny1SW14DaGoUFnMKZEopuyVFJnQw
			hex!["5c0530e5b166f90185dc3b5c72a4ecb679836e8f44bda774473353c858e4433b"]
				.unchecked_into(),
			// 5HTkbQKfGwfqKtxcrvJ6SuHVFbfD2oELM5NPXnWWx18jmwGA
			hex!["eec10641cd0ea867a25de2242bd7325f52aff749adcab3ad3e2ef4e120665433"]
				.unchecked_into(),
			// 5Ges6uKoMStJiBG6U9SyfXRBNMnaiZeCAC6uMdNQ11zfFu9p
			hex!["cafe668138b13be9b1b634120502af5b94bfa5f84e1a60ba27180e1c31d7b03d"]
				.unchecked_into(),
		),
		(
			// 5GvbqibTDM7poo3fiam9ygzsVGRxXeNBB3HroE9hEyxAt8sp
			hex!["d6fef377b9b139a5878fae43b3fd7fb81c57d84451a7b7e6d8618db5b7ac245a"].into(),
			// 5CWy4HaKQr4GSvYe9QzUDSE2ZW5HdxRV1gvQZcVZX2mKfQQM
			hex!["14078c9f8b7a0e0e211b6ebb3a6a3d49c1d729d9d8543216f402c7d6edfbbe44"].into(),
			// 5FcJeanmxXr1GeY6TokHFGD3nqogonzWdpcyP5P58C5u25VM
			hex!["9cce6d4dad7230ffb947b57b4c893993fbcddb95e7833ecc9f8364b4b122167b"]
				.unchecked_into(),
			// 5CzeWz6gdwCUGEuG2radHEcSQguvdnAecHUB1mDMp9DUhbKT
			hex!["292402b0f2422b617f3e767568cf684c463d5b49249da7bad0e6cab62da59f35"]
				.unchecked_into(),
			// 5HWWvokRgoKnUVqNUzpVs6tzii64DoRvJ5sygfk93HTPUggM
			hex!["f0dcc236e3f42c33fadebc3d0598c9b44a65a1ddf43798eb066018512d718f54"]
				.unchecked_into(),
		),
		(
			// 5EvvoanKphM7627EK2BSazJvsVbambqJkYRu8WmjPzjEWfrD
			hex!["7ec651e7d7b526652f5d8d90af64a2afd89c5c1b14c6d0dca003352b40b7d13c"].into(),
			// 5Gc1k6KFkKyqF2eiaytAsqACRtn6zzmUCfALJrqx1AfEQSZX
			hex!["c8d1c0c8aefdf8ef28f82865b97ff8f9ff727de7e790c1ad319c188708cb0d68"].into(),
			// 5HE6Aa2kpV6Q4czJDEJaBHHiT3R4LmGATMemDJtRkWEwKUSs
			hex!["e4557937ac522fb9b37fdbbc29a095bea2c720825555357075d05bf997a9ed1b"]
				.unchecked_into(),
			// 5G4z7jF4BRXTq1BvYb21oUWKsxqHxWS7RZ4FrMG3nQuUYD5B
			hex!["b127ab30bd4c546c81f6e2eec5fdf14bc48abe72dfeede437b3d7b7e2017a0fd"]
				.unchecked_into(),
			// 5CXEFshj3WjGp5nNLicp2hf3CtW7CBu4CxRFNbbKGqLgrjxQ
			hex!["143ab765d99f620afa1326e6843c9e1f677c22b55a522ebb905ed49de76a0236"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5D7tFaeJ6cfYP2rYZ2oZP6WAtB8LAwLymNHQYCxBqqEmwgjp
		"2ea8f664f8d88208742f7b2f0df87d11e011eb2c9814be64b570be42debba217"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![
		root_key.clone(),
		// 5FRHuTwLJNacWiVHeipJdYTQW8431hL7fBFFn8EVLKZGV3T3
		hex!["94683996ba380b51d678b2f2ac57a3c46f3b552a5b8c08b6c9b05c2f67ff901a"].into(),
		// 5EeBUyATqNtcopfyU5CqYTPt8PctWcHD1mzN5mYs2FF7SnQc
		hex!["72008fed483bb939cd816799682d5d30ef16669e091f5fce2e909b3d8c623f4b"].into(),
		// 5F7WnRvszr3fu7czADjnsdCfLdb9HSfmNEM7J7VAgcYbKL5F
		hex!["86d9294a6c2b6c2b0ad4d80163db108465b66d11258447f68cea63d9bff4407f"].into(),
		// 5Dd75E8UM2TqKRKEkBx4XjPyGJBWdV2xPU68s9H35LKhUrn7
		hex!["44f23d7c7eb7fe9db27b25e5b8c4ae1348693ac47a032330b291bb6ceeb36e45"].into(),
	];

	testnet_genesis(initial_authorities, vec![], root_key, endowed_accounts)
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Substrate Stencil",
		"stencil_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			// 5Grpw9i5vNyF6pbbvw7vA8pC5Vo8GMUbG8zraLMmAn32kTNH
			hex!["d41e0bf1d76de368bdb91896b0d02d758950969ea795b1e7154343ee210de649"].into(),
			// 5DLMZF33f61KvPDbJU5c2dPNQZ3jJyptsacpvsDhwNS1wUuU
			hex!["382bd29103cf3af5f7c032bbedccfb3144fe672ca2c606147974bc2984ca2b14"].into(),
			// 5Dhd2QbrSE4dyNn3YUg8j5TY3fG7ZAWZMoRRF9KUc7VPVGmC
			hex!["48640c12bc1b351cf4b051ac1cf7b5740765d02e34989d0a9dd935ce054ebb21"]
				.unchecked_into(),
			// 5C6rkxAZB437B5Bf1yS4B4qjW4HZPeBp8Kzx2Se9FLKhfyHY
			hex!["01a474a93a0cf830fb40b1d17fd1fc7c6b4a95fa11f90345558574a72da0d4b1"]
				.unchecked_into(),
			// 5DscuovXyY1o7DxYroYjYgipn87eqYLyQA3HJ21Utb7TqAai
			hex!["50041e469c63c994374a2829b0b0829213abd53be5113e751043318a9d7c0757"]
				.unchecked_into(),
		),
		(
			// 5CFDk3yCSgQ2goiaksMfRMFRS7ZU28BZqPQDeAsgZUa6FRzt
			hex!["08050f1b6bcd4651004df427c884073652bafd54e5ca25cea69169532db2910b"].into(),
			// 5F1ks2enazaPktQa3HURLK8GywzNZaGirovPtFvvbv91TLhJ
			hex!["8275157f2a1d8373106cb00078a73a92a3303f3bf6eb72c3a67413bd943b020b"].into(),
			// 5CQ7gVQj96m8y79qPCqrM291rSNREfZ1Tf2fiLPSJReWTNy2
			hex!["0ecddcf7643a98de200b80fe7b18ebd38987fa106c5ed84fc004fa75ea4bac67"]
				.unchecked_into(),
			// 5FyNaMc6GaioN7K9QzPJDEtGThJ1HmcruRdgtiRxaoAwn2VD
			hex!["acdfcce0e40406fac1a8198c623ec42ea13fc627e0274bbb6c21e0811482ce13"]
				.unchecked_into(),
			// 5EUhcM9WPJGvhCz1UptA7ye8TgktGqbhaeSohCkAfW76q5bS
			hex!["6ac58683d639d3992a0090ab15f8c1dcf5a5ab7652fc9de60845441f9fc93903"]
				.unchecked_into(),
		),
		(
			// 5F6YideXfGcskpdFUczu3nZcJFmU9WKHgjjNVQjqgeVGRs66
			hex!["861c6d95051f942bb022f13fc2125b2974933d8ab1441bfdee9855e9d8051556"].into(),
			// 5F92x4qKNYaHtfp5Yy7kb9r6gHCHkN3YSvNuedERPHgrURTn
			hex!["8801f479e09a78515f1badee0169864dae45648109091e29b03a7b4ea97ec018"].into(),
			// 5CLqVJSpfAdMYW1FHygEV8iEi8XFornEcCzrhw9WmFbbp8Qp
			hex!["0c4d9de1e313572750abe19140db56433d20e4668e09de4df81a36566a8f2528"]
				.unchecked_into(),
			// 5HEQh8yEv4QU7joBCKYdjJJ57qU1gDAm4Xv5QZKfFnSbXpeo
			hex!["e493d74f9fa7568cca9dd294c9619a54c2e1b6bd3ecf3677fa7f9076b98c3fcd"]
				.unchecked_into(),
			// 5GUEUCusMfW9c229gyuDG6XUH9pi3Cs4EZR9STtw8opfKuS6
			hex!["c2e2a133b23995a48ff46cc704ef61929ee4a29b5fa468e41019ac63f3694e1f"]
				.unchecked_into(),
		),
		(
			// 5FxxpyvEnE2sVujvhr6x4A4G171uv4WKSLvrUNst9M8MfdpV
			hex!["ac8fdba5bbe008f65d0e85181daa5443c2eb492fea729a5981b2161467f8655c"].into(),
			// 5FxFAYsTNf31D5AGbXW9ETZPUZofpreHjJkdKehidcvDt5X4
			hex!["ac039bef73f76755d3747d711554f7fb0f16022da51483e0d600c9c7c8cbf821"].into(),
			// 5GdjiBeMEFqTE6mWod3UqPrtkQTscRGtAcmdSbR26vGiXpwB
			hex!["ca2245b6fa117fab9353a2031104d1d5d62e311957f375762324e65d71127465"]
				.unchecked_into(),
			// 5DMfkaaR4tzmarUsRMkrbnFNmVnYtYjTPFJsjvA4X15WAZZB
			hex!["392c51bf0c08f89cb1e091782d81359475d780986968ba7f6fa60f41feda6bf7"]
				.unchecked_into(),
			// 5HGzdyJakxDdnERv3nvNjd6Xmz5R39NEuuJ2B3miubDY6BHD
			hex!["e68c9a2ee25e1999a4e87906aea429f3e5f3fc8dc9cd89f423d82860c6937b2e"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5FemZuvaJ7wVy4S49X7Y9mj7FyTR4caQD5mZo2rL7MXQoXMi
		"9eaf896d76b55e04616ff1e1dce7fc5e4a417967c17264728b3fd8fee3b12f3c"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, vec![], root_key, endowed_accounts)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			// TODO: ForceEra::ForceNone
			..Default::default()
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: GrandpaConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * DOLLARS,
			min_join_bond: 1 * DOLLARS,
			..Default::default()
		},
	}
}
