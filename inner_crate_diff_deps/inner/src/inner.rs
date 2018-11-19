use rand::{thread_rng, Rng};

pub fn inner() -> usize {
    thread_rng().gen_range(0, 99)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
