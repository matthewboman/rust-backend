//! # Tracks

/// Service
pub mod service;

/// Model
pub mod model;

/// GraphQL Queries
pub mod queries;

/// GraphQL Mutations
pub mod mutations;

/// GraphQL Resolver
pub mod resolver;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("tracks/authorization.polar");
// 
// /// Tests
// #[cfg(test)]
// mod tests;
