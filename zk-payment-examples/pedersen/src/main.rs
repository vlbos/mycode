use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT, ristretto::RistrettoPoint, scalar::Scalar,
};
use rand::rngs::OsRng;

struct PedersenGens {
    g: RistrettoPoint,
    h: RistrettoPoint,
}

impl PedersenGens {
    fn new() -> Self {
        let mut rng = OsRng;
        let h = RistrettoPoint::random(&mut rng);

        PedersenGens {
            g: RISTRETTO_BASEPOINT_POINT,
            h,
        }
    }

    fn commit(&self, value: Scalar, blinding: Scalar) -> RistrettoPoint {
        self.g * value + self.h * blinding
    }

    fn verify(&self, commitment: RistrettoPoint, value: Scalar, blinding: Scalar) -> bool {
        let recomputed_commitment = self.commit(value, blinding);
        commitment == recomputed_commitment
    }
}

fn prover() -> (PedersenGens, RistrettoPoint, RistrettoPoint, Scalar) {
    // Initialize random number generator
    let mut rng = OsRng;
    // Define generator for Pedersen commitment
    let pedersen_gens = PedersenGens::new();
    // Balance
    let balance: u64 = 100;
    let balance_scalar = Scalar::from(balance);
    // Generate blinding factor for balance
    let blinding_balance: Scalar = Scalar::random(&mut rng);

    // Calculate balance commitment
    let balance_ct: RistrettoPoint = pedersen_gens.commit(balance_scalar, blinding_balance);
    // Balance after payment:
    let new_balance: u64 = 40;
    let new_balance_scalar = Scalar::from(new_balance);
    // Generate blinding factor for balance after payment
    let blinding_new_balance: Scalar = Scalar::random(&mut rng);

    // Calculate the balance commitment after payment
    let new_balance_ct: RistrettoPoint =
        pedersen_gens.commit(new_balance_scalar, blinding_new_balance);

    let blinding_payment = blinding_balance - blinding_new_balance;

    (pedersen_gens, balance_ct, new_balance_ct, blinding_payment)
}

fn verifier(
    pedersen_gens: PedersenGens,
    balance_ct: RistrettoPoint,
    new_balance_ct: RistrettoPoint,
    blinding_payment: Scalar,
) {
    // Calculate payment commitment based on the homomorphic property of commitment
    let payment_ct = balance_ct - new_balance_ct;
    // The recipient knows the payment amount
    let payment: u64 = 60;
    let payment_scalar = Scalar::from(payment);
    // Verify that the payment amount is correct without knowing the balance
    if pedersen_gens.verify(payment_ct, payment_scalar, blinding_payment) {
        println!("Commitment verified successfully!");
    } else {
        println!("Failed to verify the commitment.");
    }
}

fn main() {
    let (pedersen_gens, balance_ct, new_balance_ct, blinding_payment) = prover();
    verifier(pedersen_gens, balance_ct, new_balance_ct, blinding_payment);
}
