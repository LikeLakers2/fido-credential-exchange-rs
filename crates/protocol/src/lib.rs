use fcx_format::CredentialType;

pub type Base64Url = String;
pub type ShouldBe<T> = Result<T, String>;

pub type JWK = String;

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct ExportRequest {
	// Default: 0
	pub version: u16,
	// At least one
	pub hpke: Vec<HPKEParameters>,
	// At least one
	pub archive: Vec<ShouldBe<ArchiveAlgorithm>>,
	// This field is defined solely as type `tstr` in the spec, but it seems to want a ResponseMode,
	// so I'm going to assume they allow either-or.
	pub mode: ShouldBe<ResponseMode>,
	pub importer: String,
	// At least one, if defined
	pub credential_types: Option<Vec<ShouldBe<CredentialType>>>,
	// At least one, if defined
	pub known_extensions: Option<Vec<String>>,
}

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum ResponseMode {
	// "direct"
	Direct,
	// "indirect"
	Indirect,
	// "self"
	Self_,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct ExportResponse {
	pub version: u16,
	pub hpke: HPKEParameters,
	pub archive: ShouldBe<ArchiveAlgorithm>,
	pub exporter: String,
	pub payload: Base64Url,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct HPKEParameters {
	pub mode: ShouldBe<HPKEMode>,
	pub kem: u16,
	pub kdf: u16,
	pub aead: u16,
	pub key: Option<JWK>,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum HPKEMode {
	// mode = "base"
	Base,
	// mode = "psk"
	Psk,
	// mode = "auth"
	Auth,
	// mode = "auth-psk"
	AuthPsk,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum ArchiveAlgorithm {
	// archive = "deflate"
	Deflate,
}