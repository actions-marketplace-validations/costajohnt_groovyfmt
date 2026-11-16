// Constructs the sixteenth grammar batch added: a `final` declaration with a type and names
// without values, an annotated `final` declaration as the first statement of a block, and a
// dotted generic array type after `as` and `instanceof`; next to the shapes that stay as
// they were.
final String a, b
a = 'x'
b = 'y'
final int c, d
c = 3; d = 4
final String e = 'x', f
final List<String> l, m
String g, h
final x
foo = 1

def cond = true
if (cond) {
    @Deprecated final y = 1
    println y
}
while (cond) {
    @Deprecated final String z = 'z'
    cond = false
}
for (i in [1]) {
    @Deprecated final w
}
for (int i = 0; i < 1; i++) {
    @Deprecated final v = i
    println v
}
if (cond) {
    @Deprecated def u = 1
}
[1].each { @Deprecated final p -> println p }
[1].each { @Deprecated final String q -> println q }
def cl = { @Deprecated final r -> r }

def list = []
def arr = list as java.nio.file.WatchEvent.Kind<?>[]
def isArr = arr instanceof Map.Entry<String, String>[]
def plain = list as Map.Entry[]
def gen = list as List<String>[]
def deep = list as java.util.Map.Entry<String, String>[][]
