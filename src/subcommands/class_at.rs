use anyhow::Result;
use clap::Parser;
use colored_json::{ColorMode, Output};
use starknet::{
    core::types::{BlockId, BlockTag, FieldElement},
    providers::Provider,
};

use crate::ProviderArgs;

#[derive(Debug, Parser)]
pub struct ClassAt {
    #[clap(flatten)]
    provider: ProviderArgs,
    #[clap(help = "Contract address")]
    address: String,
}

impl ClassAt {
    pub async fn run(self) -> Result<()> {
        let provider = self.provider.into_provider();
        let address = FieldElement::from_hex_be(&self.address)?;

        // TODO: allow custom block
        let class = provider
            .get_class_at(BlockId::Tag(BlockTag::Latest), address)
            .await?;

        let class_json = serde_json::to_value(class)?;
        let class_json =
            colored_json::to_colored_json(&class_json, ColorMode::Auto(Output::StdOut))?;
        println!("{class_json}");

        Ok(())
    }
}
