use std::{collections::HashMap, env};

use strum::IntoEnumIterator;

pub trait EnvEnum {
    fn var_name(var: &Self) -> &'static str;
    fn get_var_name(&self) -> &'static str {
        Self::var_name(self)
    }
}

pub type EnvMap = HashMap<&'static str, String>;

pub trait EnvContainer<T>
where
    Self: Sized,
    T: EnvEnum + IntoEnumIterator,
{
    fn get_map(&self) -> &EnvMap;
    fn with_env_map(map: EnvMap) -> Self;

    fn load_env() -> Self {
        let mut map = HashMap::<&'static str, String>::new();

        for var in T::iter() {
            let key = var.get_var_name();
            let value = env::var(key).expect(format!("Env Var {} not defined.", key).as_str());

            map.insert(key, value);
        }

        Self::with_env_map(map)
    }

    fn get_env_var(&self, env_var: T) -> String {
        let map = self.get_map();
        let key = env_var.get_var_name();

        map.get(key).unwrap().to_owned()
    }
}
