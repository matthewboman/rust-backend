use async_graphql::{InputObject, SimpleObject};

use super::model::User;

/// The `CreateUserInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct CreateUserInput {
    /// The Profile's email address
    pub email:        String

    // The Profile's display name
    // pub display_name: String,

    // // The Profiles picture URL
    // pub picture:      Option<String>
}

// The `UpdateUserInput` input type
// #[derive(Clone, Default, Eq, PartialEq, InputObject)]
// pub struct UpdateUserInput {
//     /// The User's email address
//     pub email:        String,

//     /// The User's display name
//     pub display_name: String,

//     // The User's picture URL
//     pub picture:      Option<String>
// }

/// The `MutateUserResult` input type
#[derive(Clone, Default, Eq, PartialEq, SimpleObject)]
pub struct MutateUserResult {
    /// The User's subscriber ID
    pub user: Option<User>,
}
