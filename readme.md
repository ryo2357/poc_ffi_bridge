# poc rust ffi bridge

## 困ったこと

通信ライブラリ（Cライブラリ）においてデータの転送がcallback関数内で処理させる

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

### poc_function_pointer

[【C言語】関数ポインタを利用して呼び出す関数を動的に変更する](https://www.kishiro.com/programming/c/function_pointer.html)の挙動確認

### pointer_callback

[Rust と C言語 をコールバックで行き来する（Cブリッジが必要なVer） | d.sunnyone.org](http://d.sunnyone.org/2016/04/rust-c-cver.html)の実装。挙動を確かめる

### pass_rust_object

- brige.cにRustオブジェクトをセット。セットされた関数ポインタを得る
- bridge.cの関数ポインタをmain.cのcallback関数に渡す

STATUS_STACK_OVERFLOW エラーがか解決できず挫折。

⇒ 後々の検討でbridge.cとmain.cの名前空間が衝突してるっぽいことが分かる

⇒ 直したら動いた

### pass_rust_object_2

- main.cに渡したcallback関数からstate.cの関数を呼ぶ
- state.cからセットされたRustオブジェクトを引数にcallback関数を呼ぶ

引数で返ってきたRustオブジェクトを編集できない。(exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)

### pass_rust_object_3

- state.cはRust構造体のset,get
- main.cからcallback⇒state.cから構造体をget⇒構造体の編集

### bridge_with_tokio

tokioを使った並列処理とFFIの同時実装

callback関数が同期関数なのでその中で非同期関数のSender.send()を実行する方法が思いつかない

tokioランライム内でtokioランタイムを生成できない

### bridge_with_sync

tokioでなくstd::syncだと難なくできた。動的にスレッドを増やさない場合はstd::syncのほうが楽そう

## 参考

[Rust と C言語 をコールバックで行き来する（Cブリッジが必要なVer） | d.sunnyone.org](http://d.sunnyone.org/2016/04/rust-c-cver.html)

[【C言語】関数ポインタを利用して呼び出す関数を動的に変更する](https://www.kishiro.com/programming/c/function_pointer.html)

[How to receive a callback from Rust in C/C++ (C-API/FFI) - help - The Rust Programming Language Forum](https://users.rust-lang.org/t/how-to-receive-a-callback-from-rust-in-c-c-c-api-ffi/10270/9)

[Passing a callback to a C function : rust](https://www.reddit.com/r/rust/comments/b7e0ty/passing_a_callback_to_a_c_function/)

[どうにかなりそう](https://drivingmecrazy.netlify.app/blog/rust-c-ffi/)