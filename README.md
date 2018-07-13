# yubihsm.rs

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![MIT/Apache2 licensed][license-image]

[crate-image]: https://img.shields.io/crates/v/yubihsm.svg
[crate-link]: https://crates.io/crates/yubihsm
[docs-image]: https://docs.rs/yubihsm/badge.svg
[docs-link]: https://docs.rs/yubihsm/
[build-image]: https://circleci.com/gh/tendermint/yubihsm-rs.svg?style=shield
[build-link]: https://circleci.com/gh/tendermint/yubihsm-rs
[license-image]: https://img.shields.io/badge/license-MIT/Apache2.0-blue.svg

A pure-Rust client for [YubiHSM2] devices from [Yubico].

[Documentation][docs-link]

[libyubihsm]: https://developers.yubico.com/YubiHSM2/Component_Reference/libyubihsm/
[YubiHSM2]: https://www.yubico.com/products/yubihsm/
[Yubico]: https://www.yubico.com/

## About

This is a pure-Rust client library for [YubiHSM2] devices which implements
most the functionality of the closed-source [libyubihsm] library from the
Yubico SDK. It communicates with the [yubihsm-connector] service: an HTTP(S)
server which sends the commands to the YubiHSM2 hardware device over USB.

Note that this is **NOT** an official Yubico project and is in no way supported
or endorsed by Yubico (although whoever runs their Twitter account
[thinks it's awesome]).

[yubihsm-connector]: https://developers.yubico.com/YubiHSM2/Component_Reference/yubihsm-connector/
[thinks it's awesome]: https://twitter.com/Yubico/status/971186516796915712

## Prerequisites

This crate builds on Rust 1.27+ and by default uses SIMD features
which require the following RUSTFLAGS:

```
RUSTFLAGS=-Ctarget-feature=+aes`
```

You can configure your `~/.cargo/config` to always pass these flags:

```toml
[build]
rustflags = ["-Ctarget-feature=+aes"]
```

## Supported Commands

NOTE: If there's a command on this list which isn't presently supported,
[contributing is easy! (See below)](https://github.com/tendermint/yubihsm-rs/blob/master/README.md#contributing)

| Command                | Impl'd | [MockHSM] | Description |
|------------------------|--------|-----------|-------------|
| [Attest Asymmetric]    | ✅     | ⛔        | Create X.509 certificate for asymmetric key |
| [Authenticate Session] | ✅     | ✅        | Authenticate to HSM with password or encryption key|
| [Blink]                | ✅     | ✅        | Blink the HSM's LEDs (to identify it) |
| [Close Session]        | ✅     | ✅        | Terminate an encrypted session with the HSM |
| [Create Session]       | ✅     | ✅        | Initiate a new encrypted session with the HSM |
| [Decrypt ECDH]         | ⛔     | ⛔        | Compute Elliptic Curve Diffie-Hellman using HSM-backed key |
| [Decrypt OAEP]         | ⛔     | ⛔        | Decrypt data encrypted with RSA-OAEP |
| [Decrypt PKCS1]        | ⛔     | ⛔        | Decrypt data encrypted with RSA-PKCS#1v1.5 |
| [Device Info]          | ✅     | ✅        | Get information about the HSM |
| [Delete Object]        | ✅     | ✅        | Delete an object of the given ID and type |
| [Echo]                 | ✅     | ✅        | Echo a message sent to the HSM |
| [Export Wrapped]       | ✅     | ✅        | Export an object from the HSM in encrypted form|
| [Generate Asymmetric]  | ✅     | ✅        | Randomly generate new asymmetric key in the HSM |
| [Generate HMAC Key]    | ⛔     | ⛔        | Randomly generate HMAC key in the HSM |
| [Generate OTP Key]     | ⛔     | ⛔        | Randomly generate AES key for Yubico OTP authentication |
| [Generate Wrap Key]    | ✅     | ✅        | Randomly generate AES key for exporting/importing objects |
| [Get Logs]             | ✅     | ✅        | Obtain the audit log for the HSM |
| [Get Object Info]      | ✅     | ✅        | Get information about an object |
| [Get Opaque]           | ✅     | ✅        | Get an opaque bytestring from the HSM |
| [Get Option]           | ⛔     | ⛔        | Get information about HSM settings |
| [Get Pseudo Random]    | ✅     | ✅        | Get random data generated by the HSM's internal PRNG |
| [Get Pubkey]           | ✅     | ✅        | Get public key for an HSM-backed asymmetric private key |
| [HMAC Data]            | ⛔     | ⛔        | Perform an HMAC operation using an HSM-backed key |
| [Import Wrapped]       | ✅     | ✅        | Import an encrypted key into the HSM |
| [List Objects]         | ✅     | ✅        | List objects visible from the current session |
| [OTP AEAD Create]      | ⛔     | ⛔        | Create a Yubico OTP AEAD |
| [OTP AEAD Random]      | ⛔     | ⛔        | Randomly generate a Yubico OTP AEAD |
| [OTP AEAD Rewrap]      | ⛔     | ⛔        | Re-wrap a Yubico OTP AEAD from one key to another |
| [OTP Decrypt]          | ⛔     | ⛔        | Decrypt a Yubico OTP, obtaining counters and timer info |
| [Put Asymmetric]       | ✅     | ✅        | Put an existing asymmetric key into the HSM |
| [Put Auth Key]         | ✅     | ✅        | Put AES-128x2 preshared authentication key into HSM |
| [Put HMAC Key]         | ✅     | ⛔        | Put an HMAC key into the HSM |
| [Put Opaque]           | ✅     | ✅        | Put an opaque bytestring into the HSM |
| [Put Option]           | ⛔     | ⛔        | Change HSM settings |
| [Put OTP AEAD Key]     | ✅     | ⛔        | Put a Yubico OTP key into the HSM |
| [Put Wrap Key]         | ✅     | ✅        | Put an AES keywrapping key into the HSM |
| [Reset]                | ✅     | ✅        | Reset the HSM back to factory default settings |
| [Session Message]      | ✅     | ✅        | Send an encrypted message to the HSM |
| [Set Log Index]        | ⛔     | ⛔        | Mark log messages in the HSM as consumed |
| [Sign Data ECDSA]      | ✅     | ✅        | Compute an ECDSA signature using HSM-backed key |
| [Sign Data EdDSA]      | ✅     | ✅        | Compute an Ed25519 signature using HSM-backed key |
| [Sign Data PKCS1]      | ⛔     | ⛔        | Compute an RSASSA-PKCS#1v1.5 signature using HSM-backed key |
| [Sign Data PSS]        | ⛔     | ⛔        | Compute an RSASSA-PSS signature using HSM-backed key |
| [Storage Status]       | ✅     | ✅        | Fetch information about currently free storage |
| [Unwrap Data]          | ✅     | ⛔        | Decrypt data encrypted using a wrap key |
| [Verify HMAC]          | ⛔     | ⛔        | Verify that an HMAC tag for given data is valid |
| [Wrap Data]            | ✅     | ⛔        | Encrypt data using a wrap key |

[Attest Asymmetric]: https://docs.rs/yubihsm/latest/yubihsm/commands/attest_asymmetric/fn.attest_asymmetric.html
[Authenticate Session]: https://developers.yubico.com/YubiHSM2/Commands/Authenticate_Session.html
[Blink]: https://docs.rs/yubihsm/latest/yubihsm/commands/blink/fn.blink.html
[Close Session]: https://developers.yubico.com/YubiHSM2/Commands/Close_Session.html
[Create Session]: https://developers.yubico.com/YubiHSM2/Commands/Create_Session.html
[Decrypt ECDH]: https://developers.yubico.com/YubiHSM2/Commands/Decrypt_Ecdh.html
[Decrypt OAEP]: https://developers.yubico.com/YubiHSM2/Commands/Decrypt_Oaep.html
[Decrypt PKCS1]: https://developers.yubico.com/YubiHSM2/Commands/Decrypt_Pkcs1.html
[Delete Object]: https://docs.rs/yubihsm/latest/yubihsm/commands/delete_object/fn.delete_object.html
[Device Info]: https://docs.rs/yubihsm/latest/yubihsm/commands/device_info/fn.device_info.html
[Echo]: https://docs.rs/yubihsm/latest/yubihsm/commands/echo/fn.echo.html
[Export Wrapped]: https://docs.rs/yubihsm/latest/yubihsm/commands/export_wrapped/fn.export_wrapped.html
[Generate Asymmetric]: https://docs.rs/yubihsm/latest/yubihsm/commands/generate_asymmetric_key/fn.generate_asymmetric_key.html
[Generate HMAC Key]: https://developers.yubico.com/YubiHSM2/Commands/Generate_Hmac_Key.html
[Generate OTP Key]: https://developers.yubico.com/YubiHSM2/Commands/Generate_Otp_Aead_Key.html
[Generate Wrap Key]: https://docs.rs/yubihsm/latest/yubihsm/commands/generate_wrap_key/fn.generate_wrap_key.html
[Get Logs]: https://docs.rs/yubihsm/latest/yubihsm/commands/get_logs/fn.get_logs.html
[Get Object Info]: https://docs.rs/yubihsm/latest/yubihsm/commands/get_object_info/fn.get_object_info.html
[Get Opaque]: https://developers.yubico.com/YubiHSM2/Commands/Get_Opaque.html
[Get Option]: https://developers.yubico.com/YubiHSM2/Commands/Get_Option.html
[Get Pseudo Random]: https://developers.yubico.com/YubiHSM2/Commands/Get_Pseudo_Random.html
[Get Pubkey]: https://docs.rs/yubihsm/latest/yubihsm/commands/get_pubkey/fn.get_pubkey.html
[HMAC Data]: https://developers.yubico.com/YubiHSM2/Commands/Hmac_Data.html
[Import Wrapped]: https://docs.rs/yubihsm/latest/yubihsm/commands/import_wrapped/fn.import_wrapped.html
[List Objects]: https://docs.rs/yubihsm/latest/yubihsm/commands/list_objects/fn.list_objects.html
[OTP AEAD Create]: https://developers.yubico.com/YubiHSM2/Commands/Otp_Aead_Create.html
[OTP AEAD Random]: https://developers.yubico.com/YubiHSM2/Commands/Otp_Aead_Random.html
[OTP AEAD Rewrap]: https://developers.yubico.com/YubiHSM2/Commands/Otp_Aead_Rewrap.html
[OTP Decrypt]: https://developers.yubico.com/YubiHSM2/Commands/Otp_Decrypt.html
[Put Asymmetric]: https://docs.rs/yubihsm/latest/yubihsm/commands/put_asymmetric_key/fn.put_asymmetric_key.html
[Put Auth Key]: https://docs.rs/yubihsm/latest/yubihsm/commands/put_auth_key/fn.put_auth_key.html
[Put HMAC Key]: https://docs.rs/yubihsm/latest/yubihsm/commands/put_hmac_key/fn.put_hmac_key.html
[Put Opaque]: https://docs.rs/yubihsm/latest/yubihsm/commands/put_opaque/fn.put_opaque.html
[Put Option]: https://developers.yubico.com/YubiHSM2/Commands/Put_Option.html
[Put OTP AEAD Key]: https://docs.rs/yubihsm/latest/yubihsm/commands/put_otp_aead_key/fn.put_otp_aead_key.html
[Put Wrap Key]: https://docs.rs/yubihsm/latest/yubihsm/commands/put_wrap_key/fn.put_wrap_key.html
[Reset]: https://docs.rs/yubihsm/latest/yubihsm/commands/reset/fn.reset.html
[Session Message]: https://developers.yubico.com/YubiHSM2/Commands/Session_Message.html
[Set Log Index]: https://developers.yubico.com/YubiHSM2/Commands/Set_Log_Index.html
[Sign Data ECDSA]: https://docs.rs/yubihsm/latest/yubihsm/commands/sign_ecdsa/fn.sign_ecdsa_sha2.html
[Sign Data EdDSA]: https://docs.rs/yubihsm/latest/yubihsm/commands/sign_eddsa/fn.sign_ed25519.html
[Sign Data PKCS1]: https://developers.yubico.com/YubiHSM2/Commands/Sign_Data_Pkcs1.html
[Sign Data PSS]: https://developers.yubico.com/YubiHSM2/Commands/Sign_Data_Pss.html
[Storage Status]: https://docs.rs/yubihsm/latest/yubihsm/commands/storage_status/fn.storage_status.html
[Unwrap Data]: https://docs.rs/yubihsm/latest/yubihsm/commands/unwrap_data/fn.unwrap_data.html
[Verify HMAC]: https://developers.yubico.com/YubiHSM2/Commands/Verify_Hmac.html
[Wrap Data]: https://docs.rs/yubihsm/latest/yubihsm/commands/wrap_data/fn.wrap_data.html

## Getting Started

The following documentation describes the most important parts of this crate's API:

* [Session]: end-to-end encrypted connection with the YubiHSM. You'll need an active one to do anything.
* [commands]: commands supported by the YubiHSM2 (i.e. main functionality)

[Session]: https://docs.rs/yubihsm/latest/yubihsm/session/struct.Session.html
[commands]: https://docs.rs/yubihsm/latest/yubihsm/commands/index.html

Here is an example of how to create a `Session` by connecting to a [yubihsm-connector]
process, and then performing an Ed25519 signature:

```rust
extern crate yubihsm;
use yubihsm::Session;

// Default host, port, auth key ID, and password for yubihsm-connector
let mut session = Session::create_from_password(
     "http://127.0.0.1:12345",
     1,
     b"password",
     true
).unwrap();

// Note: You'll need to create this key first. Run the following from yubihsm-shell:
// `generate asymmetric 0 100 ed25519_test_key 1 asymmetric_sign_eddsa ed25519`
let signature = yubihsm::sign_ed25519(&session, 100, "Hello, world!").unwrap();
println!("Ed25519 signature: {:?}", signature);
```

## Contributing

If there are additional [YubiHSM2 commands] you would like to use but aren't
presently supported, adding them is very easy, and PRs are welcome!

The YubiHSM2 uses a simple, bincode-like message format, which largely consists
of fixed-width integers, bytestrings, and bitfields. This crate implements a
[Serde-based message parser] which can automatically parse command/response
messages used by the HSM derived from a corresponding Rust struct describing
their structure.

Here's a list of steps necessary to implement a new command type:

1. Find the command you wish to implement on the [YubiHSM2 commands] page, and
   study the structure of the command (i.e. request) and response.
2. Create a new module under the [commands] module which matches the name
   of the command and implements the `Command` and `Response` traits.
3. (Optional) Implement the command in [mockhsm/commands.rs] and write an
   [integration test].

[YubiHSM2 commands]: https://developers.yubico.com/YubiHSM2/Commands/
[Serde-based message parser]: https://github.com/tendermint/yubihsm-rs/tree/master/src/serializers
[commands]: https://github.com/tendermint/yubihsm-rs/tree/master/src/commands
[mockhsm/commands.rs]: https://github.com/tendermint/yubihsm-rs/blob/master/src/mockhsm/commands.rs
[integration test]:  https://github.com/tendermint/yubihsm-rs/blob/master/tests/integration.rs

## Testing

This crate allows you to run the integration test suite in two different ways:
live testing against a real YubiHSM2 device, and simulated testing using
a [MockHSM] service which reimplements some YubiHSM2 functionality in software.

[MockHSM]: https://docs.rs/yubihsm/latest/yubihsm/mockhsm/struct.MockHSM.html

### `cargo test --features=integration`: test live against a YubiHSM2 device

This mode assumes you have a YubiHSM2 hardware device, have downloaded the
[YubiHSM2 SDK] for your platform, and are running a **yubihsm-connector**
process listening on localhost on the default port of 12345.

The YubiHSM2 device should be in the default factory state. To reset it to this
state, either use the [yubihsm-shell reset] command or press on the YubiHSM2 for
10 seconds immediately after inserting it.

You can confirm the tests are running live against the YubiHSM2 by the LED
blinking rapidly for 1 second.

**NOTE THAT THESE TESTS ARE DESTRUCTIVE: DO NOT RUN THEM AGAINST A YUBIHSM2
WHICH CONTAINS KEYS YOU CARE ABOUT**

[YubiHSM2 SDK]: https://developers.yubico.com/YubiHSM2/Releases/
[yubihsm-shell reset]: https://developers.yubico.com/YubiHSM2/Commands/Reset.html

### `cargo test --features=mockhsm`: simulated tests against a mock HSM

This mode is useful for when you don't have access to physical YubiHSM2
hardware, such as CI environments.

## License

**yubihsm.rs** is distributed under the terms of both the MIT license and
the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
