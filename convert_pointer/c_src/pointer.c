#include <stddef.h>
// printfに必要？
#include <stdio.h>
// 配列に必要？
#include <stdlib.h>


int * arr_2;
size_t size_2;

int sum(const int *arr, size_t size)
{
    int total = 0;
    for (size_t i = 0; i < size; ++i)
    {
        total += arr[i];
    }
    return total;
}

void set_pointer(const int *arr, size_t size)
{
  arr_2 = arr;
  size_2 = size;
}

int sum_from_pointer(){
  int total = 0;
    for (size_t i = 0; i < size_2; ++i)
    {
        total += arr_2[i];
    }
    return total;
}

int *make_pointer_array(void)
{
    int* array = (int*)malloc(sizeof(int) * 3);
    array[0] = 2;
    array[1] = 4;
    array[2] = 6;
    return array;
}

int *get_sequential_array(size_t size)
{
    int *arr = (int *)malloc(size * sizeof(int));
    for (size_t i = 0; i < size; ++i)
    {
        arr[i] = i;
    }

    return arr;
}

void free_array(const int *parr)
{
    free(parr);
    printf("free\n");
}