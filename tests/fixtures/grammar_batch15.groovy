// Constructs the fifteenth grammar batch added: a method named `pipeline` taking a closure
// and followed by an operator or a dot, a closure or block whose whole body is a command
// call with one name argument, and a spaced call with a lambda argument; next to the
// shapes that stay as they were.
def pipeline(Closure c) { 6 }
pipeline { a = 1 } | 1
pipeline { a = 1 }.with { println it }
pipeline { a }?.with { println it }
assert pipeline { a } == 6
pipeline { a } + 1
pipeline {
    agent any
    stages {
        stage('x') {
            steps {
                sh "echo ${'}'}"
                echo '}' // }
            }
        }
    }
}

def c = { x -> println x }
[1, 2].each { println it }
for (k in [3]) { println k }
[4].each { v ->
    println v
}

configurations.all (Configuration it) -> {
    it.transitive = false
}
def r = foo () -> {
    1
}
foo (a, b) -> a + b
foo (a)

a {
    b {
        foo(x: '''
bar
''') {
            c()
        }
        d()
    }
}
