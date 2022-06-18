use crate::actix::Addr;
use crate::actors::db::DbActor;
use crate::schema::words;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

pub struct AppState {
  pub db: Addr<DbActor>,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Word {
  pub id: i32,
  pub word: String,
  pub definition: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "words"]
pub struct NewWord {
  pub word: String,
  pub definition: String,
}

#[derive(Serialize, Deserialize)]
pub struct WordData {
  pub word: String,
  pub definition: String,
}
