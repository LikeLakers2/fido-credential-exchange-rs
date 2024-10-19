use fcx_format::CredentialType;

pub type Base64UrlString = String;
pub type ShouldBe<T> = Result<T, String>;

pub type JWK = String;

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub struct ExportRequest {
	pub version: u16,
	pub hpke: Vec<HPKEParameters>,
	pub archive: Vec<ShouldBe<ArchiveAlgorithm>>,
	pub importer: String,
	pub credential_types: Option<Vec<ShouldBe<CredentialType>>>,
	pub known_extensions: Option<Vec<String>>,
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
	pub payload: Base64UrlString,
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
	Base,
	Psk,
	Auth,
	AuthPsk,
}

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
)]
pub enum ArchiveAlgorithm {
	Deflate,
}