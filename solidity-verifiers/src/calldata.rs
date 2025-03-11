use crate::utils::eth::Soliditify;
use ark_bn254::Bn254;
use ark_groth16::Groth16;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use folding_schemes::commitment::kzg::KZG;
use folding_schemes::folding::nova::decider_eth::Proof;
use folding_schemes::folding::nova::CommittedInstance;
use folding_schemes::Error;
use num_bigint::BigUint;

/// Specifies which API to use for a proof verification in a contract.
#[derive(Copy, Clone, Debug, Default)]
pub enum NovaVerificationMode {
    /// Use the `verifyNovaProof` function.
    #[default]
    Explicit,
    /// Use the `verifyOpaqueNovaProof` function.
    Opaque,
    /// Use the `verifyOpaqueNovaProofWithInputs` function.
    OpaqueWithInputs,
}

/// Formats call data from a vec of bytes to a hashmap
/// Useful for debugging directly on the EVM
/// !! Should follow the contract's function signature, we assume the order of arguments is correct
pub fn get_formatted_calldata(calldata: Vec<u8>) -> Vec<String> {
    // let mut formatted_calldata = vec![];
    // for i in (4..calldata.len()).step_by(32) {
    //     let val = BigUint::from_bytes_be(&calldata[i..i + 32]);
    //     formatted_calldata.push(format!("{}", val));
    // }
    // formatted_calldata
    vec![]
}

macro_rules! generate_contract_interface {
    ($state_len:ident) => {
        alloy_sol_types::sol! {
            // #[derive(Debug)]
            // function verifyNovaProof(
            //     uint256[{2 * $state_len + 1}] calldata i_z0_zi,
            //     uint256[4] calldata U_i_cmW_U_i_cmE,
            //     uint256[2] calldata u_i_cmW,
            //     uint256[3] calldata cmT_r,
            //     uint256[2] calldata pA,
            //     uint256[2][2] calldata pB,
            //     uint256[2] calldata pC,
            //     uint256[4] calldata challenge_W_challenge_E_kzg_evals,
            //     uint256[2][2] calldata kzg_proof
            // ) public view returns (bool);
            //
            // #[derive(Debug)]
            // function verifyOpaqueNovaProofWithInputs(
            //     uint256 steps,
            //     uint256[{$state_len}] calldata initial_state,
            //     uint256[{$state_len}] calldata final_state,
            //     uint256[25] calldata proof
            // ) public override view returns (bool);

            // #[derive(Debug)]
            function verifyOpaqueNovaProof(
                uint256[$state_len] calldata proof
            ) public override view returns (bool);
        }
    }
}

// pub fn _prepare_calldata_for_nova_cyclefold_verifier(
//     verification_mode: NovaVerificationMode,
//     i: ark_bn254::Fr,
//     z_0: Vec<ark_bn254::Fr>,
//     z_i: Vec<ark_bn254::Fr>,
//     running_instance: &CommittedInstance<ark_bn254::G1Projective>,
//     incoming_instance: &CommittedInstance<ark_bn254::G1Projective>,
//     proof: &Proof<ark_bn254::G1Projective, KZG<Bn254>, Groth16<Bn254>>,
// ) -> Result<Vec<u8>, Error> {
//     let call = verifyNovaProofCall {
//         i_z0_zi: [],
//         U_i_cmW_U_i_cmE: [],
//         u_i_cmW: [],
//         cmT_r: [],
//         pA: [],
//         pB: [],
//         pC: [],
//         challenge_W_challenge_E_kzg_evals: [],
//         kzg_proof: [],
//     }
// }

/// Prepares solidity calldata for calling the NovaDecider contract
pub fn prepare_calldata_for_nova_cyclefold_verifier<const STATE_LEN: usize>(
    verification_mode: NovaVerificationMode,
    i: ark_bn254::Fr,
    z_0: Vec<ark_bn254::Fr>,
    z_i: Vec<ark_bn254::Fr>,
    running_instance: &CommittedInstance<ark_bn254::G1Projective>,
    incoming_instance: &CommittedInstance<ark_bn254::G1Projective>,
    proof: &Proof<ark_bn254::G1Projective, KZG<Bn254>, Groth16<Bn254>>,
) -> Result<Vec<u8>, Error> {
    generate_contract_interface!(STATE_LEN);

    Ok([
        // i.soliditify(),   // i
        // z_0.soliditify(), // z_0
        // z_i.soliditify(), // z_i
        // running_instance.cmW.soliditify(),
        // running_instance.cmE.soliditify(),
        // incoming_instance.cmW.soliditify(),
        // proof.cmT().soliditify(),                 // cmT
        // proof.r().to_eth(),                   // r
        // proof.snark_proof().soliditify(),         // pA, pB, pC
        // proof.kzg_challenges().soliditify(),      // challenge_W, challenge_E
        // proof.kzg_proofs()[0].eval.to_eth(),  // eval W
        // proof.kzg_proofs()[1].eval.to_eth(),  // eval E
        // proof.kzg_proofs()[0].proof.soliditify(), // W kzg_proof
        // proof.kzg_proofs()[1].proof.soliditify(), // E kzg_proof
    ]
    .concat())
}
