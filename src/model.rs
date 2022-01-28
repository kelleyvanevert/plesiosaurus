use async_graphql::*;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

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

    async fn subscriptions(&self, ctx: &Context<'_>) -> Result<Vec<Subscription>> {
        let db_pool = ctx.data::<PgPool>()?;

        let subscriptions = sqlx::query_as!(
            Subscription,
            r#"
                select s.*
                from subscriptions as s
            "#
        )
        .fetch_all(db_pool)
        .await?;

        Ok(subscriptions)
    }
}

pub struct RootValue;

impl RootValue {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Subscription {
    id: Uuid,
    email: String,
    name: String,
    subscribed_at: chrono::DateTime<Utc>,
}

#[Object]
/// Does this also work?
impl Subscription {
    async fn id(&self) -> Uuid {
        self.id
    }

    // Test that normal comments don't work
    async fn email(&self) -> String {
        self.email.clone()
    }

    /// You can add graphql docs like so
    #[graphql(name = "subscriberName")]
    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn subscribed_at(&self) -> chrono::DateTime<Utc> {
        self.subscribed_at
    }
}
