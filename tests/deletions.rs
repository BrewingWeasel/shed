mod common;
use shed::{parse, Config};

#[test]
fn simple_delete_line_one() {
    run_test!("Hello everyone!", "1d", "");
}

#[test]
fn delete_lines_in_range() {
    run_test!(
        "1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20",
        "1,18d",
        "19\n20\n"
    );
}

#[test]
fn delete_lines_in_range_2() {
    run_test!(
        "1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20",
        "3,19d",
        "1\n2\n20\n"
    );
}

#[test]
fn delete_lines_step() {
    run_test!(
        "1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20",
        "0~2d",
        "1
3
5
7
9
11
13
15
17
19
"
    );
}

#[test]
fn delete_lines_matching() {
    run_test!(
        "hi
lol
xd
lol
xd
xd
xd",
        "/lol/d",
        "hi\nxd\nxd\nxd\nxd\n"
    );
}
