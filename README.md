# 某ゲームのイベントステートマシンを設計
## State
- Normal, Super, Fireの基本3状態
- Starred, NonStarredの無敵か否かを表す状態

## Event
- AttackedByEnemy：敵に攻撃を受ける
- GetMach：キノコGet
- GetFlower：ファイアフラワーGet
- GetStar：スターGet
- TimePassed：スター状態終了$$

## 某を模した状態遷移マシン
<image src="img/event_state_machine.png">
