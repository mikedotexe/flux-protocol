use super::*;
use near_sdk::MockedBlockchain;
use near_sdk::{VMContext, testing_env};
use near_crypto::{InMemorySigner, KeyType, Signer};
use near_runtime_standalone::{init_runtime_and_signer, RuntimeStandalone};
use near_primitives::{
    account::{AccessKey},
    errors::{RuntimeError, TxExecutionError},
    hash::CryptoHash,
    transaction::{ExecutionOutcome, ExecutionStatus, Transaction},
    types::{AccountId, Balance},
};
use std::collections::{HashMap};

use serde_json::json;
use near_sdk::json_types::U128;

type TxResult = Result<ExecutionOutcome, ExecutionOutcome>;

lazy_static::lazy_static! {
    static ref MARKETS_BYTES: &'static [u8] = include_bytes!("../../../res/flux_protocol.wasm").as_ref();
    static ref FUNGIBLE_TOKEN_BYTES: &'static [u8] = include_bytes!("../../../res/fungible_token.wasm").as_ref();
}


pub fn ntoy(near_amount: Balance) -> Balance {
    near_amount * 10u128.pow(24)
}

fn outcome_into_result(outcome: ExecutionOutcome) -> TxResult {
    match outcome.status {
        ExecutionStatus::SuccessValue(_) => Ok(outcome),
        ExecutionStatus::Failure(_) => Err(outcome),
        ExecutionStatus::SuccessReceiptId(_) => panic!("Unresolved ExecutionOutcome run runtime.resolve(tx) to resolve the final outcome of tx"),
        ExecutionStatus::Unknown => unreachable!()
    }
}

pub struct ExternalUser {
    account_id: AccountId,
    signer: InMemorySigner,
}

impl ExternalUser {
    pub fn new(account_id: AccountId, signer: InMemorySigner) -> Self {
        Self { account_id, signer }
    }

    pub fn get_account_id(&self) -> AccountId {
        return self.account_id.to_string();
    }

    pub fn markets_init_new(&self, runtime: &mut RuntimeStandalone) -> TxResult {
        let args = json!({}).to_string().as_bytes().to_vec();

        let tx = self
            .new_tx(runtime, "flux-tests".to_string())
            .create_account()
            .transfer(99994508400000000000000000)
            .deploy_contract(MARKETS_BYTES.to_vec())
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        let ans = outcome_into_result(res);
        //println!("{:?}", ans);
        return ans;
    }

