def longest_prefix_suffix(x, y):
    n = len(x)
    m = len(y)
    max_possible = min(n, m)
    # Sprawdzamy od największego możliwego k do 0
    for k in range(max_possible, -1, -1):
        if x[:k] == y[m - k:]:
            return k
    return 0  # Gwarantowane, że zawsze zwróci przynajmniej 0

# Przykład użycia
x = "Hello"
y = "olleY"

k = longest_prefix_suffix(x, y)
print(f'Największe k, takie że "{x[:k]}" = "{y[len(y)-k:]}" wynosi: {k}')
