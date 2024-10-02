# Opay

- [Description](#description)
- [Getting Started](#getting-started)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)
- [Help](#help)
- [Authors](#authors)
- [Version History](#version-history)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Description

Opay SDK in the Rust Programming language

## Getting Started

To get started create a account from https://merchant.opaycheckout.com/signup or
login to existing account

### Installing

```sh
cargo add opay_rs tokio -F full 
```

### Executing program

```rust
use opay_rs::{
    opay_client::OpayClient,
    error::OpayClientError,
    opay::{Environment, MerchantId, PublicKey},
};

#[tokio::main]
async fn main() -> Result<(), OpayClientError> {
    let env = Environment::Development; // Environment::Default
    let public_key = PublicKey::from(&std::env::var("OPAY_PUBLIC_KEY").ok().unwrap());
    let merchant_id = MerchantId::from(&std::env::var("OPAY_MERCHANT_ID").ok().unwrap());
    let opay_client = OpayClient::new(env, public_key, merchant_id);

    let bank_list = opay_client.get_bank_list().await;

    Ok(())
}

```

## Documentation

Describe any special instructions that are necessary to install a software
package on your computer (if applicable).

## Help

Any advise for common problems or issues.

```
command to run if program contains helper info
```

## Authors

Contributors names and contact info

ex. Dominique Pizzie\
ex. [@DomPizzie](https://twitter.com/dompizzie)

## Version History

- 0.2
  - Various bug fixes and optimizations
  - See [commit change]() or See [release history]()
- 0.1
  - Initial Release

## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file
for details

## Acknowledgments

Inspiration, code snippets, etc.
