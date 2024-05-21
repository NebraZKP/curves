use ark_ff::fields::Fp256;
#[cfg(not(any(target_vendor = "risc0", target_vendor = "succinct")))]
use ark_ff::MontBackend;

#[cfg(not(any(target_vendor = "risc0", target_vendor = "succinct")))]
mod mont {
    use ark_ff::fields::MontConfig;

    #[derive(MontConfig)]
    #[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
    #[generator = "5"]
    pub struct FrConfig;
}

#[cfg(any(target_vendor = "risc0", target_vendor = "succinct"))]
mod plain {
    #[cfg(not(any(target_vendor = "risc0", target_vendor = "succinct")))]
    use ark_ff::MontBackend;
    use ark_ff::{
        fields::{
            plain_backend::{Fp256PlainBackend, Fp256PlainConfig},
            Fp, Fp256,
        },
        BigInt,
    };
    use core::marker::PhantomData;

    pub struct FrConfig;
    impl Fp256PlainConfig for FrConfig {
        // sage: u256_to_u64s(21888242871839275222246405745257275088548364400416034343698204186575808495617)
        const MODULUS: BigInt<4> = BigInt([
            4891460686036598785,
            2896914383306846353,
            13281191951274694749,
            3486998266802970665,
        ]);

        const GENERATOR: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([5, 0, 0, 0]), PhantomData);

        //
        const ZERO: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([0, 0, 0, 0]), PhantomData);

        //
        const ONE: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([1, 0, 0, 0]), PhantomData);

        // sage: g = Fr(5)
        // sage: x = bn_r - 1
        // sage: factor(x)
        // 2^28 * 3^2 * 13 * 29 * 983 * 11003 * 237073 * 405928799 * 1670836401704629 * 13818364434197438864469338081
        const TWO_ADICITY: u32 = 28;

        // sage: h = 3^2 * 13 * 29 * 983 * 11003 * 237073 * 405928799 * 1670836401704629 * 13818364434197438864469338081
        // sage: pow(g, h)
        // 19103219067921713944291392827692070036145651957329286315305642004821462161904
        // sage: u256_to_u64s(19103219067921713944291392827692070036145651957329286315305642004821462161904)
        const TWO_ADIC_ROOT_OF_UNITY: Fp256<Fp256PlainBackend<Self>> = Fp(
            BigInt([
                11229192882073836016,
                4624371214017703636,
                63235024940837564,
                3043318377369730693,
            ]),
            PhantomData,
        );

        #[cfg(not(any(target_vendor = "risc0", target_vendor = "succinct")))]
        type FullImplConfig = MontBackend<super::mont::FqConfig, 4>;
    }
}

#[cfg(any(target_vendor = "risc0", target_vendor = "succinct"))]
use ark_ff::fields::plain_backend;
#[cfg(any(target_vendor = "risc0", target_vendor = "succinct"))]
pub use plain::*;
#[cfg(any(target_vendor = "risc0", target_vendor = "succinct"))]
pub type Fr = Fp256<plain_backend::Fp256PlainBackend<FrConfig>>;

#[cfg(not(any(target_vendor = "risc0", target_vendor = "succinct")))]
pub use mont::*;
#[cfg(not(any(target_vendor = "risc0", target_vendor = "succinct")))]
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;
