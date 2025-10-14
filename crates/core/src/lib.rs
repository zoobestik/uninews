#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod news;
pub use news::LiveNewsService;
pub use news::NewsService;

mod http;
pub use http::HttpService;
pub use http::LiveHttpService;
