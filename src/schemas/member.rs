use crate::graphql_schema::Context;
use crate::schema::members;

use juniper::{graphql_object, GraphQLInputObject};

/// Member
#[derive(Default, Queryable, Debug)]
pub struct Member {
    id: i32,
    name: String,
    knockouts: i32,
    team_id: i32,
}

#[derive(GraphQLInputObject, Insertable, Debug)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
#[table_name = "members"]
pub struct MemberInput {
    name: String,
    knockouts: i32,
    team_id: i32,
}

#[graphql_object(Context = Context)]
impl Member {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn knockouts(&self) -> i32 {
        self.knockouts
    }

    pub fn team_id(&self) -> i32 {
        self.team_id
    }
}
