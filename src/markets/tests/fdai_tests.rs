use super::*;

#[test]
fn can_deploy_fdai() {
	testing_env!(get_context(carol(), current_block_timestamp()));	
	let mut contract = Markets::default(); 
	contract.deploy_fdai("fdai_test_deployment_1".to_string(), 200000000000000);
}

#[test]
fn can_claim_fdai() {
	testing_env!(get_context(carol(), current_block_timestamp()));	
	let mut contract = Markets::default();
	contract.deploy_fdai("fdai_test_deployment_1".to_string(), 200000000000000);
	contract.claim_fdai();
}

#[test]
fn can_get_fdai_balance() {
	testing_env!(get_context(carol(), current_block_timestamp()));	
	let mut contract = Markets::default(); 
	contract.deploy_fdai("fdai_test_deployment_1".to_string(), 200000000000000);
	contract.claim_fdai();
	let balance = contract.get_fdai_balance(carol());

	println!("{:?}", balance)
}
#[test]
fn get_and_set_fdai() {
	testing_env!(get_context(carol(), current_block_timestamp()));	
	let mut contract = Markets::default(); 
	contract.deploy_fdai("fdai_test_deployment_1".to_string(), 200000000000000);
	let balance = contract.get_and_set_fdai(carol());

	println!("{:?}", balance)
}