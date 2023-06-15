#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]


pub mod utils;
pub mod keys;
pub mod signatures;
pub mod schemes;
pub mod bbsplus;
pub mod cl03;
pub mod tests;


#[cfg(test)]
mod bbsplus_tests {
    use crate::{schemes::algorithms::{BBSplusSha256, BBSplusShake256}, tests::{map_message_to_scalar_as_hash, message_generators, msg_signature, h2s}};

    #[test]
    fn map_message_to_scalar_as_hash_sha256() {
        map_message_to_scalar_as_hash::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/MapMessageToScalarAsHash.json");
    }

    #[test]
    fn map_message_to_scalar_as_hash_shake256() {
        map_message_to_scalar_as_hash::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/MapMessageToScalarAsHash.json");
    }

    #[test]
    fn message_generators_sha256() {
        message_generators::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/generators.json");
    }

    #[test]
    fn message_generators_shake256() {
        message_generators::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/generators.json");
    }


    //MSG SIGNATURE
    #[test]
    fn msg_signature_sha256_1() {
        msg_signature::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "signature/signature001.json");
    }
    #[test]
    fn msg_signature_sha256_2() {
        msg_signature::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "signature/signature002.json");
    }
    #[test]
    fn msg_signature_sha256_3() {
        msg_signature::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "signature/signature003.json");
    }
    #[test]
    fn msg_signature_sha256_4() {
        msg_signature::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json");
    }
    #[test]
    fn msg_signature_sha256_5() {
        msg_signature::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "signature/signature005.json");
    }
    #[test]
    fn msg_signature_sha256_6() {
        msg_signature::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "signature/signature006.json");
    }
    #[test]
    fn msg_signature_sha256_7() {
        msg_signature::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "signature/signature007.json");
    }
    #[test]
    fn msg_signature_sha256_8() {
        msg_signature::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "signature/signature008.json");
    }
    #[test]
    fn msg_signature_sha256_9() {
        msg_signature::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "signature/signature009.json");
    }


    //MSG SIGNATURE - SHAKE256
    #[test]
    fn msg_signature_shake256_1() {
        msg_signature::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "signature/signature001.json");
    }
    #[test]
    fn msg_signature_shake256_2() {
        msg_signature::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "signature/signature002.json");
    }
    #[test]
    fn msg_signature_shake256_3() {
        msg_signature::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "signature/signature003.json");
    }
    #[test]
    fn msg_signature_shake256_4() {
        msg_signature::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json");
    }
    #[test]
    fn msg_signature_shake256_5() {
        msg_signature::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "signature/signature005.json");
    }
    #[test]
    fn msg_signature_shake256_6() {
        msg_signature::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "signature/signature006.json");
    }
    #[test]
    fn msg_signature_shake256_7() {
        msg_signature::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "signature/signature007.json");
    }
    #[test]
    fn msg_signature_shake256_8() {
        msg_signature::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "signature/signature008.json");
    }
    #[test]
    fn msg_signature_shake256_9() {
        msg_signature::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "signature/signature009.json");
    }

    //h2s - SHA256
    #[test]
    fn h2s_sha256_1() {
        h2s::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "h2s/h2s001.json")
    }
    #[test]
    fn h2s_sha256_2() {
        h2s::<BBSplusSha256>("./fixture_data/bls12-381-sha-256/", "h2s/h2s002.json")
    }

    //h2s - SHAKE256
    #[test]
    fn h2s_shake256_1() {
        h2s::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "h2s/h2s001.json")
    }
    #[test]
    fn h2s_shake256_2() {
        h2s::<BBSplusShake256>("./fixture_data/bls12-381-shake-256/", "h2s/h2s002.json")
    }


    //mocked_rng - SHA256
    #[test]
    fn mocked_rng_sha256() {

    }

    //mocked_rng - SHAKE256
    #[test]
    fn mocked_rng_shake256() {

    }

}


#[cfg(test)]
mod cl03_tests {
    #[test]
    fn prova_cl() {
        println!("CL03");
    }
}