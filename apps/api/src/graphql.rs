use anyhow::Result;
use async_graphql::{
    dataloader::DataLoader,
    EmptySubscription,
    MergedObject,
    Schema,
};
use std::sync::Arc;

use crate::Context;
use xor_domains::users::{
    resolver::{UsersMutation, UsersQuery},
    service::UserLoader
};

/// The GraphQL top-level Query type
#[derive(MergedObject, Default)]
pub struct Query(UsersQuery);

/// The GraphQL top-level Mutation type
#[derive(MergedObject, Default)]
pub struct Mutation(UsersMutation);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// Initialize all necessary dependencies to create a `GraphQLSchema`. Very simple dependency
/// injection based on async-graphql's `.data()` calls.
pub fn create_schema(ctx: Arc<Context>) -> Result<GraphQLSchema> {
    // Instantiate loaders
    let user_loader = UserLoader::new(&ctx.users);

    // Inject the initialized services into the `Schema` instance.
    Ok(
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(ctx.config)
            .data(ctx.users.clone())
            .data(DataLoader::new(user_loader, tokio::spawn))
            .finish(),
    )
}
