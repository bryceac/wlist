use clap::Parser;

#[derive(Parser)]
pub struct Delete {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,
}