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

The previous section explained how to run RustyDO. This section explains how to play Digimon Online v1.5 with RustyDO.

{: .warning }
The client of Digimon Online v1.5 only supports Windows OS. Check your OS first.

### Download Client

Download the client of Digimon Online v1.5. We've archieved the old client obtained in 2002 [(link)](https://archive.org/details/digimonbattleserver).

Extract the file and find `Digimon Online` directory; we will use contents in the directory only.

### Edit Registry

Copy the absolute path of `Digimon Online` directory. And, with Windows Registry Editor, paste it to the registry item with the key `PATH` of `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\X2Online\Digimon Online V1.5`.

Or, you can use a registry script. For example, let's say the path of your `Digimon Online` is `C:/X2Online/Digimon Online`. You can use the following registry script:

```
Windows Registry Editor Version 5.00

[HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\X2Online\Digimon Online V1.5]
"PATH"="C:\\X2Online\\Digimon Online"
```

You can copy the registry script above, and save it to a file with the extension `reg` (i.e. `digimon.reg`). Execute the file, and it will register the `PATH` automatically.

### Edit Server Configuration

In the `Digimon Online` directory, find `svr.info`. Open it with any text editor.

Search ip addresses in the file. Replace them by your ip address on which RustyDO is running. Use a dot-separated address (not a domain name).

For example, let's say `192.168.0.1` is your server address. The result should be as follows:

```
101	0	100	192.168.0.1		Status1
...
401	3	5000	192.168.0.1		폴더대륙
```

{: .note }
IP addresses should be public ip addresses if you'd like to accept players from the outside of your local network.

{: .warning }
Do not distribute the client with any modification. In Korean copyright law, it is strictly forbidden.

### Run Game

In the `Digimon Online` directory, type the instruction to execute the client:

```bash
./digimon.dll "1 1"
```

Enjoy!