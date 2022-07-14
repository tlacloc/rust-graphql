use juniper::GraphQLInputObject;

use diesel::prelude::*;

use crate::graphql_schema::Context;
use crate::schemas::member::Member;

use crate::schema::teams;

#[derive(Default, Queryable, Debug)]
pub struct Team {
  pub id: i32,
  pub name: String,
}

#[juniper::graphql_object(Context = Context)]
impl Team {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn members(&self, context: &Context) -> Vec<Member> {
    use crate::schema::members::dsl::*;
    let connection = context.db.get().unwrap();
    members
      .filter(team_id.eq(self.id))
      .limit(100)
      .load::<Member>(&connection)
      .expect("Error loading members")
  }
}

#[derive(GraphQLInputObject, Insertable, Debug)]
#[graphql(description = "Product Input")]
#[table_name = "teams"]
pub struct TeamInput {
    pub name: String,
}