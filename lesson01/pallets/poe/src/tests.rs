use super::{Config, Proofs};
use crate::{
	mock::{new_test_ext, Event as TestEvent, Origin, PoeModule, System, Test},
	Error,
};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
		// System::assert_has_event(TestEvent::PoeModule(crate::Event::ClaimCreated(
		// 	1,
		// 	claim.clone(),
		// )));
	});
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
	new_test_ext().execute_with(|| {
		let limit_size = <Test as Config>::MaxClaimLength::get();
		assert_noop!(
			PoeModule::create_claim(
				Origin::signed(1),
				vec![0u8; limit_size.checked_add(1).unwrap() as usize]
			),
			Error::<Test>::ClaimTooLong
		);
	});
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

		assert_eq!(Proofs::<Test>::get(&bounded_claim), None);

		// System::assert_has_event(TestEvent::PoeModule(crate::Event::ClaimRevoked(
		// 	1,
		// 	claim.clone(),
		// )));
	});
}

#[test]
fn revoke_claim_failed_when_claim_is_too_long() {
	new_test_ext().execute_with(|| {
		let limit_size = <Test as Config>::MaxClaimLength::get();
		assert_noop!(
			PoeModule::revoke_claim(
				Origin::signed(1),
				vec![0u8; limit_size.checked_add(1).unwrap() as usize]
			),
			Error::<Test>::ClaimTooLong
		);
	});
}

#[test]
fn revoke_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn revoke_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	});
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);

		// System::assert_has_event(TestEvent::PoeModule(crate::Event::ClaimTransferred(
		// 	1,
		// 	2,
		// 	claim.clone(),
		// )));
	});
}

#[test]
fn transfer_claim_failed_when_claim_is_too_long() {
	new_test_ext().execute_with(|| {
		let limit_size = <Test as Config>::MaxClaimLength::get();
		assert_noop!(
			PoeModule::transfer_claim(
				Origin::signed(1),
				vec![0u8; limit_size.checked_add(1).unwrap() as usize],
				2
			),
			Error::<Test>::ClaimTooLong
		);
	});
}

#[test]
fn transfer_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn transfer_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	});
}
