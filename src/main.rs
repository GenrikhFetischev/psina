mod graphql_representation;
mod type_resolver;
mod cli;
mod parser;
mod generator;

#[macro_use]
extern crate pest_derive;


use std::fs;
use crate::cli::Args;
use crate::generator::generate_gql_schema;
use crate::graphql_representation::{Enum, Model};
use crate::parser::parse_prisma_file;


use clap::Parser;


fn main() {
  let args = Args::parse();
  let unparsed_file = fs::read_to_string(&args.prisma_file);
  if let Err(e) = unparsed_file {
    println!("💔 \x1b[93mCan't read prisma file in path: {}\x1b[0m", args.prisma_file);
    panic!("{}", e)
  }

  println!("✅  Got prisma file...");
  let (models, enums) = parse_prisma_file(unparsed_file.unwrap());
  println!("✅  Parsed prisma file...");
  let gql_spec_content = generate_gql_schema(&models, &enums);
  println!("✅  Generated QGL schema content...");
  fs::write(args.output_gql, gql_spec_content).unwrap();
  println!("🥂 Done!");
}


