use crate::{Base64Url, ShouldBe};

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
		accessors: Vec<SharingAccessor>,
	},
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct SharingAccessor {
	pub type_: ShouldBe<SharingAccessorType>,
	pub account_id: Base64Url,
	pub name: String,
	pub permissions: Vec<ShouldBe<SharingAccessorPermission>>,
}

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum SharingAccessorType {
	// type_ = "user"
	User,
	// type_ = "group"
	Group,
}

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum SharingAccessorPermission {
	// permissions = ["read"]
	Read,
	// permissions = ["update"]
	Update,
	// permissions = ["create"]
	Create,
	// permissions = ["delete"]
	Delete,
	// permissions = ["share"]
	Share,
	// permissions = ["manage"]
	Manage
}
