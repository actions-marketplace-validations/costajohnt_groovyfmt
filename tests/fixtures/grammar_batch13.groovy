#! /usr/bin/env nix-shell
#! nix-shell -i groovy
// Constructs the thirteenth grammar batch added: several `#!` lines opening a script
// (nix-shell style) and a comparison or other expression as a Groovy 4 switch arrow case
// label; next to the shapes that stay as they were.
def kind(x, a, b, c, d, e) {
    def r = switch (x) {
        case a < b -> 3
        case c > d -> 4
        case a <= b, c == d && e -> 5
        case x in [1, 2] -> 6
        case !e -> 7
        case (a) -> 8
        default -> 0
    }
    def s = switch (x) {
        case FOO -> 1
        case String -> 2
        case List<String> -> 3
        case 1 -> 4
        case APRIL, MAY -> 5
        case 2 -> { 6 }
        default -> 0
    }
    switch (x) {
        case a < b:
            return 9
    }
    r + s
}

def inc = x -> x + 1
def lt = (p, q) -> p < q
[1, 2].each(v -> println(v))
