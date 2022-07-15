use diesel::prelude::*;

use crate::graphql_schema::Context;
use crate::schema::teams;

use crate::schemas::member::Member;

use juniper::{graphql_object, GraphQLInputObject};

/// Team
#[derive(Default, Queryable, Debug)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[derive(GraphQLInputObject, Insertable, Debug)]
#[graphql(description = "Product Input")]
#[table_name = "teams"]
pub struct TeamInput {
    pub name: String,
}

#[graphql_object(Context = Context)]
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
