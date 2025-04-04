use super::*;

#[test]
fn correct_token_verifies() {
    let timestamp = 123456789;

    let token = sign(timestamp);

    assert!(verify(timestamp, &token).is_ok_and(|v| v == 123456789))
}

#[test]
fn valid_base64_but_incorrect_token_does_not_verify() {
    let timestamp = 123456789;
    let bad_timestamp = 987654321;
    let bad_token = sign(bad_timestamp);

    assert!(verify(timestamp, &bad_token).is_err())
}

#[test]
fn too_short_token_does_not_verify() {
    let timestamp = 123456789;

    let token = sign(timestamp);

    assert!(verify(123456789, &token[0..12]).is_err())
}

#[test]
fn too_long_token_does_not_verify() {
    let timestamp = 123456789i64;

    // manually sign so we can get extra bytes
    let token = {
        let mut mac = make_mac();

        mac.update(&timestamp.to_ne_bytes());

        let result = mac.finalize();

        let bytes = result.into_bytes();

        // 12 bytes for 16 base64 characters
        BASE64_URL_SAFE.encode(&bytes[0..24])
    };

    assert!(verify(123456789, &token).is_err())
}

#[test]
fn invalid_base64_does_not_verify() {
    assert!(verify(123456789, &"$$$$$$$$$$$$$$$$").is_err())
}
