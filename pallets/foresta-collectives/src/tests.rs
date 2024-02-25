use crate::{mock::*, Error, Event};
use frame_support::{
	assert_noop, assert_ok,
	traits::tokens::fungibles::{metadata::Inspect as MetadataInspect, Inspect},
	BoundedVec,
};
use frame_system::RawOrigin;
use pallet_carbon_credits::{
	BatchGroupListOf, BatchGroupOf, BatchOf, ProjectCreateParams, RegistryListOf, SDGTypesListOf,
};
use primitives::{Batch, RegistryDetails, RegistryName, Royalty, SDGDetails, SdgType};
use sp_runtime::Percent;
use sp_std::convert::TryInto;

/// helper function to generate standard creation details


#[test]
fn it_works_for_add_collective() {
	new_test_ext().execute_with(|| {
		// Root creates collective
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![1],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));
	});
}

#[test]
fn it_works_for_adding_members_to_collective() {
	new_test_ext().execute_with(|| {
		// Root creates collective
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![1],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));

		let member = 1;

		assert_ok!(ForestaCollectives::join_collective(RawOrigin::Signed(member).into(),1));
	});
}

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(ForestaCollectives::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(ForestaCollectives::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			ForestaCollectives::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}