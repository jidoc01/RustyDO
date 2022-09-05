RustyDO
=================================

A server emulator for Digimon Online v1.5.


## Credits

Digimon Online v1.5 was created by CCR. The rights of the game and its IP belong
to CCR and BANDAI, respectively.

This repository is to emulate the server and does not copyright the game itself
in any form or use it for profit. The user is solely responsible for the
consequences of using this.

This repository is maintained by JungHyun Kim (@jidoc01).


## Announcements

### RC4 Algorithm

Now, RustyDO uses its self-contained RC4 algorithm, which does not require
Windows APIs. At the same time, it won't need Windows OS for its environment.


## Objective

This project is an open-source version of a toy server that I used to make in my
free time. The goal is to recreate the game server of the time. Due to the lack
of information, difficulties are expected, but we can try to imitate it as much
as we can.


## Videos

[![Video Label](http://img.youtube.com/vi/qFHj128fxyM/0.jpg)](https://youtu.be/qFHj128fxyMI)


## Screenshots

| ![lobby](https://user-images.githubusercontent.com/12146267/183245660-494904c7-a072-4f31-839e-db5fffd7d04d.png) |
|:--:|
| *In a lobby* |

| ![room](https://user-images.githubusercontent.com/12146267/183245665-af69dc61-a110-4577-a1e3-a4baa2fc7247.png) |
|:--:|
| *In a room* |

| ![game](https://user-images.githubusercontent.com/12146267/183245667-5524b15f-648a-4d35-aaf6-b6dbb95eb018.png) | 
|:--:| 
| *In a game* |


## References

+ Wiki:
  + https://namu.wiki/w/%EB%94%94%EC%A7%80%EB%AA%AC%20%EC%98%A8%EB%9D%BC%EC%9D%B8 (Korean)
  + https://digimon.fandom.com/wiki/Digimon_Battle_Server

+ Client: https://archive.org/details/digimonbattleserver


## How to run

### 0. Pre-requisite

#### For Server

+ Rust: [Cargo 1.62.0+](https://www.rust-lang.org/tools/install)
+ Git: [Git](https://git-scm.com/downloads)

#### For Client

+ Windows OS 

### [Server] 1. Build the source code

First, you need to clone this respository: 
`git clone https://github.com/jidoc01/RustyDO`

After cloning it, compile the source code:
`cargo build --release`

When it succeeds, it will generate an executable on `{root_directory}/target/release/server.*`. Copy the executable to some place and put `config.toml` to the same directory.

### [Server] 2. Run the server

To run the server, launch the `server.*`. It will open network port `9874` for both TCP and UDP. So, you should not use any of the same ports to prevent port collision. It is not modifiable unless you directly hack the client, and it is easily achievable but  out-of-scope here.

### [Client] 1. Prepare the client program

First, download the client file `digimonbattleserver.rar` and uncompress it to some place. In its root directory, there are three sub-directories. You will use `Digimon Online` directory only.

And, you should edit configurations to launch the client program.

*NOTE*: You should not share the client with any modification. It is strictly forbidden in Korean copyright law. The archived client is as it was 2002 without modification.

#### [Client] 2-1. Edit the configuration

In the `Digimon Online` directory, there is `svr.info`, and open it with a text editor. Then, you can see ip addresses there. Replace them with your ip address. Let `SERVER_IP` be your ip address, then it'd look like:

```
101	0	100	SERVER_IP		Status1
...
401	3	5000	SERVER_IP		폴더대륙
```

Note that the address should be a ***public*** ip address if it needs to be accessed via public network. And it should be a ***dot-separated*** address, not a domain name.

#### [Client] 2-2. Edit the registry

You need to write the absolute path of `Digimon Online` directory into the registry item  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\X2Online\Digimon Online V1.5` with a key `PATH`.

For example, if your `Digimon Online` directory is `C:/X2Online/Digimon Online`, then you can use the following registry script:

```
Windows Registry Editor Version 5.00

[HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\X2Online\Digimon Online V1.5]
"PATH"="C:\\X2Online\\Digimon Online"
```

### [Client] 3. Launch the client executable

Now you can launch the client executable in `Digimon Online` directory. In the directory, the following instruction will launch the game: `digimon.dll "1 1"`.

FYI, you can write a batch script (*.bat) and launch it instead of typing the instruction every time you launch the game.


## To those who want to contribute

This project only implements basic functions. The parts that need to be
implemented are marked as TODO on the source code.

If you want to implement TODO, introduce new features (ex. processing a new
packet, introducing new implementation, etc.), or improve inefficient parts of
its existing implementation, feel free to issue. Especially, it is written in
Rust language, but I am not very fluent in the language. And I am not aware of
server architectures. Feel free to suggest any form of improvement in its design.


## TODOs

There are features not implemented (or finished) yet. You can search `TODO` in the source code to check what features are not implemented yet.

### Board
- [x] Writing a new post.
- [x] Reading a post.
- [ ] Modification/deletion/reply of posts.

### Shop
- [x] Entering/Leaving a shop.
- [ ] Checking validity of purchasing.

### Room
- [x] Changing team/character/map.
- [x] Kicking a player.

### In-game
- [x] Basic movements.
- [x] Using items.
- [ ] Expressing emotions.
- [ ] Kurumon generation algorithm.
  + It could be impossible to recover the exact algorithms which its original server used.
  + But I hope we imitate it as much as we can.
- [ ] Item generation algorithm.
- [ ] Red crack generation algorithm.
- [ ] Improving priority calculation algorithm.
- [ ] Leaving a game while playing it.
- [ ] Gaining experiences after game.

### Messenger
- [ ] Packet analysis (both in TCP/UDP).

### Ranking
- [ ] Packet analysis.

### Etc.
- [x] Changing nickname.
- [x] Changing game settings.
- [x] Chatting.
- [x] Whispering.
- [ ] Server migration.
- [ ] Scheduling priority in events.

## License

[GNU AGPL v3+](https://www.gnu.org/licenses/agpl-3.0.en.html)

Copyright 2022. JungHyun Kim (jidoc01).