use strata_risc0_adapter::Risc0ZkVmEnv;
use zeth::process_block;

fn main() {
    process_block(&Risc0ZkVmEnv)
}
