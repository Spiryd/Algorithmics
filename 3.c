#include <stdio.h>
#include <string.h>

// Funkcja zwraca największe k, takie że x[0:k] == y[m-k:m]
// (uwzględniamy indeksowanie od 0, czyli x[0:k] odpowiada x[1:k] w notacji matematycznej)
int longest_prefix_suffix(const char *x, const char *y) {
    int n = strlen(x);
    int m = strlen(y);
    int max_possible = (n < m) ? n : m;
    int k;
    // Sprawdzamy od największego możliwego k do 0
    for (k = max_possible; k >= 0; k--) {
        // Porównujemy prefiks x o długości k z sufiksem y o długości k
        if (strncmp(x, y + m - k, k) == 0) {
            return k;
        }
    }
    return 0; // Zawsze istnieje k = 0, gdyż pusty ciąg jest wspólny
}

int main(void) {
    const char *x = "Hello";
    const char *y = "Olley";
    
    int k = longest_prefix_suffix(x, y);
    printf("Największe k, takie że \"%.*s\" = \"%.*s\" wynosi: %d\n",
           k, x, k, y + strlen(y) - k, k, k);
    return 0;
}
