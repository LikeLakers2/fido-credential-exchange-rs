use crate::{Base64Url, ShouldBe, Uri};



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
		urls: Vec<Uri>,
		username: Option<EditableField>,
		password: Option<EditableField>,
	},
	
	Passkey {
		// type_ = "passkey"
		credential_id: Base64Url,
		rp_id: String,
		user_name: String,
		user_display_name: String,
		user_handle: String,
		key: Base64Url,
		fido2_extensions: Option<Fido2Extensions>,
	},
	
	Totp {
		// type_ = "totp"
		secret: String,
		period: u16,
		digits: u16,
		username: String,
		algorithm: ShouldBe<OtpHashAlgorithm>,
		issuer: Option<String>,
	},
	
	CryptographicKey {
		// type_ = "cryptographic-key"
	},
	
	Note {
		// type_ = "note"
		content: String,
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
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum OtpHashAlgorithm {
	// algorithm = "sha1"
	Sha1,
	// algorithm = "sha256"
	Sha256,
	// algorithm = "sha512"
	Sha512,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct EditableField {
	pub id: Base64Url,
	pub field_type: ShouldBe<FieldType>,
	pub value: String,
	pub label: Option<String>,
}

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum FieldType {
	// field_type = "string"
	String,
	// field_type = "concealed-string"
	ConcealedString,
	// field_type = "email"
	Email,
	// field_type = "number"
	Number,
	// field_type = "boolean"
	Boolean,
	// field_type = "date"
	Date,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fido2Extensions {
	pub hmac_secret: Option<Fido2HmacSecret>,
	pub cred_blob: Option<Base64Url>,
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
	pub secret: Base64Url,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fido2LargeBlob {
	pub size: u64,
	pub alg: String,
	pub data: Base64Url,
}

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fido2SupplementalKeys {
	pub device: Option<bool>,
	pub provider: Option<bool>,
}
