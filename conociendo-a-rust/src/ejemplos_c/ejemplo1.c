#include <stdio.h>

int main(int argc, char const *argv[])
{
    const int numeros_primos[] = {2, 3, 5, 7, 13};
    const int SIZE = sizeof(numeros_primos) / sizeof(int);
    for (int i = 0; i < SIZE; ++i){
        printf("%d \n", numeros_primos[i]);
    }
    
    return 0;
}
