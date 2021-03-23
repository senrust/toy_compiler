#include <stdio.h>

int echo() { 
    printf("hello\n"); 
}

int add(int x, int y) { 
    printf("add result is %d\n", x+y); 
    return x+y;
}

int add3(int x, int y, int z) { 
    printf("add result is %d\n", x+y+z); 
    return x+y+z;
}

int add4(int x, int y, int z, int a) { 
    printf("add result is %d\n", x+y+z+a); 
    return x+y+z+a;
}

int add5(int x, int y, int z, int a, int b) { 
    printf("add result is %d\n", x+y+z+a+b); 
    return x+y+z+a+b;
}

int add6(int x, int y, int z, int a, int b, int c) { 
    printf("add result is %d\n", x+y+z+a+b+c); 
    return x+y+z+a+b+c;
}