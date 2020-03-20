int t1 := 0;
int t2 := 1;
for int i := 0; i < 20; i := i + 1 {
    int next := t1 + t2;
    t1 := t2;
    t2 := next;
    print t1;
}

if t1 > 6764 {
    print 2 + 2;
}