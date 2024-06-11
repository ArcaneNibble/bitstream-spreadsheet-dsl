pub mod property;

fn is_valid_ident(s: &str) -> bool {
    if s == "_" {
        return false;
    }

    let mut chars = s.chars();

    let c0 = chars.next();
    if let Some(c0) = c0 {
        if c0 != '_' && !unicode_ident::is_xid_start(c0) {
            return false;
        }
    } else {
        return false;
    }

    for ci in chars {
        if !unicode_ident::is_xid_continue(ci) {
            return false;
        }
    }

    return true;
}
