## 概要

色々時間はかけたがなんか使い所がいまいちなデザインパターン。
生成したオブジェクトを登録して登録したデータを引っ張ってくる。特筆すべきは登録時に生成したオブジェクト自身ではなく、そのオブジェクト自身のコピーであることである。

ただし浅いコピーなのでアドレス・ポインタはそのまま同じアドレスを指すことに注意。

## Rustでの注意点

### create_clone関数の戻り値の型

Managerがcreateすることで登録されたオブジェクトのCloneを実行する。単純にこれは`create`関数内で登録されたオブジェクトのcloneを実行するだけなわけだが、createで生成されるオブジェクトはProductを実装したオブジェクトでなくてはならないため,今回は戻り値にOption<Box<dyn Product>>と指定する。
その場合そのままproduct.cloneは利用できないなぜなら`derve(Clone)`で利用できるclone関数はSelfを返すためObjectSafe出ないからだ。
そのためObjectSafeにするためにcreate_clone内でBox::new(self.clone())でラップする

ObjectSafeじゃないと何がだめかというと戻り値の型がわからないと利用している側からメモリサイズが算出できないためちゃんと型を指定して上げる必要がある。

## dyn <trait>

このパターンもやっぱりtraitを実装するオブジェクトだけを受け入れたい.
そのためiterator patternで説明したようにBox<dyn trait>で型情報を落として上げる必要がある。
詳細は[こちら](../iterator/README.md#dyn)