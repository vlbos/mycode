#![allow(non_snake_case)]
use curve25519_dalek::scalar::Scalar;
use libspartan::{
    ComputationCommitment, InputsAssignment, Instance, SNARKGens, VarsAssignment, SNARK,
};
use merlin::Transcript;

// Input:
// balance_initial: initial balance.
// balance_new: new balance.
// payment_amount: payment amount.
// Return:
// Meta information such as constraints, variables, and input quantity.
// Instance: R1CS instance, representing the constraint system.
// VarsAssignment and InputsAssignment: specific assignments of variables and inputs, respectively.
fn produce_balance_r1cs(
    balance_initial: u64,
    balance_new: u64,
    payment_amount: u64,
) -> (
    usize,
    usize,
    usize,
    usize,
    Instance,
    VarsAssignment,
    InputsAssignment,
) {
    // We will create a constraint system to verify:
    // B - P = B'

    // parameters of the R1CS instance rounded to the nearest power of two
let num_cons = 2; // number of constraints
let num_vars = 3; // number of variables: B, P, temp
let num_inputs = 1; // number of inputs: B'
let num_non_zero_entries = 4; // total number of non-zero entries in the sparse matrix

    // Create sparse matrices A, B, C
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    // Use scalar fields from curve25519-dalek
    let one = Scalar::ONE.to_bytes();
    let minus_one = -Scalar::ONE;
    // constraint 0 entries in (A,B,C)
    // B - P = temp
A.push((0, 0, one)); // A[0][0] = 1, indicating that the coefficient of B is 1
A.push((0, 1, minus_one.to_bytes())); // A[0][1] = -1, indicating that the coefficient of -P is -1
B.push((0, num_vars, one)); // B[0][3] = 1, corresponding to temp
C.push((0, 2, one)); // C[0][2] = 1, the target is temp
    // temp // constraint 1 entries in (A,B,C)
    // temp - B' = 0
A.push((1, 2, one)); // A[1][2] = 1, indicating that the coefficient of temp is 1
B.push((1, num_vars, one)); // B[1][3] = 1
C.push((1, num_inputs + num_vars, one)); // C[1][4] = 1, indicating that the target B'
    // B'
    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();
    // Calculate the assignment that satisfies the condition
    let B = Scalar::from(balance_initial);
    let P = Scalar::from(balance_new);
    let temp = B - P; // temp = B - P
    let B_prime = Scalar::from(payment_amount);

    // Create variable assignments
    let mut vars = vec![Scalar::ZERO.to_bytes(); num_vars];
    vars[0] = B.to_bytes();
    vars[1] = P.to_bytes();
    vars[2] = temp.to_bytes();
    let assignment_vars = VarsAssignment::new(&vars).unwrap();

    // Create input assignments for generating proofs
    let mut witness_inputs = vec![Scalar::ZERO.to_bytes(); num_inputs];
    witness_inputs[0] = B_prime.to_bytes();
    let assignment_witness_inputs = InputsAssignment::new(&witness_inputs).unwrap();

    // Check if the instance we created satisfies
    let res = inst
        .is_sat(&assignment_vars, &assignment_witness_inputs)
        .unwrap();
    assert!(res);

    (
        num_cons,
        num_vars,
        num_inputs,
        num_non_zero_entries,
        inst,
        assignment_vars,
        assignment_witness_inputs,
    )
}


fn prover_snark() -> (SNARK, SNARKGens, ComputationCommitment) {
    // Initial balance B
    let balance_initial: u64 = 100;

    // Payment amount P
    let payment_amount: u64 = 40;

    // New balance after payment B'
    let balance_new: u64 = balance_initial - payment_amount;

    // Generate R1CS instance and its constraints
    let (
        num_cons,
        num_vars,
        num_inputs,
        num_non_zero_entries,
        inst,
        assignment_vars,
        assignment_witness_inputs,
    ) = produce_balance_r1cs(balance_initial, balance_new, payment_amount);

    // Generate public parameters
    let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_non_zero_entries);

    // Create a commitment to an R1CS instance
    let (comm, decomm) = SNARK::encode(&inst, &gens);

    // Generate proof
    let mut prover_transcript = Transcript::new(b"balance_verification");
    (
        SNARK::prove(
            &inst,
            &comm,
            &decomm,
            assignment_vars,
            &assignment_witness_inputs, // Generate proof using witness inputs
            &gens,
            &mut prover_transcript,
        ),
        gens,
        comm,
    )
}

fn verifier_snark(proof: SNARK, gens: SNARKGens, comm: ComputationCommitment) {
    // Payment amount P
    let payment_amount: u64 = 40;
    let num_inputs = 1; // B'
    let mut public_input = vec![Scalar::ZERO.to_bytes(); num_inputs];
    // Public input for verification
    public_input[0] = Scalar::from(payment_amount).to_bytes();
    let assignment_public_inputs = InputsAssignment::new(&public_input).unwrap();

    // Verification proof
    let mut verifier_transcript = Transcript::new(b"balance_verification");
    assert!(proof
        .verify(
            &comm,
            &assignment_public_inputs,
            &mut verifier_transcript,
            &gens
        )
        .is_ok());

    println!("proof verification successful!");
}
fn main() {
    let (proof, gens, comm) = prover_snark();
    verifier_snark(proof, gens, comm);
}
