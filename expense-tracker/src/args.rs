use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        amount: i64,
    },
    List,
    Summary {
        #[arg(short, long)]
        month: Option<u32>,
    },
    Delete {
        #[arg(short, long)]
        id: i64,
    },
}
