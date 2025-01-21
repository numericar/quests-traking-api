use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Associations)]
#[diesel(table_name = quest_adventurer_junction)]
pub struct QuestAdventurerJunction {

}