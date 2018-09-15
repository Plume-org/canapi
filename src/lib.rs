extern crate serde;

use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Display;

pub enum Error {
    Fetch(String),
    SerDe(String),
    NotFound(String),
    Authorization(String),
}

/// API to DB link
/// 
/// `E` is the endpoint (API)
/// `P` is the provider (DB)
pub trait Provider<P> {
    type Data: Endpoint;

    /// Get a single result by ID
    fn get(provider: &P, id: <Self::Data as Endpoint>::Id) -> Result<Self::Data, Error>;

    /// List all matching results
    fn list(provider: &P, query: Self::Data) -> Vec<Self::Data>;

    /// Save a new object, and returns it
    fn create(provider: &P, query: Self::Data) -> Result<Self::Data, Error>;

    /// Update an object
    fn update(provider: &P, id: <Self::Data as Endpoint>::Id, new_data: Self::Data) -> Result<Self::Data, Error>;

    /// Delete an object
    fn delete(provider: &P, id: <Self::Data as Endpoint>::Id);
}

/// API Endpoint, common to the server and the front
pub trait Endpoint: Default + Serialize + DeserializeOwned {
    type Id: Display;

    /// The URL on which this endpoint is mounted.
    /// 
    /// It should start with a /, and end without.
    fn endpoint() -> &'static str;

    fn get<F: Fetch>(&self, id: Self::Id) -> Result<Self, Error> {
        F::fetch("GET", format!("{}/{}", Self::endpoint(), id), None)
    }

    fn list<F: Fetch>(&self) -> Result<Self, Error> {
        F::fetch("GET", format!("{}", Self::endpoint()), None)
    }

    fn find<F: Fetch>(&self, query: Self) -> Result<Self, Error> {
        F::fetch("GET", format!("{}", Self::endpoint()), Some(query))
    }

    fn create<F: Fetch>(&self, new: Self) -> Result<Self, Error> {
        F::fetch("POST", format!("{}", Self::endpoint()), Some(new))
    }

    fn update<F: Fetch>(&self, id: Self::Id, data: Self) -> Result<Self, Error> {
        F::fetch("PUT", format!("{}/{}", Self::endpoint(), id), Some(data))
    }
    
    fn delete<F: Fetch>(&self, id: Self::Id) -> Result<Self, Error> {
        F::fetch("DELETE", format!("{}/{}", Self::endpoint(), id), None)
    }
}

/// Anything that can perform a network request to fetch an endpoint
pub trait Fetch {

    /// Fetch a given endpoint
    fn fetch<T: Endpoint>(method: &'static str, url: String, query: Option<T>) -> Result<T, Error>;    
}
