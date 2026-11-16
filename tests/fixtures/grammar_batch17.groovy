// Constructs the seventeenth grammar batch added: a closure's or block's whole body on one
// line that is a declaration with no value, `{ Foo bar }`, which Groovy reads as a declaration
// when the type's last part starts with a capital letter; next to the one-line bodies that
// stay command calls.
def a = { Foo bar }
def b = { String s }
def c = { x -> Foo bar }
def d = { FOO bar }
def e = { java.util.List l }
def f = { foo.Bar baz }
def g = { List<String> l }
def h = { Foo[] arr }
def i = { Foo bar; }
def cond = true
if (cond) { Foo bar }
while (cond) { String s; cond = false }

def j = { foo bar }
def k = { x -> println x }
def l = { println it }
def m = [a: 1]
for (key in m) { println key }
def n = { Foo.bar baz }
def o = { _Foo bar }
def p = { this.Foo bar }
def q = { List l -> l }
