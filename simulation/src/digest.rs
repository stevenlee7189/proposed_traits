use proposed_traits::digest::DigestOp;

struct DigestEngine;


struct Sha256Context<'a> {
    engine: &'a mut  DigestEngine,
}

struct Sha512Context<'a> {
    engine: &'a mut  DigestEngine,
}


enum DigestContext<'a> {
    Sha256(Sha256Context<'a>),
    Sha512(Sha512Context<'a>),
}


enum DigestOutput {
    Sha256([u32; 8]),   // Output for SHA-256 algorithm
    Sha512([u32; 16]),  // Output for SHA-512 algorithm
}

enum Algorithm {
    Sha256,
    Sha512,
}


impl DigestOp for Sha256Context {
    type Output = DigestOutput;

    fn update(&mut self, data: &[u8]) {
        match self {
            DigestContext::Sha256(ctx) => ctx.update(data),
            DigestContext::Sha512(ctx) => ctx.update(data),
        }
    }

    fn finalize(self) -> Self::Output {
        match self {
            DigestContext::Sha256(ctx) => DigestOutput::Sha256(ctx.finalize()),
            DigestContext::Sha512(ctx) => DigestOutput::Sha512(ctx.finalize()),
        }
    }
}