    pub fn token_deploy_call_new(&self, runtime: &mut RuntimeStandalone, account: String, total_supply: U128) -> TxResult {
        // let args = json!({}).to_string().as_bytes().to_vec();
        let args = json!({
            "owner_id": account,
            "total_supply": total_supply
        }).to_string().as_bytes().to_vec();

        let tx = self
            .new_tx(runtime, account)
            .deploy_contract(FUNGIBLE_TOKEN_BYTES.to_vec())
            .function_call("new".into(), args, 10000000000000000, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        let ans = outcome_into_result(res);
        return ans;
    }

    fn new_tx(&self, runtime: &RuntimeStandalone, receiver_id: AccountId) -> Transaction {
        let nonce = runtime
            .view_access_key(&self.account_id, &self.signer.public_key())
            .unwrap()
            .nonce
            + 1;
        Transaction::new(
            self.account_id.clone(),
            self.signer.public_key(),
            receiver_id,
            nonce,
            CryptoHash::default(),
        )
    }

    pub fn claim_fdai(&self, runtime: &mut RuntimeStandalone) -> TxResult {
        let args = json!({})
            .to_string()
            .as_bytes()
            .to_vec();

        let tx = self
            .new_tx(runtime, "flux-tests".to_string())
            .function_call("claim_fdai".into(), args, 10000000000000000, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        let ans = outcome_into_result(res);
        return ans;
    }

    // TODO: Note I changed u128's to 64's throughout file
    pub fn create_market(
        &self,
        runtime: &mut RuntimeStandalone,
        description: String,
        extra_info: String,
        outcomes: u64,
        outcome_tags: Vec<String>,
        categories: Vec<String>,
        end_time: u64,
        fee_percentage: u64,
        cost_percentage: u64,
        api_source: String,
    ) -> TxResult {
        let args = json!({
            "description": description,
            "extra_info": extra_info,
            "outcomes": outcomes,
            "outcome_tags": outcome_tags,
            "categories": categories,
            "end_time": end_time,
            "fee_percentage": fee_percentage,
            "cost_percentage": cost_percentage,
            "api_source": api_source,
        })
            .to_string()
            .as_bytes()
            .to_vec();
        let tx = self
            .new_tx(runtime, "flux-tests".to_string())
            .function_call("create_market".into(), args, 10000000000000000, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        let ans = outcome_into_result(res);
        return ans;
    }

    pub fn place_order(
        &self,
        runtime: &mut RuntimeStandalone,
        market_id: u64,
        outcome: u64,
        spend: U128,
        price: U128,
    ) -> TxResult {
        let args = json!({
            "market_id": market_id,
            "outcome": outcome,
            "spend": spend,
            "price": price,
        })
            .to_string()
            .as_bytes()
            .to_vec();
        let tx = self
            .new_tx(runtime, "flux-tests".to_string())
            .function_call("place_order".into(), args, 10000000000000000, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        let ans = outcome_into_result(res);
        return ans;
    }

    pub fn resolute_market(
        &self,
        runtime: &mut RuntimeStandalone,
        market_id: u64,
        winning_outcome: Option<u64>,
        stake: U128
    ) -> TxResult {
        let args = json!({
            "market_id": market_id,
            "winning_outcome": winning_outcome,
            "stake": stake
        })
        .to_string()
        .as_bytes()
        .to_vec();
        let tx = self
            .new_tx(runtime, "flux-tests".to_string())
            .function_call("resolute_market".into(), args, 10000000000000000, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        let ans = outcome_into_result(res);
        return ans;
    }

    pub fn finalize_market (
        &self,
        runtime: &mut RuntimeStandalone,
        market_id: u64,
        winning_outcome: Option<u64>
    ) -> TxResult {
        let args = json!({
            "market_id": market_id,
            "winning_outcome": winning_outcome,
        })
        .to_string()
        .as_bytes()
        .to_vec();
        let tx = self
            .new_tx(runtime, "flux-tests".to_string())
            .function_call("finalize_market".into(), args, 10000000000000000, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        let ans = outcome_into_result(res);
        return ans;
    }

    pub fn claim_earnings (
        &mut self,
        runtime: &mut RuntimeStandalone,
        market_id: u64,
        account_id: String
    ) -> TxResult {
        let args = json!({
            "market_id": market_id,
            "account_id": account_id,
        })
        .to_string()
        .as_bytes()
        .to_vec();
        let tx = self
            .new_tx(runtime, "flux-tests".to_string())
            .function_call("claim_earnings".into(), args, 10000000000000000, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        let ans = outcome_into_result(res);
        return ans;
    }

    pub fn get_open_orders(&self, runtime: &RuntimeStandalone, market_id: u64, outcome: u64) -> HashMap<u128, Order> {
        // TODO: SPECIFY WHAT ACCOUNT TO CALL TO
        let open_orders = runtime
            .view_method_call(
                &("flux-tests".to_string()),
                "get_open_orders",
                json!({"market_id": market_id, "outcome": outcome})
                    .to_string()
                    .as_bytes(),
            )
            .unwrap()
            .0;

        //TODO: UPDATE THIS CASTING
        let data: HashMap<&str, serde_json::Value> = serde_json::from_slice(open_orders.as_slice()).unwrap();
        let open_orders_map: HashMap<u128, Order> = serde_json::from_value(serde_json::to_value(data).unwrap()).unwrap();
        println!("{:?}", open_orders_map);
        return open_orders_map;
    }

    pub fn get_filled_orders(&self, runtime: &RuntimeStandalone, market_id: u64, outcome: u64) -> HashMap<u128, Order>{
        // SHOULD RETURN HashMap<u128, Order>
        let filled_orders = runtime
            .view_method_call(
                &("flux-tests".to_string()),
                "get_filled_orders",
                json!({
                    "market_id": market_id,
                    "outcome": outcome
                    })
                    .to_string()
                    .as_bytes(),
            )
            .unwrap()
            .0;
        let data: HashMap<&str, serde_json::Value> = serde_json::from_slice(filled_orders.as_slice()).unwrap();
        let filled_orders_map: HashMap<u128, Order> = serde_json::from_value(serde_json::to_value(data).unwrap()).unwrap();
        println!("{:?}", filled_orders_map);
        return filled_orders_map;
    }

    pub fn get_fdai_metrics(&self, runtime: &RuntimeStandalone) -> Vec<u128> {
        let fdai_metrics = runtime
            .view_method_call(
                &("flux-tests".to_string()),
                "get_fdai_metrics",
                json!({})
                    .to_string()
                    .as_bytes(),
            )
            .unwrap()
            .0;

        //TODO: UPDATE THIS CASTING
        let data: Vec<serde_json::Value> = serde_json::from_slice(fdai_metrics.as_slice()).unwrap();
        let fdai_metrics_vec: Vec<u128> = serde_json::from_value(serde_json::to_value(data).unwrap()).unwrap();
        println!("{:?}", fdai_metrics_vec);
        return fdai_metrics_vec;
    }

    pub fn get_fdai_balance(&self, runtime: &RuntimeStandalone, account_id: String) -> u128 {
        let fdai_balance_json = runtime
            .view_method_call(
                &("flux-tests".to_string()),
                "get_fdai_balance",
                json!({
                    "account_id": account_id
                })
                    .to_string()
                    .as_bytes(),
            )
            .unwrap()
            .0;

        let data: serde_json::Value = serde_json::from_slice(fdai_balance_json.as_slice()).unwrap();
        let fdai_balance: u128 = serde_json::from_value(serde_json::to_value(data).unwrap()).unwrap();
        println!("{:?}", fdai_balance);
        return fdai_balance;
    }

    pub fn get_claimable(&self, runtime: &RuntimeStandalone, market_id: u64, account_id: String) -> u128 {
        let claimable_json = runtime
            .view_method_call(
                &("flux-tests".to_string()),
                "get_claimable",
                json!({
                    "market_id": market_id,
                    "account_id": account_id
                    })
                    .to_string()
                    .as_bytes(),
            )
            .unwrap()
            .0;

        //TODO: UPDATE THIS CASTING
        let data: serde_json::Value = serde_json::from_slice(claimable_json.as_slice()).unwrap();
        let claimable: u128 = serde_json::from_value(serde_json::to_value(data).unwrap()).unwrap();
        println!("{:?}", claimable);
        return claimable;
    }

    pub fn create_external(
        &self,
        runtime: &mut RuntimeStandalone,
        new_account_id: AccountId,
        amount: Balance,
    ) -> Result<ExternalUser, ExecutionOutcome> {
        let new_signer =
            InMemorySigner::from_seed(&new_account_id, KeyType::ED25519, &new_account_id);
        let tx = self
            .new_tx(runtime, new_account_id.clone())
            .create_account()
            .add_key(new_signer.public_key(), AccessKey::full_access())
            .transfer(amount)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx);

        // TODO: this temporary hack, must be rewritten
        if let Err(err) = res.clone() {
            if let RuntimeError::InvalidTxError(tx_err) = err {
                let mut out = ExecutionOutcome::default();
                out.status = ExecutionStatus::Failure(TxExecutionError::InvalidTxError(tx_err));
                return Err(out);
            } else {
                unreachable!();
            }
        } else {
            outcome_into_result(res.unwrap())?;
            runtime.process_all().unwrap();
            Ok(ExternalUser {
                account_id: new_account_id,
                signer: new_signer,
            })
        }
    }

}

pub fn init_markets_contract() -> (RuntimeStandalone, ExternalUser) {
    let (mut runtime, signer) = init_runtime_and_signer(&"root".into());
    let root = ExternalUser::new("root".into(), signer);

    root.markets_init_new(&mut runtime).unwrap();
    return (runtime, root);
}
