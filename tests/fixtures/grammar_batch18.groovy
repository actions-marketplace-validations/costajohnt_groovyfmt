// Constructs the eighteenth grammar batch added: a closure's last parameter with a bare name as
// its default, `{ a, List<String> e = g -> e }`, which Groovy reads as the parameter's default
// before the closure's `->`, not as a lambda `g -> e`; next to the declarations and
// assignments with a name value that stay as they were.
def g = ['a']
def h = [k: 'v']
def x = [k: [1]]
def c1 = { String a = 'x', List<String> e = g -> e }
def c2 = { a, Map<String, String> m = h -> m }
def c3 = { a, List<String> e = g, b -> e }
def c4 = { Map<String, List<Integer>> m = x -> m }
def c5 = { List<String> e = g -> e }
def c6 = { a, List e = g -> e }
def c7 = { a, e = g -> e }
def c8 = { e = g -> e }
def c9 = { String e = g -> e }
def c10 = { a, final String... e = g -> e }
def c11 = { a, def e = g ->
    e
}

def d1 = { String e = g, f = h }
def d2 = { e = g }
def d3 = { List<String> e = g.reverse() -> e }
