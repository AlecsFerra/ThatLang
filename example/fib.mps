int t1 := 0;
int t2 := 1;
for int i := 1; i < 20; i := i + 1 {
    int next := t1 + t2;
    t1 := t2;
    t2 := next;
}