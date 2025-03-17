#include <stdio.h>
#include <string.h>

int is_prefix(const char *prefix, const char *str) {
    while (*prefix) {
        if (*prefix != *str)
            return 0; // Nie jest prefiksem
        prefix++;
        str++;
    }
    return 1; // Jest prefiksem
}

int is_suffix(const char *suffix, const char *str) {
    size_t len_suffix = strlen(suffix);
    size_t len_str = strlen(str);
    if (len_suffix > len_str)
        return 0; // Nie może być postfiksem, jeśli jest dłuższy
    return strcmp(str + len_str - len_suffix, suffix) == 0;
}

int main(void) {
    const char *text = "Hello, World!";
    const char *prefix = "Hello";
    const char *suffix = "World!";

    // Sprawdzenie czy 'prefix' jest prefiksem 'text'
    if (is_prefix(prefix, text))
        printf("\"%s\" jest prefiksem \"%s\"\n", prefix, text);
    else
        printf("\"%s\" nie jest prefiksem \"%s\"\n", prefix, text);

    // Sprawdzenie czy 'suffix' jest postfiksem 'text'
    if (is_suffix(suffix, text))
        printf("\"%s\" jest postfiksem \"%s\"\n", suffix, text);
    else
        printf("\"%s\" nie jest postfiksem \"%s\"\n", suffix, text);

    return 0;
}
