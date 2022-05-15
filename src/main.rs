mod cli;
mod generator;
mod graphql_representation;
mod parser;
mod type_resolver;

use colored::Colorize;

use crate::cli::Args;
use crate::generator::generate_gql_schema;
use crate::graphql_representation::{Enum, Model};
use crate::parser::parse_prisma_file;

use std::fs;

use clap::Parser;

fn main() {
    let args = Args::parse();
    let unparsed_file = fs::read_to_string(&args.prisma_file);

    let unparsed_file = match unparsed_file {
        Ok(file) => file,
        Err(e) => {
            println!(
                "{} {}",
                "💔 Can't read prisma file in path:".yellow(),
                args.prisma_file.cyan(),
            );

            panic!("{}", e)
        }
    };

    println!("✅ Got prisma file...");
    let (models, enums) = parse_prisma_file(unparsed_file);
    println!("✅ Parsed prisma file...");
    let gql_spec_content = generate_gql_schema(&models, &enums);
    println!("✅ Generated QGL schema content...");
    fs::write(args.output_gql, gql_spec_content).unwrap();
    println!("🥂 Done!");
}
