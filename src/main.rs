macro_rules! switch {
    ($s:ident;$(case $c:literal:$v:expr;)+default:$d:expr;) => {{
        match $s {
            $($c => $v,)+
            _ => $d
        }
    }};
    ($s:ident$(;case $c:literal:$v:expr);+;case $cd:literal:$d:expr;) => {{
        $(if $s == $c {$v} else)+{$d}
    }}
}

fn main() {
    let n = 5;
    let a = switch!{
        n;
        case 1: 10;
        case 2: 20;
        case 3: 30;
        default: 69;
    };
    let b = switch!{
        n;
        case 5: "(6, 9)";
        case 6: "(0, 0)";
        //case 7: "this breaks the macro";
    }.split(&['(',')',',',' ']).filter_map(|x|x.parse().ok()).fold((0,0), |a,e|(a.1, e));
    println!("Hello, world! {}, {:?}", a, b);
}
