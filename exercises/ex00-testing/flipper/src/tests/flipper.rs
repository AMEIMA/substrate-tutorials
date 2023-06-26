use super::mock::*;
use crate::{pallet, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(Origin::signed(ALICE), false));
		assert_eq!(Flipper::value(), Some(false));
	});
}

#[test]
fn set_value_err_already_set() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(Origin::signed(ALICE), false));
		assert_eq!(Flipper::value(), Some(false));
		assert_noop!(
			Flipper::set_value(Origin::signed(ALICE), false),
			Error::<TestRuntime>::AlreadySet
		);
	});
}

#[test]
fn flip_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(Origin::signed(ALICE), false));
		assert_eq!(Flipper::value(), Some(false));
		assert_ok!(Flipper::flip_value(Origin::signed(ALICE)));
		assert_eq!(Flipper::value(), Some(true));
	});
}

#[test]
fn flip_value_ko() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Flipper::flip_value(Origin::signed(ALICE)),
			Error::<TestRuntime>::NoneValue
		);
	});
}
