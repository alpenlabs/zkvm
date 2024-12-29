use std::{fmt, sync::Arc};

use openvm_sdk::{config::SdkVmConfig, prover::AppProver, Sdk};
use strata_zkvm::ZkVmHost;

use crate::{input::OpenVmInputBuilder, proof::OpenVmProofReceipt};

use openvm::platform::memory::MEM_SIZE;
use openvm_sdk::config::AppConfig;
use openvm_stark_sdk::config::FriParameters;
use openvm_transpiler::elf::Elf;

/// A host for the `OpenVm` zkVM that stores the guest program in ELF format
/// The `OpenVm` is responsible for program execution and proving
#[derive(Clone)]
pub struct OpenVm {
    elf: Vec<u8>,
}

impl fmt::Display for OpenVm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OpenVm")
    }
}

impl OpenVm {
    pub fn init(guest_code: &[u8]) -> Self {
        OpenVm {
            elf: guest_code.to_vec(),
        }
    }
}

impl ZkVmHost for OpenVm {
    type Input<'a> = OpenVmInputBuilder;
    type ZkVmProofReceipt = OpenVmProofReceipt;

    fn prove_inner<'a>(
        &self,
        input: <Self::Input<'a> as strata_zkvm::ZkVmInputBuilder<'a>>::Input,
        proof_type: strata_zkvm::ProofType,
    ) -> strata_zkvm::ZkVmResult<Self::ZkVmProofReceipt> {
        let elf = Elf::decode(&self.elf, MEM_SIZE as u32).unwrap();
        let sdk = Sdk;

        let vm_config = SdkVmConfig::builder()
            .system(Default::default())
            .rv32i(Default::default())
            .rv32m(Default::default())
            .io(Default::default())
            .build();

        // Transpile the ELF into a VmExe
        let exe = sdk.transpile(elf, vm_config.transpiler()).unwrap();

        // Run the program
        let output = sdk
            .execute(exe.clone(), vm_config.clone(), input.clone())
            .unwrap();
        println!("public values output: {:?}", output);

        // Set app configuration
        let app_log_blowup = 2;
        let app_fri_params =
            FriParameters::standard_with_100_bits_conjectured_security(app_log_blowup);
        let app_config = AppConfig::new(app_fri_params, vm_config);

        // 7. Commit the exe
        let app_committed_exe = sdk.commit_app_exe(app_fri_params, exe).unwrap();

        // Generate an AppProvingKey
        let app_pk = Arc::new(sdk.app_keygen(app_config).unwrap());

        // 9a. Generate a proof
        let proof = sdk
            .generate_app_proof(app_pk.clone(), app_committed_exe.clone(), input.clone())
            .unwrap();

        todo!()
    }

    fn get_verification_key(&self) -> strata_zkvm::VerificationKey {
        todo!()
    }

    fn extract_serde_public_output<T: serde::Serialize + serde::de::DeserializeOwned>(
        public_values: &strata_zkvm::PublicValues,
    ) -> strata_zkvm::ZkVmResult<T> {
        todo!()
    }

    fn verify_inner(&self, proof: &Self::ZkVmProofReceipt) -> strata_zkvm::ZkVmResult<()> {
        todo!()
    }

    fn prove<'a>(
        &self,
        input: <Self::Input<'a> as strata_zkvm::ZkVmInputBuilder<'a>>::Input,
        proof_type: strata_zkvm::ProofType,
    ) -> strata_zkvm::ZkVmResult<strata_zkvm::ProofReceipt> {
        todo!()
    }

    fn extract_borsh_public_output<T: borsh::BorshDeserialize>(
        public_values: &strata_zkvm::PublicValues,
    ) -> strata_zkvm::ZkVmResult<T> {
        todo!()
    }

    fn verify(&self, proof: &strata_zkvm::ProofReceipt) -> strata_zkvm::ZkVmResult<()> {
        todo!()
    }
}
