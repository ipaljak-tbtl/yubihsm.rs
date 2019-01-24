use yubihsm::{Capability, HmacAlg, ObjectOrigin, ObjectType};

use crate::{clear_test_key_slot, TEST_DOMAINS, TEST_KEY_ID, TEST_KEY_LABEL};

/// Generate an HMAC key
#[test]
fn hmac_key_test() {
    let mut client = crate::get_hsm_client();

    let algorithm = HmacAlg::SHA256;
    let capabilities = Capability::SIGN_HMAC | Capability::VERIFY_HMAC;

    clear_test_key_slot(&mut client, ObjectType::HmacKey);

    let key_id = client
        .generate_hmac_key(
            TEST_KEY_ID,
            TEST_KEY_LABEL.into(),
            TEST_DOMAINS,
            capabilities,
            algorithm,
        )
        .unwrap_or_else(|err| panic!("error generating wrap key: {}", err));

    assert_eq!(key_id, TEST_KEY_ID);

    let object_info = client
        .get_object_info(TEST_KEY_ID, ObjectType::HmacKey)
        .unwrap_or_else(|err| panic!("error getting object info: {}", err));

    assert_eq!(object_info.capabilities, capabilities);
    assert_eq!(object_info.object_id, TEST_KEY_ID);
    assert_eq!(object_info.domains, TEST_DOMAINS);
    assert_eq!(object_info.object_type, ObjectType::HmacKey);
    assert_eq!(object_info.algorithm, algorithm.into());
    assert_eq!(object_info.origin, ObjectOrigin::Generated);
    assert_eq!(&object_info.label.to_string().unwrap(), TEST_KEY_LABEL);
}
