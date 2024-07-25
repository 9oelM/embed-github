use worker::{Method, Response};

const GH_REF: &str = "See https://github.com/9oelM/embed-github for usage.";

#[derive(thiserror::Error, Debug)]
pub enum HighlightCodeError {
    #[error("Invalid highlight theme: {0}")]
    InvalidTheme(String),
}

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("Failed to construct url from {method:?} {path}")]
    ReqToUrl { method: Method, path: String },
    #[error("Empty querystring")]
    EmptyQuerystring,
}

#[derive(thiserror::Error, Debug)]
pub enum OptionsBuilderError {
    #[error("Missing 'gh' query parameter")]
    MissingGhQuery,
    #[error(transparent)]
    RequestedSourceInfo(RequestedSourceInfoError),
    #[error(transparent)]
    ConvertGithubPermalinkToRawGithubUserContent(ConvertGithubPermalinkToRawGithubUserContentError),
}

#[derive(thiserror::Error, Debug)]
pub enum RequestedSourceInfoError {
    #[error("Invalid URL from 'gh' query parameter: {0}")]
    InvalidGhQueryUrl(String),
    #[error("Failed to parse domain from 'gh' query parameter: {0}")]
    ParseDomain(String),
    #[error("Domain of URL from 'gh' query parameter is not github.com: {0}")]
    DomainNotGithub(String),
    #[error(transparent)]
    DecodeLineRange(DecodeLineRangeError),
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeLineRangeError {
    #[error("Failed to parse line number from {fragment}")]
    ParseLineNumber { fragment: String },
    #[error("Need starting number <= ending number but received {start} and {end}")]
    LineStartBiggerThanEnd { start: u64, end: u64 },
}

#[derive(thiserror::Error, Debug)]
pub enum GetSourceError {
    #[error("Provided line number {number} is out of range of the source code")]
    LineNumberOutOfRange { number: u64 },
    #[error(transparent)]
    FetchSource(worker::Error),
    #[error("Failed to fetch source code from {url}. Status code: {status_code}")]
    FetchSourceStatusCode { url: String, status_code: u16 },
}

#[derive(thiserror::Error, Debug)]
pub enum ConvertGithubPermalinkToRawGithubUserContentError {
    #[error("Provided URL {0} is too short. It's probably not a valid Github Permalink URL")]
    PathTooShort(String),
    #[error(
        "Could not construct correct URL pointing to raw.githubusercontent.com. Composed: {url}"
    )]
    ComposeRawCodeUrl { url: String },
}

impl From<HighlightCodeError> for worker::Result<Response> {
    fn from(err: HighlightCodeError) -> Self {
        Response::error(format!("{}. {}", err, GH_REF), 400)
    }
}

impl From<RequestError> for worker::Result<Response> {
    fn from(err: RequestError) -> Self {
        Response::error(format!("{}. {}", err, GH_REF), 400)
    }
}

impl From<OptionsBuilderError> for worker::Result<Response> {
    fn from(err: OptionsBuilderError) -> Self {
        Response::error(format!("{}. {}", err, GH_REF), 400)
    }
}

impl From<RequestedSourceInfoError> for worker::Result<Response> {
    fn from(err: RequestedSourceInfoError) -> Self {
        Response::error(format!("{}. {}", err, GH_REF), 400)
    }
}

impl From<DecodeLineRangeError> for worker::Result<Response> {
    fn from(err: DecodeLineRangeError) -> Self {
        Response::error(format!("{}. {}", err, GH_REF), 400)
    }
}

impl From<GetSourceError> for worker::Result<Response> {
    fn from(err: GetSourceError) -> Self {
        Response::error(format!("{}. {}", err, GH_REF), 400)
    }
}

impl From<ConvertGithubPermalinkToRawGithubUserContentError> for worker::Result<Response> {
    fn from(err: ConvertGithubPermalinkToRawGithubUserContentError) -> Self {
        Response::error(format!("{}. {}", err, GH_REF), 400)
    }
}
