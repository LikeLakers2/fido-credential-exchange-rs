pub mod extensions;
pub mod credentials;

pub type Base64Url = String;
pub type ShouldBe<T> = Result<T, String>;
pub type Uri = String;

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Header {
	pub version: u16,
	pub exporter: String,
	pub timestamp: u64,
	pub accounts: Vec<Account>,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Account {
	pub id: Base64Url,
	pub user_name: String,
	pub email: String,
	pub full_name: Option<String>,
	pub icon: Option<String>,
	pub collections: Vec<Collection>,
	pub items: Vec<Item>,
	// Optional field, but the default is []
	pub extensions: Vec<crate::extensions::Extension>,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Collection {
	pub id: Base64Url,
	pub title: String,
	pub subtitle: Option<String>,
	pub icon: Option<String>,
	pub items: Vec<LinkedItem>,
	// Optional field, but the default is []
	pub sub_collections: Vec<Self>,
	// Optional field, but the default is []
	pub extensions: Vec<crate::extensions::Extension>,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Item {
	pub id: Base64Url,
	pub creation_at: u64,
	pub modified_at: u64,
	pub type_: ShouldBe<ItemType>,
	pub title: String,
	pub subtitle: Option<String>,
	pub credentials: Vec<crate::credentials::Credential>,
	// Optional field, but the default is []
	pub tags: Vec<String>,
	// Optional field, but the default is []
	pub extensions: Vec<crate::extensions::Extension>,
}

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum ItemType {
	// type_ = "login"
	Login,
	// type_ = "document"
	Document,
	// type_ = "identity"
	Identity,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct LinkedItem {
	pub item: Base64Url,
	pub account: Option<Base64Url>,
}
