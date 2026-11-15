// Constructs the eleventh grammar batch added: a method with annotations, modifiers and a
// return type after another member and a comment, in a class and in a script, next to
// fields of the same shape that stay fields.
class Batch11 {
    def x = 1
    // doc
    @Deprecated
    private boolean isLeaf() {
        true
    }
    def y = 2;
    /** doc */ @SuppressWarnings('X') private static boolean isRoot(Object n) { n == null }
    /* c */
    @Deprecated private String name() { 'a' }
    int z = 3 // trailing
    @Deprecated
    protected List<String> names() { [] }
    // c
    @Deprecated
    private int[] nums() { null }
    /**
     * doc
     */
    @Deprecated
    private static final Map<String, Integer> counts(int a) throws Throwable { [:] }

    // c
    @Deprecated private boolean flag
    // c
    private boolean other = true
    /** doc */
    @Deprecated
    private static String NAME = 'n'
    // c
    private untyped
    // c
    static TTL = 900
    // c
    private boolean a, b
    // c
    Batch11() {}
    // c
    @Deprecated private Batch11(int v) {}
}

def top = 1
// doc
@Deprecated
private boolean leaf() {
    true
}
/** doc */ @Deprecated static String label() { 'a' }
// c
@Deprecated final int count = 2
