use serde::{Deserialize, Serialize};

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug)]
pub enum ClientError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_path_to_error::Error<serde_json::Error>),
    ServerError(ServerError),
    UrlParseError(url::ParseError),
    ErrorMessage(ErrorMessage),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClientError::ReqwestError(err) => write!(f, "ReqwestError: {}", err),
            ClientError::SerdeJsonError(err) => write!(f, "SerdeJsonError: {}", err),
            ClientError::ServerError(err) => write!(f, "ServerError: {:?}", err),
            ClientError::UrlParseError(err) => write!(f, "UrlParseError: {}", err),
            ClientError::ErrorMessage(err) => write!(f, "ErrorMessage: {}", err.0),
        }
    }
}

impl std::error::Error for ClientError {}

#[derive(Debug)]
pub struct ErrorMessage(pub String);

impl ClientError {
    pub fn error_message(msg: String) -> Self {
        ClientError::ErrorMessage(ErrorMessage(msg))
    }
    pub fn server_error(errcode: i64, errmsg: String) -> Self {
        ClientError::ServerError(ServerError { errcode, errmsg })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerError {
    pub errcode: i64,
    pub errmsg: String,
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::ReqwestError(err)
    }
}

impl From<ServerError> for ClientError {
    fn from(err: ServerError) -> Self {
        ClientError::ServerError(err)
    }
}

impl From<url::ParseError> for ClientError {
    fn from(err: url::ParseError) -> Self {
        ClientError::UrlParseError(err)
    }
}

impl From<serde_path_to_error::Error<serde_json::Error>> for ClientError {
    fn from(err: serde_path_to_error::Error<serde_json::Error>) -> Self {
        ClientError::SerdeJsonError(err)
    }
}

macro_rules! enum_int_str {
    ($name:ident { $($variant:ident($int:expr, $str:expr), )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                serializer.serialize_str(match *self {
                    $( $name::$variant => $str, )*
                })
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, "a string for {}", stringify!($name))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(::serde::de::Unexpected::Other(
                                &format!("unknown {} variant: {}", stringify!($name), value)
                            ), &self)),
                        }
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $str.to_string(), )*
                }
            }
        }

        impl TryFrom<$name> for i64 {
            type Error = crate::client::types::ClientError;

            fn try_from(value: $name) -> crate::client::types::ClientResult<Self> {
                match value {
                    $( $name::$variant => Ok($int), )*
                }
            }
        }

        impl TryFrom<i64> for $name {
            type Error = crate::client::types::ClientError;

            fn try_from(value: i64) -> crate::client::types::ClientResult<Self> {
                match value {
                    $( $int => Ok($name::$variant), )*
                    _ => Err(crate::client::types::ClientError::error_message(format!("unknown {} variant: {}", stringify!($name), value))),
                }
            }
        }
    }
}

#[allow(unused)]
macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                serializer.serialize_str(match *self {
                    $( $name::$variant => $str, )*
                })
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, "a string for {}", stringify!($name))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(::serde::de::Unexpected::Other(
                                &format!("unknown {} variant: {}", stringify!($name), value)
                            ), &self)),
                        }
                    }
                }

                // 从字符串反序列化枚举。
                deserializer.deserialize_str(Visitor)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $str.to_string(), )*
                }
            }
        }
    }
}

pub(crate) use enum_int_str;
#[allow(unused)]
pub(crate) use enum_str;
