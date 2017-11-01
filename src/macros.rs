macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as _, $y as _, $w as _, $h as _)
    )
);
