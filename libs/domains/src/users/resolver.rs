use async_graphql::{Context, Object, Result};
use hyper::StatusCode;
use std::sync::Arc;

use super::{
    model::User,
    mutations::{CreateUserInput, MutateUserResult},
    service::UsersServiceTrait
};
// use xor_auth::authenticate::Subject;
use xor_utils::errors::{as_graphql_error, graphql_error};

/// The Query segment for Users
#[derive(Default)] 
pub struct UsersQuery {}

/// The Mutation segment for Users
#[derive(Default)]
pub struct UsersMutation {}

/// Queries for the User model
#[Object]
impl UsersQuery {
    /// Get the current User from the GraphQL context
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let user = ctx.data_unchecked::<Option<User>>();

        Ok(user.clone())
    }
}

/// Mutations for User model
#[Object]
impl UsersMutation {
    /// Get or create the current User based on the current token username (the "sub" claim)
    async fn get_or_create_current_user(
        &self,
        ctx:   &Context<'_>,
        input: CreateUserInput
    ) -> Result<MutateUserResult> {
        let user    = ctx.data_unchecked::<Option<User>>();
        let users   = ctx.data_unchecked::<Arc<dyn UsersServiceTrait>>();

        // If the User exists in the GraphQL context, return it
        if let Some(user) = user {
            return Ok(MutateUserResult {
                user: Some(user.clone())
            });
        }

        let user = users.create(&input.email)
            .await
            .map_err(as_graphql_error("Error while creating User", StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(MutateUserResult {
            user: Some(user)
        })
    }

    // Update the current User based on the current token email (the "sub" claim)
    // async fn update_current_user(
    //     &self,
    //     ctx:   &Context<'_>,
    //     input: UpdateUserInput,
    // ) -> Result<MutateUserResult> {
    //     let user       = ctx.data_unchecked::<Option<User>>();
    //     let users      = ctx.data_unchecked::<Arc<dyn UsersServiceTrait>>();
    //     // let with_roles = ctx.look_ahead().field("user").field("roles").exists();

    //     if let Some(user) = user {
    //         let updated = users
    //             .update(&user.id, &input)
    //             // .update(&user.id, &input, &with_roles)
    //             .await
    //             .map_err(as_graphql_error("Error while updating User", StatusCode::INTERNAL_SERVER_ERROR))?;

    //         return Ok(MutateUserResult {
    //             user: Some(updated)
    //         })
    //     }

    //     Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED))
    // }
}
