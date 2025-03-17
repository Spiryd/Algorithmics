#include <stdio.h>
#include <string.h>

// Funkcja wyszukująca wzorzec w tekście przy użyciu memcmp.
// Zwraca indeks pierwszego wystąpienia wzorca w tekście lub -1, jeśli wzorzec nie został znaleziony.
int naive_match(const char *text, const char *pattern) {
    int n = strlen(text);
    int m = strlen(pattern);

    // Jeśli wzorzec jest dłuższy od tekstu, nie ma sensu szukać
    if (m > n) return -1;

    // Iterujemy po wszystkich możliwych pozycjach, gdzie wzorzec mógłby się zmieścić
    for (int i = 0; i <= n - m; i++) {
        // Porównujemy fragment tekstu o długości m z wzorcem
        if (memcmp(text + i, pattern, m) == 0) {
            return i; // Wzorzec znaleziony na pozycji i
        }
    }
    return -1; // Wzorzec nie został znaleziony
}

int main(void) {
    const char *text = "Hello, world!";
    const char *pattern = "world";
    int pos = naive_match(text, pattern);

    if (pos >= 0) {
        printf("Wzorzec znaleziony na pozycji %d.\n", pos);
    } else {
        printf("Wzorzec nie został znaleziony.\n");
    }
    return 0;
}
