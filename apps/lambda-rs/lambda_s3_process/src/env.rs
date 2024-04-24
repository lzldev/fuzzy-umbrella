use std::collections::HashMap;
use std::env;
use strum::EnumIter;
use strum::IntoEnumIterator;

pub trait EnvVar {
    fn var_name(var: &Self) -> &'static str;
    fn get_var_name(&self) -> &'static str {
        Self::var_name(self)
    }
}

pub type EnvHashMap<'a> = HashMap<&'a str, String>;
pub trait HashMapInternal<V>: EnvVar
where
    V: IntoEnumIterator,
{
}

pub trait EnumMapEnv<'a, V, T>
where
    T: IntoEnumIterator,
    V: IntoEnumIterator + EnvVar,
{
    fn get_map(&'a self) -> &'a EnvHashMap<'a>;
    fn return_map(map: EnvHashMap<'a>) -> Self;
    fn load_env() -> Self
    where
        Self: Sized,
    {
        let mut hash_map = EnvHashMap::<'a>::new();

        for var in V::iter() {
            let v2 = var as V;
            let name = v2.get_var_name();
            let var = env::var(name).expect(format!("{} not defined", name).as_str());
            hash_map.insert(name, var);
        }

        Self::return_map(hash_map)
    }
    fn get(&'a self, var: V) -> &String {
        let map = self.get_map();
        map.get(var.get_var_name()).unwrap() //Value has to be defined
    }
}

#[derive(EnumIter)]
pub enum LambdaEnv {
    OutputBucket,
    RedisHost,
    TursoURL,
    TursoToken,
}

// let env_vars = vec!["TURSO_CONNECTION_URL", "TURSO_AUTH_TOKEN", "WEBHOOK_SECRET"];

impl EnvVar for LambdaEnv {
    fn var_name(var: &Self) -> &'static str {
        match var {
            Self::OutputBucket => "OUTPUT_BUCKET",
            Self::RedisHost => "REDIS_HOST",
            Self::TursoURL => "TURSO_URL",
            Self::TursoToken => "TURSO_TOKEN",
        }
    }
}

pub struct EnvTwo<'env> {
    map: EnvHashMap<'env>,
}

impl<'env> EnumMapEnv<'env, LambdaEnv, LambdaEnv> for EnvTwo<'env> {
    fn get_map(&'env self) -> &'env EnvHashMap<'env> {
        &self.map
    }

    fn return_map(map: EnvHashMap<'env>) -> Self {
        Self { map }
    }
}
