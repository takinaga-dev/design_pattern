## Singletonのクラス図

![class図](./class.png)

MSingletonが十分なSingletonだと思う。Mutexとか考えずに実装するとSingletonになるので履歴として残しておくことに

Rustはstructの中にstatic変数おけないためグローバル変数として宣言してそれをget_instanceでしか扱えないようにしている。グローバル変数と言ってもpubはつけていないので`singleton.rs`以外からのアクセスはできないためアクセスできるメソッドは`get_instance/get_value/set_value`に限定されている。


[OnceCellの遅延初期化の使い方](https://docs.rs/once_cell/latest/once_cell/#lazy-initialized-global-data)をそのまま利用している。
