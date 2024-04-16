use ark_ff::Fp256;
#[cfg(not(target_vendor = "risc0"))]
use ark_ff::MontBackend;

#[cfg(not(target_vendor = "risc0"))]
mod mont {
    use ark_ff::fields::MontConfig;

    #[derive(MontConfig)]
    #[modulus = "21888242871839275222246405745257275088696311157297823662689037894645226208583"]
    #[generator = "3"]
    pub struct FqConfig;
}

#[cfg(target_vendor = "risc0")]
mod plain {
    #[cfg(not(target_vendor = "risc0"))]
    use ark_ff::MontBackend;
    use ark_ff::{
        fields::{
            plain_backend::{Fp256PlainBackend, Fp256PlainConfig},
            Fp, Fp256,
        },
        BigInt,
    };
    use core::marker::PhantomData;

    pub struct FqConfig;
    impl Fp256PlainConfig for FqConfig {
        const MODULUS: BigInt<4> = BigInt([
            4332616871279656263,
            10917124144477883021,
            13281191951274694749,
            3486998266802970665,
        ]);

        //
        const GENERATOR: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([3, 0, 0, 0]), PhantomData);

        //
        const ZERO: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([0, 0, 0, 0]), PhantomData);

        //
        const ONE: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([1, 0, 0, 0]), PhantomData);

        const TWO_ADICITY: u32 = 2;

        // u256_to_u64s(21888242871839275222246405745257275088696311157297823662689037894645226208582)
        // = Fq(-1)
        const TWO_ADIC_ROOT_OF_UNITY: Fp256<Fp256PlainBackend<Self>> = Fp(
            BigInt([
                4332616871279656262,
                10917124144477883021,
                13281191951274694749,
                3486998266802970665,
            ]),
            PhantomData,
        );

        #[cfg(not(target_vendor = "risc0"))]
        type FullImplConfig = MontBackend<super::mont::FqConfig, 4>;

        // const TWO_ADICITY: u32 = 42;
        // const TWO_ADIC_ROOT_OF_UNITY: Fp<Self, N> = value;
        // const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp256<Fp256PlainBackend<Self>>>> = None;
    }

    // pub type Fq = Fp256<Fp256PlainBackend<FqConfig>>;
}

#[cfg(target_vendor = "risc0")]
use ark_ff::fields::plain_backend;
#[cfg(target_vendor = "risc0")]
pub use plain::*;
#[cfg(target_vendor = "risc0")]
pub type Fq = Fp256<plain_backend::Fp256PlainBackend<FqConfig>>;

#[cfg(not(target_vendor = "risc0"))]
pub use mont::*;
#[cfg(not(target_vendor = "risc0"))]
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;
