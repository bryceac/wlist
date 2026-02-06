use clap::Parser;
use wlitem::Priority;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>")]
pub struct Add {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub name: String,

    #[clap(long, short)]
    pub quantity: u32,

    #[clap(long, short, default_value = "medium")]
    pub priority: Priority,

    #[clap(long, short, default_value = "")]
    pub url: String,

    #[clap(long, num_args = 0.., value_delimiter = ',', required = false)]
    pub notes: Vec<String>
}