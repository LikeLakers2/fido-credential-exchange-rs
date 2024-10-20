pub mod shared;

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Extension {
	Other {
		name: String,
	},
	
	Shared {
		// name = "shared"
		accessors: Vec<self::shared::SharingAccessor>,
	},
}