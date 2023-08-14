<div align="center">

# ripi
Rip an ISO from media.

<!-- <a href="https://crates.io/crates/ripi/"><img src="https://img.shields.io/crates/v/ripi?style=flat&amp;labelColor=032a1a&amp;color=065535&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a> -->
<br>
<a href="https://github.com/xTekC/ripi/actions?query=workflow%3A%22Continuous+Integration%22"><img src="https://img.shields.io/github/actions/workflow/status/xTekC/ripi/ci.yml?branch=main&amp;style=flat&amp;labelColor=032a1a&amp;color=065535&amp;logo=GitHub%20Actions&amp;logoColor=white&amp;label=Build" alt="Continuous Integration"></a>
<a href="https://github.com/xTekC/ripi/actions?query=workflow%3A%22Continuous+Deployment%22"><img src="https://img.shields.io/github/actions/workflow/status/xTekC/ripi/cd.yml?style=flat&amp;labelColor=032a1a&amp;color=065535&amp;logo=GitHub%20Actions&amp;logoColor=white&amp;label=Release" alt="Continuous Deployment"></a>
<!-- <a href="https://docs.rs/ripi/"><img src="https://img.shields.io/docsrs/ripi?style=flat&amp;labelColor=032a1a&amp;color=065535&amp;logo=Rust&amp;logoColor=white" alt="Documentation"></a> -->

[![GitHub license](https://img.shields.io/github/license/xTekC/ripi.svg?style=flat&labelColor=032a1a&color=065535&logo=GitHub&logoColor=black&label=License)](https://github.com/xTekC/ripi/blob/main/LICENSE)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A-Coffee-orange?style=flat&labelColor=FFFFFF&color=000000&logo=buy-me-a-coffee&logoColor=black)](https://www.buymeacoffee.com/xTekC)

<a href="#features">Features</a> •
<a href="#installation">Installation</a> •
<a href="#usage">Usage</a> •
<a href="#contribution">Contribution</a>
<!-- <a href="#roadmap">Roadmap</a> -->
<!-- <a href="#acknowledgements">Acknowlegements</a> • -->

</div>

## Features

**Supported systems**

_Linux_:
- riscv64gc-unknown-linux-gnu
- aarch64-unknown-linux-gnu
- aarch64-unknown-linux-musl
- x86_64-unknown-linux-gnu
- x86_64-unknown-linux-musl
<!-- - aarch64-linux-android -->

<!-- _BSD_:
- x86_64-unknown-freebsd
- x86_64-unknown-netbsd

_MacOS_:
- aarch64-apple-darwin
- x86_64-apple-darwin

_Windows_:
- aarch64-pc-windows-msvc
- x86_64-pc-windows-gnu
- x86_64-pc-windows-msvc -->

## Installation

**Cargo**

```
cargo install --git https://github.com/xTekC/ripi --branch main --locked --profile rel-opt
```

<!-- **Prebuilt Binary**

Unix-Like [Install](https://github.com/xTeKc/ripi/blob/main/scripts/install.sh)<br>

```
curl -sSL https://raw.githubusercontent.com/xTeKc/ripi/main/scripts/install.sh | sh
```

Unix-Like [Remove](https://github.com/xTeKc/ripi/blob/main/scripts/remove.sh)

```
curl -sSL https://raw.githubusercontent.com/xTeKc/ripi/main/scripts/remove.sh | sh
``` -->

<!-- (Android: Use Termux. `https://f-droid.org/repo/com.termux_118.apk`) -->

## Usage

```
ripi -h
```

## Contribution
Read the [Contributing Guide](CONTRIBUTING.md) before making a pull request.

<!-- ## Roadmap
A list of planned future developments for the project. -->

<!-- ## Acknowledgements
List of any external libraries, frameworks, or other resources used in the project. -->

<br>

Copyright (c) **xTekC** <br>
Licensed under [MPL-2.0](LICENSE)
