use crate::graphql_representation::{Enum, Model};
use colored::Colorize;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./prisma.pest"]
pub struct PrismaParser;

pub fn parse_prisma_file(unparsed_file: String) -> (Vec<Model>, Vec<Enum>) {
    let file = PrismaParser::parse(Rule::file, &unparsed_file);

    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            println!("{}", "💔 Can't parse prisma file".yellow());
            panic!("{}", e)
        }
    };

    let mut models = Vec::new();
    let mut enums = Vec::new();

    let file = file.next().unwrap();
    for statement in file.into_inner() {
        match statement.as_rule() {
            Rule::model_statement => models.push(Model::from(statement)),
            Rule::enum_statement => enums.push(Enum::from(statement)),
            _ => {}
        }
    }

    (models, enums)
}
