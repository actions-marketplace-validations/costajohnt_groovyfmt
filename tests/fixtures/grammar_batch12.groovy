// Constructs the twelfth grammar batch added: a generic type as a switch case label,
// `final (a, b) = ...`, record component defaults, a string-headed command call with
// several arguments, a script method whose only modifier is an annotation, and closure
// varargs with a default; next to the shapes that stay as they were.
import groovy.transform.CompileStatic

def kind(v) {
    switch (v) {
        case List<String>:
            return 1
        case Map<String, Integer>:
            return 2
        case a < b:
            return 3
    }
    def r = switch (v) {
        case List<String> -> 1
        case List<Integer>, Map<String, Integer> -> 2
        default -> 0
    }
    r
}

final (a, b) = [1, 2]
final def (c, d) = [3, 4]
final x = 1

def m() {
    given:
    final (p, q) = [1, 2]
    expect:
    p + q == 3
}

record R(boolean a, boolean b = false) {}
record S(int a) {}

"foo" p, 2
"${op}" p, 2
"foo" p
"foo" "x", 2

@CompileStatic foo(Integer id = 0) {}
@Deprecated
plus(int a, int b) {
    a + b
}
@Deprecated def field = 1

def varargs = { long... z = 123 -> z }
def annotated = { @Deprecated final def String... z -> z }
def plain = { String... z -> z }
