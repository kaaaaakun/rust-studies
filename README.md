簡単な実行環境：https://play.rust-lang.org/

|用語|解説|
|---|---|
|トレイト（Trait）||
|コピートレイト（Copy Trait）|束縛したオブジェクトがコピートレイトを実装したデータ型の変数から別の変数に束縛するときは，所有権は移動せず，値をコピーして新しいオブジェクト（そして所有権）を作成します．|
|シャドーイング|あるスコープのさらにローカルなスコープにおいても外側にある変数名と同じ名前で新しく束縛できます．このとき，外側の変数はローカルから隠れること|
|スライス|連続した要素を指し示すもの|
|||

--- 
2024/12/30
勉強方針
- rust入門読む
- 週7で30分
- 22:30~23:00
- 一旦の目標：- rust入門を読み切って、コマンド実装をしてみる。その先にやることはまた考える。
- [リポジトリ](https://github.com/kaaaaakun/rust-studies/issues)
- 振り返り：最初は週１くらい
- 誰かいない日はコマンドの実装を進める。
