#include <assert.h>
#include <stdio.h>

// clang -c cc/add.c -o cc/add.o
// ar -rc cc/libadd.a cc/add.o
// RUSTFLAGS='-L ./cc ' cargo run 
int add_from_c(int a, int b) {
    return a + b;
}


extern int add(int a, int b);


// clang cc/add.c -o add -lword_counter -L./target/debug
// LD_LIBRARY_PATH=./target/debug ./add
// int main() {


//     int result = add(1, 2);

//     assert(result == 3);

//     return 0;
// }