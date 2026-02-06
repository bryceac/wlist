use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Content {
    Items,
    Notes
}