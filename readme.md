# poc rust ffi bridge

## 困ったこと

通信ライブラリ（Cライブラリ）においてデータの転送がcallback関数ないで処理させる

通信制御スレッドにcallback関数を与えて、計測データが一定数蓄積されるとcallback関数が実行される

callback関数実行中は通信制御がブロックされるのですぐに値をコピー⇒別スレッドへの転送を行いたい

RustからFFIにcallback関数を渡す場合、``extern ”C" fn``のポインタを渡す。

Rust関数は環境をキャプチャできないので転送先スレッドを含んだcallback関数をFFIに渡すことができない

## 解決案

- Rustオブジェクトを引数にできるラッパー関数を用意する
- ラッパー関数内でRustオブジェクトをキャプチャした関数ポインタを作製
- 上記関数ポインタをcallback関数としてライブラリ関数に渡す

## 参考

[Rust と C言語 をコールバックで行き来する（Cブリッジが必要なVer） | d.sunnyone.org](http://d.sunnyone.org/2016/04/rust-c-cver.html)