RustyDO
=================================

A server emulator for Digimon Online v1.5.

Homepage: https://jidoc01.github.io/RustyDO

## Credits

Digimon Online v1.5 was created by CCR. The rights of the game and its IP belong
to Move Interactive and BANDAI, respectively.

This repository is to emulate the server and does not copyright the game itself
in any form or use it for profit. The user is solely responsible for the
consequences of using this.

This repository is maintained by Jung Hyun Kim (@jidoc01).

디지몬 온라인 v1.5는 CCR이 제작한 온라인 게임입니다.
게임의 저작권, 그리고 게임이 사용하는 캐릭터 IP는 각각 Move Interactive, 그리고 BANDAI에게 있습니다.

이 프로젝트는 서비스 종료된 디지몬 온라인 v1.5 게임의 서버 에뮬레이터를 구현하는 것을 목표로 하고 있습니다.
서버 로직만을 구현할 뿐, 게임과 관련한 그 어떤 저작권도 침해하지 않습니다. 또한 이 프로젝트를 이용한 서버 운영은 게임산업법에 의거하여 엄격히 금지됩니다.
프로젝트 사용에 따른 법적 책임은 개인에게 귀속됩니다.

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

Copyright 2022. Jung Hyun Kim (jidoc01).