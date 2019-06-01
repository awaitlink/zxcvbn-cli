# ❮zxcvbn-cli❯

[![version][badges/version]][crates.io/b0x]
[![downloads][badges/downloads]][crates.io/b0x]
[![license][badges/license]][license]

A simple CLI tool to check password strength using [shssoichiro/zxcvbn-rs](https://github.com/shssoichiro/zxcvbn-rs), which is a Rust port of [dropbox/zxcvbn](https://github.com/dropbox/zxcvbn).

### Installation
```console
$ cargo install zxcvbn-cli
```

### Updating
```console
$ cargo install zxcvbn-cli -f
```

### Usage
```console
$ zxcvbn [password]
```
or just
```console
$ zxcvbn
```
and enter the password via stdin.

### Examples
```console
$ zxcvbn zxcvbn
```
[![img/zxcvbn]][img/zxcvbn]

```console
$ zxcvbn "Tr0ub4dour&3"
```
[![img/Tr0ub4dour_3]][img/Tr0ub4dour_3]

```console
$ zxcvbn correcthorsebatterystaple
```
[![img/correcthorsebatterystaple]][img/correcthorsebatterystaple]

[crates.io/b0x]: https://crates.io/crates/zxcvbn-cli

[license]: https://github.com/u32i64/zxcvbn-cli/blob/master/LICENSE
[changelog]: https://github.com/u32i64/zxcvbn-cli/blob/master/CHANGELOG.md

[badges/version]: https://img.shields.io/crates/v/zxcvbn-cli.svg?style=for-the-badge
[badges/downloads]: https://img.shields.io/crates/d/zxcvbn-cli.svg?style=for-the-badge
[badges/license]: https://img.shields.io/crates/l/zxcvbn-cli.svg?style=for-the-badge

[img/zxcvbn]: https://raw.githubusercontent.com/u32i64/zxcvbn-cli/master/img/zxcvbn.png
[img/Tr0ub4dour_3]: https://raw.githubusercontent.com/u32i64/zxcvbn-cli/master/img/Tr0ub4dour_3.png
[img/correcthorsebatterystaple]: https://raw.githubusercontent.com/u32i64/zxcvbn-cli/master/img/correcthorsebatterystaple.png
