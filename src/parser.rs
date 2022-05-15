extern crate pest;

use pest::Parser;
use crate::graphql_representation::{Enum, Model};


#[derive(Parser)]
#[grammar = "./prisma.pest"]
pub struct PrismaParser;


pub fn parse_prisma_file(unparsed_file: String) -> (Vec<Model>, Vec<Enum>) {
  

  let mut file = PrismaParser::parse(Rule::file, &unparsed_file).expect("can't parse");
  let mut models = Vec::new();
  let mut enums = Vec::new();


  let file = file.next().unwrap();
  for statement in file.into_inner() {
    match statement.as_rule() {
      Rule::model_statement => {
        models.push(Model::from(statement))
      }
      Rule::enum_statement => {
        enums.push(Enum::from(statement))
      }
      _ => {}
    }
  };

  (models, enums)
}
