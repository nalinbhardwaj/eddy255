# Eddy255

Eddy255 is a prime-order elliptic curve that embeds the Edwards curve `ed25519`, i.e. the order of Eddy255 is equal to the base field size of `ed25519`(2^255 - 19).

This repository contains the parameters for the curve, as well as a reference implementation of the curve using `arkworks`. Additionally, it includes a script to check the `SafeCurves` criteria for the curve (and some other generation utilities).

The primary use case for Eddy255 is to provide a curve where we can perform native-computation for `ed25519` EC operations. This right-field arithmetic is particularly useful for constructing zero-knowledge proofs verifying knowledge of [EDDSA](https://cryptobook.nakov.com/digital-signatures/eddsa-and-ed25519) signatures, for instance with the [Spartan](https://github.com/microsoft/Spartan) proving system, which uses the IPA commitment scheme.

This was found using [eccons](https://github.com/kwantam/eccons) thanks to [kwantam](https://github.com/kwantam) and [Srinath Setty](https://twitter.com/srinathtv).

A related curve is [Yafa-108/146](https://eprint.iacr.org/2022/1145), which also embeds `ed25519`, but with a pairing-friendly curve. Eddy255 is much smaller (~2x) but it is not pairing-friendly.
