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

## 検証用クレート

### poc_cc

cのプログラムをプロジェクト内で記述、コンパイルを行う

### poc_callback

callback関数をCで実装し、挙動を確認する

## 参考

[Rust と C言語 をコールバックで行き来する（Cブリッジが必要なVer） | d.sunnyone.org](http://d.sunnyone.org/2016/04/rust-c-cver.html)

[【C言語】関数ポインタを利用して呼び出す関数を動的に変更する](https://www.kishiro.com/programming/c/function_pointer.html)

[How to receive a callback from Rust in C/C++ (C-API/FFI) - help - The Rust Programming Language Forum](https://users.rust-lang.org/t/how-to-receive-a-callback-from-rust-in-c-c-c-api-ffi/10270/9)

[Passing a callback to a C function : rust](https://www.reddit.com/r/rust/comments/b7e0ty/passing_a_callback_to_a_c_function/)