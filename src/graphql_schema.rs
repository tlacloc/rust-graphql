use diesel::prelude::*;

use juniper::{
    graphql_object, EmptySubscription, FieldResult, RootNode,
};

use crate::schema::{members, teams};
use crate::schemas::{
    member::{Member, MemberInput},
    team::{Team, TeamInput},
};

use crate::db::PgPool;

pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all members")]
    fn members(&self, context: &Context) -> FieldResult<Vec<Member>> {
        let conn = &context.db.get()?;
        let members = members::table.load::<Member>(conn)?;
        Ok(members)
    }

    #[graphql(description = "List of all teams")]
    fn teams(&self, context: &Context) -> FieldResult<Vec<Team>> {
        let conn = &context.db.get()?;
        let teams = teams::table.load::<Team>(conn)?;
        Ok(teams)
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    pub fn create_member(context: &Context, input: MemberInput) -> FieldResult<Member> {
        let conn = &context.db.get()?;
        let member = diesel::insert_into(members::table)
            .values(&input)
            .get_result::<Member>(conn)?;
        Ok(member)
    }

    pub fn create_team(context: &Context, input: TeamInput) -> FieldResult<Team> {
        let conn = &context.db.get()?;
        let team = diesel::insert_into(teams::table)
            .values(&input)
            .get_result::<Team>(conn)?;
        Ok(team)
    }

}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
