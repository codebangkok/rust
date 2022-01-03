pub mod error;
pub mod method;
pub mod query_string;
pub mod request;
pub mod response;
pub mod server;
pub mod status;

pub use error::Error;
pub use method::Method;
pub use query_string::QueryString;
pub use request::Request;
pub use response::Response;
pub use server::Server;
pub use status::HttpStatus;

pub type Result<T> = std::result::Result<T, Error>;
