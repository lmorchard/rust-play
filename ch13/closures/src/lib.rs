use std::collections::HashMap;

#[derive(Debug)]
pub struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        if self.values.contains_key(&arg) {
            // TODO: This feels really awkward and maybe not right
            return self.values.get(&arg).unwrap().clone();
        }
        let v = (self.calculation)(arg);
        self.values.insert(arg, v);
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a * 2);

        let v1 = c.value(3);
        let v2 = c.value(5);

        assert_eq!(v1, 3 * 2);
        assert_eq!(v2, 5 * 2);
    }
}
