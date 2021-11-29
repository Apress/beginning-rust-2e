/* It prints:
1 2 3 :1 */
#include <stdio.h>
int main() {
    int limit = 4;
    for (int n = 1; n < limit + 2; n++) {
        limit -= 1;
        printf("%d ", n);
    }
    printf(":%d ", limit);
}
