use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use frame_support::pallet_prelude::Get;

// 1. 测试申明存证
// 1.1 测试申明存证成功
#[test]
fn create_claim_poe_ok_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
		// 由学员654签名
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(654), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((654, frame_system::Pallet::<Test>::block_number()))
		);
		assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 10);
	})
}

// 1.2 测试申明存证失败，由于已经存在存证
#[test]
fn create_claim_poe_with_claim_already_exist_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(654), claim.clone());

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(654), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

// 2.测试撤回存证
// 2.1 测试撤回存证
#[test]
fn revoke_claim_poe_ok_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		//创建
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(654), claim.clone());

		//撤回
		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(654), claim.clone()));
	})
}

// 2.2 测试撤回不存在的存证
#[test]
fn revoke_claim_poe_with_poe_not_exist_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(654), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

// 2.3 测试撤回不是自己签名的存证
#[test]
fn revoke_claim_poe_with_wrong_owner_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(654), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(655), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

// 3 测试转移存证
// 3.1 测试转移存证
#[test]
fn transfer_claim_poe_ok_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(654), claim.clone());

		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(654), claim.clone(), 655));
	})
}

// 3.2 测试转移不存在的存证
#[test]
fn transfer_claim_poe_with_claim_is_not_exist_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(654), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

// 3.3 测试转移不是自己的存证
#[test]
fn transfer_claim_poe_with_wrong_owner_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(654), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(655), claim.clone(), 666),
			Error::<Test>::NotClaimOwner
		);
	})
}
