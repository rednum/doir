macro_rules! try_eq {
    ($e1: expr, $e2: expr) => (if $e1 != $e2 { return Err(format!("Expected expressions to be equal, got {} and {} (see {} {})", $e1, $e2, module_path!(), line!())) })
}

macro_rules! try_gt {
    ($e1: expr, $e2: expr) => (if $e1.cmp(&$e2) != Ordering::Greater { return Err(format!("Expected {} > {} (see {}, line {})", $e1, $e2, module_path!(), line!())) })
}

macro_rules! try_lt {
    ($e1: expr, $e2: expr) => (if $e1.cmp(&$e2) != Ordering::Less { return Err(format!("Expected {} < {} (see {}, line {})", $e1, $e2, module_path!(), line!())) })
}
