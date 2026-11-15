// A command call's one name argument continued on a later line with `.`, `?.` or `*.`,
// which Groovy reads as `a.foo(bar.baz)`, next to name arguments that end their statement.
def tasks = [:]
tasks.createDockerfileOfficialSlim.mustRunAfter tasks
    .createDockerfileOfficialFull // the full image first
a.foo bar
    ?.baz()
a.foo bar

    *.baz
foo bar
    .baz
x = a.foo bar
    .baz

if (c) a.b key
else z()
a.b key
-1
a.b key
++i

void f() {
    a.foo bar
        .baz
}
