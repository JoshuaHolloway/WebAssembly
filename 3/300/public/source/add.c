#include <stdlib.h>
#include <emscripten.h>
#include <stdio.h>

// EMSCRIPTEN_KEEPALIVE
// int Add(int value1, int value2) 
// {
//   printf("HELLO! from add.c!\n");
//   return (value1 + value2); 
// }

EMSCRIPTEN_KEEPALIVE
int Add(int* x)
{
  printf("JOSH: Add.c\n");
  for (int i = 0; i < 4; i++)
  {
    printf("%d\n", x[i]);
  }

  return 0;
}