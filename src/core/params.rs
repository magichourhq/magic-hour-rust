use std::collections::BTreeMap;
use serde_json::json;
use crate::{Error, SdkResult};
#[allow(unused)]
#[derive(Default, Clone, Debug)]
pub enum QueryStyle {
    #[default]
    Form,
    SpaceDelimited,
    PipeDelimited,
    DeepObject,
}
#[derive(Debug, Clone, Default)]
pub struct QueryParams {
    pub params: Vec<(String, String)>,
}
impl QueryParams {
    #[allow(unused)]
    pub fn add<T: serde::Serialize>(
        &mut self,
        name: &str,
        val: &T,
        style: QueryStyle,
        explode: bool,
    ) {
        let serde_val = json!(val);
        match style {
            QueryStyle::Form => self.add_form(name, &serde_val, explode),
            QueryStyle::SpaceDelimited => {
                self.add_space_delimited(name, &serde_val, explode)
            }
            QueryStyle::PipeDelimited => {
                self.add_pipe_delimited(name, &serde_val, explode)
            }
            QueryStyle::DeepObject => self.add_deep_object(name, &serde_val),
        }
    }
    #[allow(unused)]
    pub fn add_option<T: serde::Serialize>(
        &mut self,
        name: &str,
        val: &Option<T>,
        style: QueryStyle,
        explode: bool,
    ) {
        if let Some(v) = val {
            self.add(name, v, style, explode)
        }
    }
    #[allow(unused)]
    pub fn add_patch<T: serde::Serialize>(
        &mut self,
        name: &str,
        val: &super::patch::Patch<T>,
        style: QueryStyle,
        explode: bool,
    ) {
        if let super::patch::Patch::Value(v) = val {
            self.add(name, v, style, explode);
        }
    }
    fn add_form(&mut self, name: &str, val: &serde_json::Value, explode: bool) {
        match val {
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_) => {
                self.params.push((name.into(), val.to_string()))
            }
            serde_json::Value::String(s_val) => {
                self.params.push((name.into(), s_val.into()))
            }
            serde_json::Value::Array(values) => {
                if explode {
                    values.iter().for_each(|v| self.add_form(name, v, explode));
                } else {
                    let comma_joined = values
                        .iter()
                        .map(format_string_param)
                        .collect::<Vec<String>>()
                        .join(",");
                    self.params.push((name.into(), comma_joined));
                }
            }
            serde_json::Value::Object(map) => {
                if explode {
                    map.into_iter().for_each(|(k, v)| self.add_form(k, v, explode));
                } else {
                    let mut encoded_chunks = vec![];
                    map.into_iter()
                        .for_each(|(k, v)| {
                            encoded_chunks
                                .extend([k.to_string(), format_string_param(v)])
                        });
                    self.params.push((name.into(), encoded_chunks.join(",")))
                }
            }
        }
    }
    fn add_space_delimited(
        &mut self,
        name: &str,
        val: &serde_json::Value,
        explode: bool,
    ) {
        match (val, explode) {
            (serde_json::Value::Array(values), false) => {
                let space_joined = values
                    .iter()
                    .map(format_string_param)
                    .collect::<Vec<String>>()
                    .join(" ");
                self.params.push((name.into(), space_joined))
            }
            _ => {
                self.add_form(name, val, explode);
            }
        }
    }
    fn add_pipe_delimited(
        &mut self,
        name: &str,
        val: &serde_json::Value,
        explode: bool,
    ) {
        match (val, explode) {
            (serde_json::Value::Array(values), false) => {
                let space_joined = values
                    .iter()
                    .map(format_string_param)
                    .collect::<Vec<String>>()
                    .join("|");
                self.params.push((name.into(), space_joined))
            }
            _ => {
                self.add_form(name, val, explode);
            }
        }
    }
    fn add_deep_object(&mut self, name: &str, val: &serde_json::Value) {
        match val {
            serde_json::Value::Object(map) => {
                map.iter()
                    .for_each(|(k, v)| self.add_deep_object(&format!("{name}[{k}]"), v));
            }
            serde_json::Value::Array(values) => {
                values
                    .iter()
                    .enumerate()
                    .for_each(|(i, v)| {
                        self.add_deep_object(&format!("{name}[{i}]"), v);
                    });
            }
            _ => self.params.push((name.into(), format_string_param(val))),
        }
    }
}
#[allow(unused)]
pub fn format_form_urlencoded<T: serde::Serialize>(
    val: &T,
    style: BTreeMap<&str, QueryStyle>,
    explode: BTreeMap<&str, bool>,
) -> SdkResult<String> {
    let json_val = json!(val);
    if let serde_json::Value::Object(obj) = json_val {
        let mut form_data = QueryParams::default();
        for (name, val) in &obj {
            let style = style.get(&name.as_str()).cloned().unwrap_or_default();
            let explode = explode
                .get(&name.as_str())
                .cloned()
                .unwrap_or(matches!(& style, QueryStyle::Form));
            form_data.add(name, val, style, explode);
        }
        serde_urlencoded::to_string(&form_data.params)
            .map_err(|e| Error::Custom(format!("failed form-urlencoding body: {e}")))
    } else {
        Err(
            Error::Custom(
                "x-www-form-urlencoded data must be an object at the top level".into(),
            ),
        )
    }
}
#[allow(unused)]
pub fn format_string_param<T: serde::Serialize>(val: &T) -> String {
    let serde_val = serde_json::json!(val);
    if let serde_json::Value::String(str) = serde_val {
        str
    } else {
        serde_val.to_string()
    }
}
