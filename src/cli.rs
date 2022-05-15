use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
  #[clap(short, long)]
  pub prisma_file: String,
  #[clap(short, long)]
  pub output_gql: String,

  #[clap(short, long, default_value_t = 1)]
  count: u8,
}
