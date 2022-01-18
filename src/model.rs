use async_graphql::*;

pub type PlesioSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn hello(&self) -> &str {
        "Hello"
    }
}

pub struct RootValue;

impl RootValue {
    pub fn new() -> Self {
        Self {}
    }
}
