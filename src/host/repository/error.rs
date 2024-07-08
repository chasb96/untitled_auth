use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use prost::DecodeError;
use redis::RedisError;
use sqlx::Error as SqlxError;
use mongodb::error::Error as MongoError;

#[derive(Debug)]
pub enum QueryError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
    RedisPool(PoolError<RedisError>),
    ProtobufDecode(DecodeError),
    Redis(RedisError),
    Mongo(MongoError),
    MongoPool(PoolError<MongoError>),
}

impl Error for QueryError { }

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::Sqlx(e) => write!(f, "Error running query: {}", e),
            QueryError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
            QueryError::RedisPool(e) => write!(f, "Error obtaining connection from redis pool: {}", e),
            QueryError::ProtobufDecode(e) => write!(f, "Error decoding protobuf: {}", e),
            QueryError::Redis(e) => write!(f, "Error accessing cache: {}", e),
            QueryError::Mongo(e) => write!(f, "Error accessing mongo: {}", e),
            QueryError::MongoPool(e) => write!(f, "Error obtaining connection from mongo pool: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for QueryError {
    fn from(value: PoolError<SqlxError>) -> Self {
        QueryError::Pool(value)
    }
}

impl From<PoolError<RedisError>> for QueryError {
    fn from(value: PoolError<RedisError>) -> Self {
        QueryError::RedisPool(value)
    }
}

impl From<SqlxError> for QueryError {
    fn from(value: SqlxError) -> Self {
        QueryError::Sqlx(value)
    }
}

impl From<DecodeError> for QueryError {
    fn from(value: DecodeError) -> Self {
        QueryError::ProtobufDecode(value)
    }
}

impl From<RedisError> for QueryError {
    fn from(value: RedisError) -> Self {
        QueryError::Redis(value)
    }
}

impl From<MongoError> for QueryError {
    fn from(value: MongoError) -> Self {
        QueryError::Mongo(value)
    }
}

impl From<PoolError<MongoError>> for QueryError {
    fn from(value: PoolError<MongoError>) -> Self {
        QueryError::MongoPool(value)
    }
}