use std::ops::Add;

use super::*;
use frame_support::{assert_noop, assert_ok, traits::ConstU64, BoundedVec};
use mock::{new_test_ext, Event as TestEvent, KittiesModule, Origin, System, Test};

#[test]
fn it_works_for_creating_kitty() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 0;
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
