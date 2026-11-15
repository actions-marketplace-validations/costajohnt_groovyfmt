// Constructs the fourteenth grammar batch added: a lambda whose body is a command call,
// Groovy 2.4's empty argument, list element and map entry (kept as written), and rare
// forms from Groovy's own parser tests; next to the shapes that stay as they were.
[1, 2].each(v -> println v)
def f = (x) -> println x
def g = x -> x + 1
def h = (a, b) -> {
    a + b
}
def k = s -> s as String
[3].each(v -> v * 2)
[4].each { v -> println v }

foo(a,,b)
foo(a, , b)
def modes = ["off", "cool", , "heat"]
def m = [a: 1,, b: 2]
def l = [
    'x',
    ,
    'y'
]
foo(a, b)
def n = [a, b,]

for (def (int i, int j) = [0, 0]; i < 5; i++) {
    println i + j
}
interface I<T extends @Deprecated CharSequence & @Deprecated Runnable> {}
def named = A[x: 1, y: 2]
def spread = A[*: m]
def several = a?[1, 2]
def safeNamed = A?[name: 1]
def cond = a[b ? c : d]
def s1 = '''a='b''''
def s2 = """say "hi""""
def s3 = '''it's'''
