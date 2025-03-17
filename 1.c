#include <stdio.h>

int main(void) {
    char *p = "#include <stdio.h>%cint main(void){%c    char *p = %c%s%c;%c    printf(p, 10, 10, 34, p, 34, 10, 10, 10, 10);%c    return 0;%c}%c";
    printf(p, 10, 10, 34, p, 34, 10, 10, 10, 10);
    return 0;
}
