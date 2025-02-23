use super::Prover;
use crate::{
    io::{load_snark, write_snark},
    utils::gen_rng,
};
use aggregator::CompressionCircuit;
use anyhow::{anyhow, Result};
use rand::Rng;
use snark_verifier_sdk::Snark;
use std::env::set_var;

impl Prover {
    pub fn gen_comp_snark(
        &mut self,
        id: &str,
        is_fresh: bool,
        degree: u32,
        mut rng: impl Rng + Send,
        prev_snark: Snark,
    ) -> Result<Snark> {
        set_var("COMPRESSION_CONFIG", format!("./configs/{id}.config"));

        let circuit = CompressionCircuit::new(self.params(degree), prev_snark, is_fresh, &mut rng)
            .map_err(|err| anyhow!("Failed to construct compression circuit: {err:?}"))?;

        self.gen_snark(id, degree, &mut rng, circuit)
    }

    pub fn load_or_gen_comp_snark(
        &mut self,
        name: &str,
        id: &str,
        is_fresh: bool,
        degree: u32,
        prev_snark: Snark,
        output_dir: Option<&str>,
    ) -> Result<Snark> {
        let file_path = format!(
            "{}/compression_snark_{}_{}.json",
            output_dir.unwrap_or_default(),
            id,
            name
        );

        match output_dir.and_then(|_| load_snark(&file_path).ok().flatten()) {
            Some(snark) => Ok(snark),
            None => {
                let rng = gen_rng();
                let result = self.gen_comp_snark(id, is_fresh, degree, rng, prev_snark);
                if let (Some(_), Ok(snark)) = (output_dir, &result) {
                    write_snark(&file_path, snark);
                }

                result
            }
        }
    }
}
