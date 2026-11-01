{
    f = {
        a,
        b,
    }:
        # about the body
        a + b;
    g = {
        a,
        b ? 1,
        ...
    } @ args:
        # about the body
        a + b;
}
