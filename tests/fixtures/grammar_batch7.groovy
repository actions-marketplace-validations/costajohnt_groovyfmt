// Constructs the seventh grammar batch added: spaced type arguments in a parameter type,
// `yield` and `var` as names, and an unspaced ternary with a list or map first branch
// (Groovy 2.4 and 3), next to the safe indexes and yield statements that stay as they were.
class Batch7 {
    Batch7(Map < String, String > m) {}

    def f(Map < String, List < String > > m, int n) {
        Map < String, String > copy = [:]
        def l = { a, yield -> yield.call(a) }
        def g = { yield -> yield(1) }
        var message : "Hi there"
        var x = 1
        for (var y : m.keySet()) {}
        def r = n.a()?[x: 1]:[x: 2]
        def s = n?[1]:[2]
        def t = m?[n]
        def u = n ? m?[n] : null
        assert m?[n] : 'missing'
    }

    def sw(int n) {
        def v = switch (n) {
            case 1 -> yield 5
            case 2 -> { yield (6) }
            default -> yield -1
        }
        switch (v) {
            case m?[n]: break
        }
    }
}
