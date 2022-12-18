use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure,
    traits::{Get, IsType},
    weights::{Weight},
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::{
    AtLeast32Bit, CheckedAdd, Member, One, SaturatedConversion, SimpleArithmetic, Zero,
};
use sp_std::prelude::*;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Time: Get<Self::BlockNumber>;
}

decl_storage! {
    trait Store for Module<T: Trait> as OracleEventFeed {
        /// The number of events in the feed.
        EventCount get(fn event_count): u64;
        /// The last `EventCount` events in the feed, stored in reverse chronological order.
        Events get(fn events): Vec<(T::BlockNumber, Vec<u8>)>;
        /// The maximum number of events that the feed can store.
        MaxEvents get(fn max_events): u64;
    }
}

decl_event! {
    pub enum Event<T> where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::BlockNumber,
        <T as Trait>::Event,
    {
        /// An event was added to the feed.
        EventAdded(AccountId, BlockNumber, Vec<u8>),
        /// The feed was rotated, and some events were removed.
        FeedRotated(Vec<(BlockNumber, Vec<u8>)>),
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// The event feed is full and cannot store any more events.
        FeedFull,
        /// Only the authorized account is allowed to add events.
        Unauthorized,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Add an event to the feed. Only the authorized account is allowed to do this.
        #[weight = Weight::Percent(100)]
        pub fn add_event(origin, event: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(T::Authorized::is_authorized(&sender), Error::<T>::Unauthorized);

            let event_count = Self::event_count();
            let max_events = Self::max_events();
            ensure!(event_count < max_events, Error::<T>::FeedFull);

            let now = <T as Trait>::Time::get();
            <EventCount<T>>::put(event_count + 1);
            <Events<T>>::mutate(|events| {
                events.push((now, event));
            });

            Self::deposit_event(Event::<T>::EventAdded(sender, now, event));
