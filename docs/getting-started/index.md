---
layout: default
title: Getting Started
nav_order: 2
has_children: true
permalink: /docs/getting-started
---

{: .note }
한글 문서는 [이곳]({{'/RustyDO/docs/getting-started/ko' }})을 참조하세요.


# Getting Started
{: .no_toc }

This page explains how to emulate Digimon Online v1.5 with RustyDO.
{: .fs-6 .fw-300 }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---



## Installation

This section explains how to set up and run RustyDO.

### Compile RustyDO

First, download the source of the project. You can download it from the website [(link)](https://github.com/jidoc01/RustyDO/archive/refs/heads/main.zip).

Or, if you've installed Git [(link)](https://git-scm.com/), you can also use it:

```bash
git clone https://github.com/jidoc01/RustyDO
```

Go to the root directory of the source code (i.e. `RustyDO`). Compile the source code with Rust compiler Cargo:

```bash
cargo build --release
```

{: .note }
You can download and install Cargo from the official Rust website [(link)](https://www.rust-lang.org/tools/install).

Done! It will generate the server executable on `RustyDO/target/release/server.*`.

### Move Configuration

Find `config.toml` in the root directory of the project. Copy it to the same folder as the server executable.

### Run RustyDO

Execute the server executable. It will start the server emulation for Digimon Online v1.5.

## How to Play

We do not offer any guidance for operating servers.
We focus on how to develop our server code. We do not share how to operate his/her own server.
Please understand that it is strictly forbidden to operate private servers in Korea:
see [게임산업진흥에 관한 법률 제32조 (게임산업법)](https://www.law.go.kr/%EB%B2%95%EB%A0%B9/%EA%B2%8C%EC%9E%84%EC%82%B0%EC%97%85%EC%A7%84%ED%9D%A5%EC%97%90%EA%B4%80%ED%95%9C%EB%B2%95%EB%A5%A0)
