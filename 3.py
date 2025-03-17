def overlap(x, y):
    """
    Funkcja zwraca największą liczbę k, dla której x[:k] == y[-k:].
    """
    n = len(x)
    m = len(y)
    max_k = 0
    lim = min(n, m)

    # Sprawdzamy wszystkie możliwe długości k od 1 do lim
    for k in range(1, lim + 1):
        if x[:k] == y[m - k:]:
            max_k = k
    return max_k

# Przykładowe użycie funkcji
if __name__ == "__main__":
    x = "abcdef"
    y = "xyzabc"
    print("Największe k:", overlap(x, y))
