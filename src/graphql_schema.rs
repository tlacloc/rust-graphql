use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};

use crate::db::PgPool;

use crate::schema::members;

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

#[derive(GraphQLObject, Queryable, Debug)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Member {
    id: i32,
    name: String,
    knockouts: i32,
    team_id: i32,
}

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

#[derive(GraphQLInputObject, Insertable, Debug)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
#[table_name = "members"]
struct NewMember {
    name: String,
    knockouts: i32,
    team_id: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object( Context = Context)]
impl QueryRoot {
    fn members(context: &Context) -> FieldResult<Vec<Member>> {
        use crate::schema::members::dsl::*;
        let conn = crate::establish_connection();
        let member = members
            .limit(100)
            .load::<Member>(&conn)
            .expect("Error loading members");
        Ok(member)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn create_member(new_member: NewMember, context: &Context) -> FieldResult<Member> {
        use crate::schema::members::dsl::*;
        let conn = context.db.get().unwrap();
        let member = diesel::insert_into(members)
            .values(&new_member)
            .get_result::<Member>(&conn)
            .expect("Error saving new member");
        Ok(member)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
