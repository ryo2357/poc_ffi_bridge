#include <stdio.h>
// 関数ポインタのプロトタイプ宣言
typedef int (*FUNCPTR)(int a,int b);

// 関数ポインタに格納する関数の定義
int multiple(int a,int b)
{
    return(a*b);
}

int divide(int a,int b)
{
    return(a/b);
}

void c_main()
{
    int a,b;
    int result;
    FUNCPTR tFuncPtr;

    a=8;
    b=4;

    tFuncPtr=&multiple;
    result=(tFuncPtr)(a,b);
    printf("Answer is %d\n",result);

    tFuncPtr=&divide;
    result=(tFuncPtr)(a,b);
    printf("Answer is %d\n",result);
}