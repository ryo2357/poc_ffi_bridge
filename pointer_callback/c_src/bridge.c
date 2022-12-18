#include <stdio.h>
#include <stdlib.h>

// opaqueなstructと想定
typedef struct _UnknownStruct {
    int unknown1;
    int unknown2;
    int theNumber;
} UnknownStruct;

typedef struct _MyStruct MyStruct;

// 数字を渡すと、数字とともにHello, Worldをprintしてもらうコールバック。
// コールバックの処理内容によってはGood Eveningになったりとか。
// このコールバックは実行結果的な数値を返す。
typedef int (*NumberFunc) (MyStruct *myStruct, int a, void *callback_data);
struct _MyStruct {
    UnknownStruct unknown; // ここがはっきりしていればRust側でも定義できる
    
    NumberFunc callback; // Cだったらここに関数ポインタをセットして動かすみたいなAPI
    void *callback_data; // callbackの実行結果が出力されるポインタがここ？
};

// 想定としては、元々のAPI的な関数達
MyStruct *mystruct_new() {
    printf("mystruct_new\n");
    return malloc(sizeof(MyStruct));
    // mallocとは動的メモリを確保する関数です。
}

void mystruct_free(MyStruct *mystruct) {
    printf("mystruct_free\n");
    free(mystruct);
}

void mystruct_set_number(MyStruct *mystruct, int a) {
    mystruct->unknown.theNumber = a;
}

void mystruct_hello(MyStruct *mystruct) {
    int a = (mystruct->callback)(mystruct, 
       mystruct->unknown.theNumber,
       mystruct->callback_data);
    printf("Hello Result: %d\n", a);
}

// ブリッジ用関数の想定
// この例的には、Cでは構造体のメンバに関数ポインタを直接代入するところを、関数呼び出しとして定義した。
void mystruct_set_callback(MyStruct *mystruct, NumberFunc callback, void *callback_data) {
    mystruct->callback = callback;
    mystruct->callback_data = callback_data;
}