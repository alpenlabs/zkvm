use strata_zkvm::{ProofReceipt, ZkVmProofError};

#[derive(Debug, Clone)]
pub struct OpenVmProofReceipt(Vec<u8>);

impl OpenVmProofReceipt {
    pub fn inner(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for OpenVmProofReceipt {
    fn from(receipt: Vec<u8>) -> Self {
        OpenVmProofReceipt(receipt)
    }
}

impl AsRef<[u8]> for OpenVmProofReceipt {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<ProofReceipt> for OpenVmProofReceipt {
    type Error = ZkVmProofError;
    fn try_from(value: ProofReceipt) -> Result<Self, Self::Error> {
        OpenVmProofReceipt::try_from(&value)
    }
}

impl TryFrom<&ProofReceipt> for OpenVmProofReceipt {
    type Error = ZkVmProofError;
    fn try_from(value: &ProofReceipt) -> Result<Self, Self::Error> {
        todo!() // Implementation needed
    }
}

impl TryFrom<OpenVmProofReceipt> for ProofReceipt {
    type Error = ZkVmProofError;
    fn try_from(value: OpenVmProofReceipt) -> Result<Self, Self::Error> {
        todo!() // Implementation needed
    }
}
