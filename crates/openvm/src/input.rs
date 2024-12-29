use strata_zkvm::{AggregationInput, ZkVmInputBuilder, ZkVmInputResult};

use crate::proof::OpenVmProofReceipt;
use openvm_sdk::StdIn;

// A wrapper around StdIn
pub struct OpenVmInputBuilder(StdIn);

impl ZkVmInputBuilder<'_> for OpenVmInputBuilder {
    type Input = StdIn;
    type ZkVmProofReceipt = OpenVmProofReceipt;

    fn new() -> Self {
        OpenVmInputBuilder(StdIn::default())
    }

    fn write_serde<T: serde::Serialize>(&mut self, item: &T) -> ZkVmInputResult<&mut Self> {
        self.0.write(item);
        Ok(self)
    }

    fn write_borsh<T: borsh::BorshSerialize>(&mut self, item: &T) -> ZkVmInputResult<&mut Self> {
        let slice = borsh::to_vec(item)?;
        self.write_buf(&slice)
    }

    fn write_buf(&mut self, item: &[u8]) -> ZkVmInputResult<&mut Self> {
        self.0.write_bytes(item);
        Ok(self)
    }

    fn write_proof(&mut self, _item: &AggregationInput) -> ZkVmInputResult<&mut Self> {
        panic!("Proof composition is not yet supported for OpenVM");
    }

    fn build(&mut self) -> ZkVmInputResult<Self::Input> {
        Ok(self.0.clone())
    }
}
