use serde::Deserialize;
#[derive(Clone, Debug, Default)]
pub enum Patch<T> {
    #[default]
    Undefined,
    Null,
    Value(T),
}
impl<T> Patch<T> {
    #[allow(unused)]
    pub fn new(val: T) -> Self {
        Self::Value(val)
    }
    #[allow(unused)]
    pub fn is_null(&self) -> bool {
        matches!(self, Patch::Null)
    }
    #[allow(unused)]
    pub fn is_undefined(&self) -> bool {
        matches!(self, Patch::Undefined)
    }
}
impl<T> From<Option<T>> for Patch<T> {
    fn from(opt: Option<T>) -> Patch<T> {
        match opt {
            Some(v) => Patch::Value(v),
            None => Patch::Null,
        }
    }
}
impl<T> serde::Serialize for Patch<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Undefined => todo!(),
            Self::Null => serializer.serialize_none(),
            Self::Value(v) => v.serialize(serializer),
        }
    }
}
impl<'de, T> serde::Deserialize<'de> for Patch<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}
#[allow(unused)]
pub fn deserialize_required_nullable<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let val = Option::<T>::deserialize(deserializer)?;
    Ok(val)
}
