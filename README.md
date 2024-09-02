# anchor-param-mre

To reproduce error, run `anchor test` on your machine, and check what it [says in the console](https://github.com/cryptopapi997/anchor-param-mre/blob/main/tests/anchor-mre.ts#L119) (should say "we send with buggy field = 1") and then check inside .anchor the program log for the value [it actually received](https://github.com/cryptopapi997/anchor-param-mre/blob/main/programs/anchor-mre/src/lib.rs#L80).
