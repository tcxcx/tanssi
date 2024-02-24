#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::BoundedVec;
	use scale_info::TypeInfo;
	use codec::{Codec, FullCodec, MaxEncodedLen, EncodeLike};

	use sp_runtime::{
		traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize,Zero,CheckedAdd,CheckedSub}
		,ArithmeticError,FixedPointOperand,};
	use sp_std::{
		vec::Vec,
	};
	use sp_std::{fmt::Debug,cmp::{Eq, PartialEq}};
	use frame_support::traits::Contains;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Collective<T:Config> {
        pub name: BoundedVec<u8,T::MaxStringLength>,
		pub hash: BoundedVec<u8, T::MaxStringLength>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Vote<T:Config> {
		pub yes_votes: u64,
		pub no_votes: u64,
		pub end: BlockNumberFor<T>,
		pub status: VoteStatus,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug,MaxEncodedLen, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum MembershipStatus {
		Active,
		InActive,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug,MaxEncodedLen, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum ProjectStatus {
		VoteInprogress,
		Ongoing,
		Rejected,
	}

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen,Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum VoteStatus {
		InProgress,
		Passed,
		Failed,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_carbon_credits::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		type KYCProvider: Contains<Self::AccountId>;
		type CollectiveId: Parameter
			+ FullCodec
			+ Default
			+ Eq
			+ PartialEq
			+ Copy
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ Debug
			+ TypeInfo
			+ From<u32>
			+ Into<u32>
			+ EncodeLike
			+ CheckedAdd;
		type MaxStringLength: Get<u32>;
		type MaxNumManagers: Get<u32>;
		type MaxConcurrentVotes: Get<u32>;
		type MaxProjectsPerCollective: Get<u32>;
		type VotingDuration: Get<BlockNumberFor<Self>>;
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;


	#[pallet::storage]
	#[pallet::getter(fn get_collective)]
	pub(super) type CollectivesMap<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		Collective<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn check_member)]
	pub(super) type Members<T:Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		Blake2_128Concat,
		T::AccountId,
		bool,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_membership_count)]
	pub(super) type MembersCount<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		u32,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_managers)]
	pub(super) type Managers<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		BoundedVec<T::AccountId, T::MaxNumManagers>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_approved_projects)]
	pub(super) type ApprovedProjects<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		BoundedVec<<T as pallet_carbon_credits::Config>::ProjectId, T::MaxProjectsPerCollective>,
		ValueQuery,
	>;


	#[pallet::storage]
	#[pallet::getter(fn get_project_approval_voting)]
	pub type ProjectApprovalVoting<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<(T::CollectiveId,<T as pallet_carbon_credits::Config>::ProjectId), T::MaxConcurrentVotes>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_project_approval_vote)]
	pub(super) type ProjectApprovalVote<T:Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		Blake2_128Concat,
		<T as pallet_carbon_credits::Config>::ProjectId,
		Vote<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn check_project_approval_vote)]
	pub(super) type CheckProjectApprovalVote<T:Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		<T as pallet_carbon_credits::Config>::ProjectId,
		bool,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn collectives_count)]
	pub(super) type CollectivesCount<T: Config> = StorageValue<_, T::CollectiveId,ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		CollectiveCreated { uid2 : T::CollectiveId },
		MemberAdded {collective_id: T::CollectiveId, member: T::AccountId, uid: u32},
		ProjectApprovalInit { collective_id: T::CollectiveId, project_id: <T as pallet_carbon_credits::Config>::ProjectId},
		ProjectApprovalVoteCast { collective_id: T::CollectiveId, project_id: <T as pallet_carbon_credits::Config>::ProjectId},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// KYC Failed
		KYCAuthorisationFailed,
		/// Member Already Exists
		MemberAlreadyExists,
		/// Member Does Not Exist
		MemberDoesNotExist,
		/// Collective Does Not exist
		CollectiveDoesNotExist,
		/// Not Allowed To Manage Membership
		NotAllowedToManageMembership,
		/// NoManagersFound
		NoManagersFound,
		/// MismatchTypes
		MisMatchTypes,
		/// Max Voting Exceeded
		MaxVotingExceeded,
		/// Vote Not Found
		VoteNotFound,
		/// Not Allowed To Vote
		NotAllowedToVote,
		/// Vote Not In Progress
		VoteNotInProgress,
		/// Already Voted
		AlreadyVoted,
		/// Project Not Found
		ProjectNotFound,
		/// Max Projects Exceeded
		MaxProjectsExceeded,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight {
			let mut weight = T::DbWeight::get().reads_writes(1, 1);

			let approval_vote = ProjectApprovalVoting::<T>::take(n);

			for (c_id,p_id) in approval_vote.iter() {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(2, 2));
				let vote = ProjectApprovalVote::<T>::take(c_id,p_id);
				let mut is_approved: bool = false;
				if let Some(mut vote) = vote {
					if vote.yes_votes > vote.no_votes {
						vote.status = VoteStatus::Passed;						
						is_approved = true;
					} else {
						vote.status = VoteStatus::Failed;						
					}

					let _ = Self::do_approve_project(*c_id,*p_id,is_approved);

					ProjectApprovalVote::<T>::insert(c_id,p_id,&vote);
					
				}
			}

			weight
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn add_collective(origin: OriginFor<T>, name: BoundedVec<u8,T::MaxStringLength>,
		managers: BoundedVec<T::AccountId, T::MaxNumManagers>,hash : BoundedVec<u8,T::MaxStringLength>)  -> DispatchResult {
			
			<T as pallet::Config>::ForceOrigin::ensure_origin(origin)?;

			let collective = Collective::<T> {
				name: name,
				hash: hash,
			};

			let uid = Self::collectives_count();
			let uid2 = uid.checked_add(&1u32.into()).ok_or(ArithmeticError::Overflow)?;
			CollectivesMap::<T>::insert(uid2,&collective);
			Managers::<T>::insert(uid2,&managers);
			CollectivesCount::<T>::put(uid2);
			Self::deposit_event(Event::CollectiveCreated{ uid2 });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn join_collective(origin: OriginFor<T>, collective_id: T::CollectiveId) -> DispatchResult {
			let member = ensure_signed(origin)?;
			Self::check_kyc_approval(&member)?;
			ensure!(!Self::check_member(collective_id,member.clone()),Error::<T>::MemberAlreadyExists);
			let uid = Self::get_membership_count(collective_id.clone()).checked_add(1).ok_or(ArithmeticError::Overflow)?;
			Members::<T>::insert(collective_id.clone(),member.clone(),true);
			MembersCount::<T>::insert(collective_id.clone(),uid);
			
			Self::deposit_event(Event::MemberAdded{ collective_id, member, uid });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn add_member(origin: OriginFor<T>, collective_id: T::CollectiveId, member: T::AccountId,
		metadata: BoundedVec<u8,T::MaxStringLength>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!Members::<T>::contains_key(collective_id.clone(),&member.clone()), Error::<T>::MemberAlreadyExists);
			let managers = Managers::<T>::get(collective_id.clone());
			
			match managers.binary_search(&who) {
				Ok(_) => {
					let uid = Self::get_membership_count(collective_id.clone()).checked_add(1).ok_or(ArithmeticError::Overflow)?;


					Members::<T>::insert(collective_id.clone(),member.clone(),true);
					MembersCount::<T>::insert(collective_id.clone(),uid);
					Self::deposit_event(Event::MemberAdded{ collective_id, member, uid });

					Ok(())
				},
				Err(_) => Err(Error::<T>::NotAllowedToManageMembership.into()),

			}
			
		}

		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn init_project_approval(origin: OriginFor<T>, collective_id: T::CollectiveId, 
		project_id: <T as pallet_carbon_credits::Config>::ProjectId) -> DispatchResult {
			
			let who = ensure_signed(origin)?;
			ensure!(Members::<T>::contains_key(collective_id.clone(),&who.clone()), Error::<T>::MemberDoesNotExist);
			
			let project_details: pallet_carbon_credits::ProjectDetail<T> =
					pallet_carbon_credits::Pallet::get_project_details(project_id)
						.ok_or(Error::<T>::ProjectNotFound)?;

			let current_block = <frame_system::Pallet<T>>::block_number();

			let final_block = current_block + T::VotingDuration::get();

			ProjectApprovalVoting::<T>::try_mutate(final_block, |projects| {
				projects.try_push((collective_id,project_id)).map_err(|_| Error::<T>::MaxVotingExceeded)?;
				Ok::<(),DispatchError>(())
			})?; 

			let vote_info = Vote::<T> {
				yes_votes: 0,
				no_votes: 0,
				end: final_block,
				status: VoteStatus::InProgress,
			};

			ProjectApprovalVote::<T>::insert(collective_id,project_id,&vote_info);

			Self::deposit_event(Event::ProjectApprovalInit{ collective_id, project_id });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn project_approval_vote(origin: OriginFor<T>,collective_id: T::CollectiveId, 
		project_id: <T as pallet_carbon_credits::Config>::ProjectId, vote_cast: bool) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Check if member
			ensure!(Members::<T>::contains_key(collective_id.clone(),who.clone()), Error::<T>::NotAllowedToVote);
			// Get vote
			let mut vote = Self::get_project_approval_vote(collective_id,project_id).ok_or(Error::<T>::VoteNotFound)?;
			// Check if vote is in progress
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);
			// Check if member has already voted
			ensure!(!Self::check_project_approval_vote(who.clone(),project_id), Error::<T>::AlreadyVoted);

			if vote_cast {
				vote.yes_votes = vote.yes_votes + 1;
			} else {
				vote.no_votes = vote.no_votes + 1;
			}

			ProjectApprovalVote::<T>::insert(collective_id,project_id,vote);
			CheckProjectApprovalVote::<T>::insert(who.clone(),project_id,true);

			Self::deposit_event(Event::ProjectApprovalVoteCast{ collective_id, project_id });
			
			Ok(())
		}
		
		#[pallet::call_index(7)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(8)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

	}

	impl<T:Config> Pallet<T> {
		pub fn check_kyc_approval(account_id: &T::AccountId) -> DispatchResult {
			if !<T as pallet::Config>::KYCProvider::contains(account_id) {
				Err(Error::<T>::KYCAuthorisationFailed.into())
			} else {
				Ok(())
			}
		}

		pub fn do_approve_project(collective_id: T::CollectiveId,
		project_id: <T as pallet_carbon_credits::Config>::ProjectId, is_approved: bool) -> DispatchResult {
			pallet_carbon_credits::Pallet::<T>::approve_project(frame_system::RawOrigin::Root.into(),project_id,is_approved)?;
			ApprovedProjects::<T>::try_mutate(collective_id, |projects| {
				projects.try_push(project_id).map_err(|_| Error::<T>::MaxProjectsExceeded)?;
				Ok::<(),DispatchError>(())
			})?; 
			Ok(())
		}

		pub fn do_authorize_account(account: T::AccountId) -> DispatchResult {
			pallet_carbon_credits::Pallet::<T>::force_add_authorized_account(frame_system::RawOrigin::Root.into(),account)?;

			Ok(())
		}

	}
}
