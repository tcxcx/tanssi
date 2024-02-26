//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn add_collective() {
		#[extrinsic_call]
		add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![1],"Coll1Hash".as_bytes().to_vec().try_into().unwrap());

	}


	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}