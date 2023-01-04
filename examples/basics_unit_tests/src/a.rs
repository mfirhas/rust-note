pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn priv_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
#[test]
fn test_priv_outside() {
    assert_eq!(5, priv_add(2, 3));
}

#[cfg(test)]
mod tests {
    use super::{add, priv_add};

    #[test]
    fn test_add() {
        dbg!("DEBUG ME!!!");
        println!("PRINT ME!!!");
        assert_eq!(5, add(3, 2));
    }

    #[test]
    fn test_priv_add() {
        assert_eq!(5, priv_add(2, 3));
    }

    #[test]
    fn test_add_result() -> Result<(),String> {
        let expected = 5;
        let result = add(3,2);
        if result != expected {
            return Err("ERROR!!!".to_owned());
        }
        Ok(())
    }
}