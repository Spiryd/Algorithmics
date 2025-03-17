#include <stdio.h>
#include <string.h>

// Funkcja zwraca największą liczbę k, dla której x[0:k] == y[m-k:m]
int overlap(const char *x, const char *y) {
    int n = strlen(x);
    int m = strlen(y);
    int max = 0;
    int lim = n < m ? n : m;

    // Sprawdzamy wszystkie możliwe długości k od 1 do lim
    for (int k = 1; k <= lim; k++) {
        int ok = 1;
        // Porównujemy prefiks x[0:k] z sufiksem y[m-k:m]
        for (int i = 0; i < k; i++) {
            if (x[i] != y[m - k + i]) {
                ok = 0;
                break;
            }
        }
        if (ok) {
            max = k;
        }
    }
    return max;
}

// Przykładowe użycie funkcji
int main() {
    const char *x = "abcdef";
    const char *y = "xyzabc";
    int k = overlap(x, y);
    printf("Najwieksze k: %d\n", k);
    return 0;
}
