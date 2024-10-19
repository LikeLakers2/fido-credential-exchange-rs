pub type Base64UrlString = String;
pub type ShouldBe<T> = Result<T, String>;

pub type Object = ();


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
	pub id: Base64UrlString,
	pub user_name: String,
	pub email: String,
	pub full_name: Option<String>,
	pub icon: Option<String>,
	pub collections: Vec<Collection>,
	pub items: Vec<Collection>,
	pub extensions: Option<Vec<Extension>>,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Collection {
	pub id: Base64UrlString,
	pub title: String,
	pub subtitle: Option<String>,
	pub icon: Option<String>,
	pub items: Vec<Item>,
	pub sub_collections: Option<Vec<Self>>,
	pub extensions: Option<Vec<Extension>>,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Item {
	pub id: Base64UrlString,
	pub creation_at: u64,
	pub modified_at: u64,
	pub type_: ShouldBe<ItemType>,
	pub title: String,
	pub subtitle: Option<String>,
	pub credentials: Vec<Credential>,
	pub tags: Vec<String>,
	pub extensions: Vec<Extension>,
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
pub enum Credential {
	Other {
		// type_ is unknown or not one of the below options
		type_: String,
	},
	
	BasicAuth {
		// type_ = "basic-auth"
		urls: Vec<String>,
		username: Option<EditableField>,
		password: Option<EditableField>,
	},
	
	Passkey {
		// type_ = "passkey"
		credential_id: Base64UrlString,
		rp_id: String,
		user_name: String,
		user_display_name: String,
		user_handle: String,
		key: Object,
		fido2_extensions: Option<Fido2Extensions>,
	},
	
	Totp {
		// type_ = "totp"
	},
	
	CryptographicKey {
		// type_ = "cryptographic-key"
	},
	
	Note {
		// type_ = "note"
	},
	
	File {
		// type_ = "file"
	},
	
	Address {
		// type_ = "address"
	},
	
	CreditCard {
		// type_ = "credit-card"
		number: String,
		full_name: String,
		card_type: Option<String>,
		verification_number: Option<String>,
		expiry_date: Option<String>,
		valid_from: Option<String>,
	},
	
	SocialSecurityNumber {
		// type_ = "social-security-number"
	},
}

// CredentialType intentionally unused in the above enum
#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum CredentialType {
	// type_ = "basic-auth"
	BasicAuth,
	// type_ = "passkey"
	Passkey,
	// type_ = "totp"
	Totp,
	// type_ = "cryptographic-key"
	CryptographicKey,
	// type_ = "note"
	Note,
	// type_ = "file"
	File,
	// type_ = "address"
	Address,
	// type_ = "credit-card"
	CreditCard,
	// type_ = "social-security-number"
	SocialSecurityNumber,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct EditableField {
	pub id: Base64UrlString,
	pub field_type: String,
	pub value: String,
	pub label: Option<String>,
	pub designation: Option<String>,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fido2Extensions {
	pub hmac_secret: Option<Fido2HmacSecret>,
	pub cred_blob: Option<Base64UrlString>,
	pub large_blob: Option<Fido2LargeBlob>,
	pub payments: Option<bool>,
	pub supplemental_keys: Option<Fido2SupplementalKeys>,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fido2HmacSecret {
	pub algorithm: String,
	pub secret: Base64UrlString,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fido2LargeBlob {
	pub size: u64,
	pub alg: String,
	pub data: Base64UrlString,
}

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fido2SupplementalKeys {
	pub device: Option<bool>,
	pub provider: Option<bool>,
}

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
	pub account_id: Base64UrlString,
	pub name: String,
	pub permissions: Vec<ShouldBe<SharingAccessorPermission>>,
}

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum SharingAccessorType {
	User,
	Group,
}

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum SharingAccessorPermission {
	Read,
	Update,
	Create,
	Delete,
	Share,
	Manage
}