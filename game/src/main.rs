use anyhow::Result;

pub fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    game::run().map_err(Into::into)
}
