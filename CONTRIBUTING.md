# Contributing

First of all, thanks for taking the time to submit a pull request!

These are the few notes and guidelines to keep things coherent.


## Overview

1. Check you have all [requirements](#requirements) in place.
2. Create your [_feature_ branch](#feature-branch).
3. [Install](#install) the project dependencies for development.
4. [Test](#test).
5. Push your branch and submit a [Pull Request](https://github.com/ustwo/github-issues/compare/).
6. Add a description of your proposed changes and why they are needed.

We will review the changes as soon as possible.


## Requirements

* [Fork the project](http://github.com/ustwo/github-issues/fork) and clone.
* Rust +1.12


## Install

This project uses `cargo` to manage dependencies so making a build will fetch
anything required.

```sh
cargo build
```

If you are on Mac OSX you should probably use a custom `OPENSSL_INCLUDE_DIR`.
For example, mine is installed in `/usr/local/opt/openssl/include`.

```sh
OPENSSL_INCLUDE_DIR=/usr/local/openssl/include cargo build
```


## Feature Branch

```sh
git checkout -b features/feature-name
```


## Test

```sh
cargo test
```
