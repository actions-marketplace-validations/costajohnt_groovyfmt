// Constructs the tenth grammar batch added: an else after an if whose body is a command
// chain, annotations after `def`, a classic `for` declaration without a value, `default`
// and `case` as later named-argument keys, and (Groovy 4) switch arrow cases with name,
// type and several labels, next to the forms around them that stay as they were.
@interface Shared {}

def foo(Map m) { m }
def input(Map m) { m }

class Batch10 {
    def @Shared String loader
    def @Deprecated count = 1
    def static @Deprecated w
    def plain = 2

    def ifs(a, b, c) {
        if (a) or (b) or (c) {
        }
        else {
        }
        if (a) or (b) or (c) { println a }
        else { println b }
        if (a) foo b bar c
        else {}
        if (a) foo b
        else {}
        if (a) foo(b).bar(c)
        else {}
        if (a) foo b bar c
        if (a) or (b) or (c) {
        }
    }

    def locals() {
        def @Deprecated x = 1
        def @Deprecated String y = 'y'
        @Deprecated def (int q, int r) = [1, 2]
        def z = 3
        [x, y, q, r, z]
    }

    def loops(List l) {
        for (int i; i < 3; i++) {}
        for (int i; ;) { break }
        for (int i = 0; i < 3; i++) {}
        int k = 0
        for (k = 0; k < 5; k++);
        for (@Deprecated String s in l) {}
        for (s in l) {}
        k
    }
}

foo a: 1, default: 2
input name: "x", type: "bool", default: false
foo a: 1, case: 2
foo default: 2
foo(a: 1, default: 2)
switch (1) {
    case 1: foo a: 1
        break
    default: foo b: 2
}
switch (1) { case 1: foo; default: foo b: 3 }
switch (1) { case 1: foo default: 3 }

// Groovy 4 only from here.
enum Month { APRIL, MAY, JUNE }

def seasons(Month m, Object v) {
    def x = switch (m) {
        case APRIL -> 1
        default -> 2
    }
    switch (v) {
        case String -> { println 's' }
        case Integer, Long -> println 'n'
        case 1 -> println 'one'
        default -> println 'other'
    }
    def y = switch (m) {
        case APRIL, MAY -> 'spring'
        case JUNE -> 'summer'
        case Month.JUNE -> 'still summer'
    }
    def z = switch (3) {
        case 6, 9: yield 'a'
        case 1: yield 'b'
        default: yield 'c'
    }
    def f = { n -> n + 1 }
    [x, y, z, f]
}
