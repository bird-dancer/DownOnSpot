#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SpotifyError {
	#[error("Error: {0}")]
	Error(String),
	#[error("IO: {0:?} {1}")]
	IoError(std::io::ErrorKind, String),
	#[error("Mercury Error")]
	MercuryError,
	#[error("Authentication Error")]
	AuthenticationError,
	#[error("Unavailable!")]
	Unavailable,
	#[error("Invalid Spotify ID")]
	SpotifyIdError,
	#[error("Channel Error")]
	ChannelError,
	#[error("Audio Key Error")]
	AudioKeyError,
	#[error("Lame error: {0}")]
	LameConverterError(String),
	#[error("Tokio Join Error")]
	JoinError,
	#[error("Spotify Error: {0}")]
	ASpotify(String),
	#[error("Serde Error @{1}:{2} {0}")]
	Serde(String, usize, usize),
	#[error("Invalid URI")]
	InvalidUri,
	#[error("Parse Error: {0}")]
	ParseError(url::ParseError),
	#[error("ID3 Error: {0} {1}")]
	ID3Error(String, String),
	#[error("Reqwest Error: {0}")]
	Reqwest(String),
	#[error("Invalid Format!")]
	InvalidFormat,
	#[error("Not Connected")]
	NotConnected,
	#[error("Unknown Packet: {0}")]
	UnknownPacket(u8),
	#[error("Already Downloaded")]
	AlreadyDownloaded(String),
}

impl From<std::io::Error> for SpotifyError {
	fn from(e: std::io::Error) -> Self {
		Self::IoError(e.kind(), e.to_string())
	}
}
impl From<Box<dyn std::error::Error>> for SpotifyError {
	fn from(e: Box<dyn std::error::Error>) -> Self {
		Self::Error(e.to_string())
	}
}

impl From<librespot::core::mercury::MercuryError> for SpotifyError {
	fn from(_: librespot::core::mercury::MercuryError) -> Self {
		Self::MercuryError
	}
}

impl From<librespot::core::error::Error> for SpotifyError {
	fn from(e: librespot::core::error::Error) -> Self {
		SpotifyError::Error(e.to_string())
	}
}

impl From<librespot::core::session::SessionError> for SpotifyError {
	fn from(e: librespot::core::session::SessionError) -> Self {
		match e {
			librespot::core::session::SessionError::IoError(e) => e.into(),
			librespot::core::session::SessionError::AuthenticationError(_) => {
				SpotifyError::AuthenticationError
			}
			librespot::core::session::SessionError::NotConnected => SpotifyError::NotConnected,
			librespot::core::session::SessionError::Packet(e) => SpotifyError::UnknownPacket(e),
		}
	}
}

impl From<librespot::core::spotify_id::SpotifyIdError> for SpotifyError {
	fn from(_: librespot::core::spotify_id::SpotifyIdError) -> Self {
		Self::SpotifyIdError
	}
}

impl From<librespot::core::channel::ChannelError> for SpotifyError {
	fn from(_: librespot::core::channel::ChannelError) -> Self {
		Self::ChannelError
	}
}

impl From<librespot::core::audio_key::AudioKeyError> for SpotifyError {
	fn from(_: librespot::core::audio_key::AudioKeyError) -> Self {
		Self::AudioKeyError
	}
}

impl From<tokio::task::JoinError> for SpotifyError {
	fn from(_: tokio::task::JoinError) -> Self {
		Self::JoinError
	}
}

impl From<aspotify::Error> for SpotifyError {
	fn from(e: aspotify::Error) -> Self {
		Self::ASpotify(e.to_string())
	}
}

impl From<serde_json::Error> for SpotifyError {
	fn from(e: serde_json::Error) -> Self {
		Self::Serde(e.to_string(), e.line(), e.column())
	}
}

impl From<url::ParseError> for SpotifyError {
	fn from(e: url::ParseError) -> Self {
		Self::ParseError(e)
	}
}

impl From<id3::Error> for SpotifyError {
	fn from(e: id3::Error) -> Self {
		Self::ID3Error(e.kind.to_string(), e.description.to_string())
	}
}

impl From<reqwest::Error> for SpotifyError {
	fn from(e: reqwest::Error) -> Self {
		Self::Reqwest(e.to_string())
	}
}

impl From<lewton::VorbisError> for SpotifyError {
	fn from(e: lewton::VorbisError) -> Self {
		SpotifyError::Error(format!("Lewton: {e}"))
	}
}
