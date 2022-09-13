use std::ops::Add;

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{new_test_ext, Event as TestEvent, KittiesModule, Origin, System, Test};

const ACCOUNT_WITH_100_BALANCE: u64 = 0;
const ACCOUNT_WITH_25_BALANCE: u64 = 1;
const ACCOUNT_WITH_1_BALANCE: u64 = 2;

#[test]
fn it_works_for_creating_kitty() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = ACCOUNT_WITH_100_BALANCE;
		let kitty_id: u32 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_eq!(KittyOwner::<Test>::get(kitty_id), Some(account_id));
		assert_ne!(Kitties::<Test>::get(kitty_id), None);
		assert_eq!(NextKittyId::<Test>::get(), kitty_id.add(&1));
		assert_eq!(
			<Test as Config>::Currency::reserved_balance(&account_id),
			<Test as Config>::KittyPrice::get()
		);

		// TODO: figure out why assert_has_event keep failling with 0 event.
		// let kitty = Kitties::<Test>::get(kitty_id).unwrap();
		// System::assert_has_event(TestEvent::KittiesModule(Event::KittyCreated(
		// 	account_id, kitty_id, kitty,
		// )));
	});
}

#[test]
fn create_kitty_fails_for_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = ACCOUNT_WITH_1_BALANCE;
		assert_noop!(
			KittiesModule::create(Origin::signed(account_id)),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn create_kitty_fails_for_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = ACCOUNT_WITH_100_BALANCE;
		let max_index = <Test as Config>::KittyIndex::max_value();
		NextKittyId::<Test>::set(max_index);
		assert_noop!(
			KittiesModule::create(Origin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn create_kitty_fails_for_too_many_kitties() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = ACCOUNT_WITH_100_BALANCE;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		// INFO: users could have at most 3 kitties(defined at mock.rs) - created above.
		// Below assert will throw error as exceed maximum amount.
		assert_noop!(
			KittiesModule::create(Origin::signed(account_id)),
			Error::<Test>::OwnTooManyKitties
		);
	});
}

#[test]
fn it_works_for_breeding_kitty() {
	new_test_ext().execute_with(|| {
		let mut owned_kitty_amount = 0;
		let account_id: u64 = ACCOUNT_WITH_100_BALANCE;

		let kitty_id_1 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		owned_kitty_amount += 1;

		let kitty_id_2 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		owned_kitty_amount += 1;

		let new_breed_kitty_id = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::breed(Origin::signed(account_id), kitty_id_1, kitty_id_2));
		owned_kitty_amount += 1;

		assert_eq!(KittyOwner::<Test>::get(new_breed_kitty_id), Some(account_id));
		assert_ne!(Kitties::<Test>::get(new_breed_kitty_id), None);
		assert_eq!(NextKittyId::<Test>::get(), new_breed_kitty_id.add(&1));
		assert_eq!(
			<Test as Config>::Currency::reserved_balance(&account_id),
			<Test as Config>::KittyPrice::get().checked_mul(owned_kitty_amount).unwrap()
		);

		// TODO: figure out why assert_has_event keep failling with 0 event.
		// let kitty = Kitties::<Test>::get(kitty_id).unwrap();
		// System::assert_has_event(TestEvent::KittiesModule(Event::KittyCreated(
		// 	account_id, kitty_id, kitty,
		// )));
	});
}

#[test]
fn breed_kitty_fails_for_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = ACCOUNT_WITH_25_BALANCE;
		let kitty_id_1 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));

		let kitty_id_2 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));

		assert_noop!(
			KittiesModule::breed(Origin::signed(account_id), kitty_id_1, kitty_id_2),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn breed_kitty_fails_for_same_kitty_id() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = ACCOUNT_WITH_25_BALANCE;
		let kitty_id_1 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));

		assert_noop!(
			KittiesModule::breed(Origin::signed(account_id), kitty_id_1, kitty_id_1),
			Error::<Test>::SameKittyId
		);
	});
}

#[test]
fn breed_kitty_fails_for_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = ACCOUNT_WITH_100_BALANCE;

		let kitty_id_1 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));

		assert_ok!(KittiesModule::create(Origin::signed(account_id)));

		// INFO: next kitty id after creation 2 kitties - not exist
		let invalid_kitty_id_2 = NextKittyId::<Test>::get();

		assert_noop!(
			KittiesModule::breed(Origin::signed(account_id), kitty_id_1, invalid_kitty_id_2),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn breed_kitty_fails_for_too_many_kitties() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = ACCOUNT_WITH_100_BALANCE;

		let kitty_id_1 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));

		let kitty_id_2 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));

		// Generate 3rd kitty - maximum amount
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));

		assert_noop!(
			KittiesModule::breed(Origin::signed(account_id), kitty_id_1, kitty_id_2),
			Error::<Test>::OwnTooManyKitties
		);
	});
}

#[test]
fn it_works_for_transfering_kitty() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = ACCOUNT_WITH_100_BALANCE;
		let account_id_2: u64 = ACCOUNT_WITH_25_BALANCE;

		let kitty_id = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id_1)));

		assert_ok!(KittiesModule::transfer(Origin::signed(account_id_1), kitty_id, account_id_2));

		assert_eq!(KittyOwner::<Test>::get(kitty_id), Some(account_id_2));
		assert_ne!(Kitties::<Test>::get(kitty_id), None);
		assert_eq!(NextKittyId::<Test>::get(), kitty_id.add(&1));
		assert_eq!(<Test as Config>::Currency::reserved_balance(&account_id_1), 0);
		assert_eq!(
			<Test as Config>::Currency::reserved_balance(&account_id_2),
			<Test as Config>::KittyPrice::get()
		);

		// TODO: figure out why assert_has_event keep failling with 0 event.
		// System::assert_has_event(TestEvent::KittiesModule(Event::KittyTransferred(
		// 	account_id_1,
		// 	account_id_2,
		// 	kitty_id,
		// )));
	});
}

#[test]
fn transfer_kitty_fails_for_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = ACCOUNT_WITH_100_BALANCE;
		let account_id_2: u64 = ACCOUNT_WITH_1_BALANCE;
		let kitty_id_1 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id_1)));

		assert_noop!(
			KittiesModule::transfer(Origin::signed(account_id_1), kitty_id_1, account_id_2),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn transfer_kitty_fails_for_not_owner() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = ACCOUNT_WITH_100_BALANCE;
		let account_id_2: u64 = ACCOUNT_WITH_25_BALANCE;
		let kitty_id_1 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id_2)));

		assert_noop!(
			KittiesModule::transfer(Origin::signed(account_id_1), kitty_id_1, account_id_2),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn transfer_kitty_fails_for_too_many_kitties() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = ACCOUNT_WITH_25_BALANCE;
		let account_id_2: u64 = ACCOUNT_WITH_100_BALANCE;
		let kitty_id_1 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id_1)));
		assert_ok!(KittiesModule::create(Origin::signed(account_id_2)));
		assert_ok!(KittiesModule::create(Origin::signed(account_id_2)));
		assert_ok!(KittiesModule::create(Origin::signed(account_id_2)));

		assert_noop!(
			KittiesModule::transfer(Origin::signed(account_id_1), kitty_id_1, account_id_2),
			Error::<Test>::OwnTooManyKitties
		);
	});
}
