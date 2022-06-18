use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::diesel::prelude::*;
use crate::models::{NewWord, Word};
use crate::schema::words::dsl::*;
use diesel::{
  r2d2::{ConnectionManager, Pool},
  PgConnection,
};

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Message)]
#[rtype(result = "QueryResult<Word>")]
pub struct GetWord {
  pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Word>")]
pub struct CreateWord {
  pub word: String,
  pub definition: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Word>")]
pub struct UpdateWord {
  pub id: i32,
  pub word: String,
  pub definition: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Word>")]
pub struct DeleteWord {
  pub id: i32,
}

impl Actor for DbActor {
  type Context = SyncContext<Self>;
}

impl Handler<GetWord> for DbActor {
  type Result = QueryResult<Word>;

  fn handle(&mut self, msg: GetWord, _ctx: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    words.filter(id.eq(msg.id)).get_result(&conn)
  }
}

impl Handler<CreateWord> for DbActor {
  type Result = QueryResult<Word>;

  fn handle(&mut self, msg: CreateWord, _ctx: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get connection");
    let new_word = NewWord {
      word: msg.word,
      definition: msg.definition,
    };
    diesel::insert_into(words)
      .values(new_word)
      .get_result::<Word>(&conn)
  }
}

impl Handler<UpdateWord> for DbActor {
  type Result = QueryResult<Word>;

  fn handle(&mut self, msg: UpdateWord, _ctx: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    diesel::update(words)
      .filter(id.eq(msg.id))
      .set((word.eq(msg.word), definition.eq(msg.definition)))
      .get_result::<Word>(&conn)
  }
}

impl Handler<DeleteWord> for DbActor {
  type Result = QueryResult<Word>;

  fn handle(&mut self, msg: DeleteWord, _ctx: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    diesel::delete(words)
      .filter(id.eq(msg.id))
      .get_result::<Word>(&conn)
  }
}
