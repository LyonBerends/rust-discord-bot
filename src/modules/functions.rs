use rand::Rng;

pub fn randomcase(s : String) -> String {
    let mut out = String::with_capacity(s.capacity());
    let mut rng = rand::thread_rng();
    for c in s.chars() {
        if rng.gen_range(0..2) == 0 {
            out.insert(out.len(),c.to_ascii_lowercase());
        } else {
            out.insert(out.len(),c.to_ascii_uppercase());
        }
    }
    out
}