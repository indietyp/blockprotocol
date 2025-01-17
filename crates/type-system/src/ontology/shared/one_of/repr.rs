use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use tsify::Tsify;

use crate::{repr, ParseOneOfError, PropertyValues};

#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OneOf<T> {
    // TODO: tsify doesn't seem to let us override the type of this without losing the generic
    //  see https://github.com/madonoharu/tsify/issues/4
    //  we want something like #[cfg_attr(target_arch = "wasm32", tsify(type = "[T, ...T[]]"))]
    #[serde(rename = "oneOf")]
    pub possibilities: Vec<T>,
}

impl TryFrom<OneOf<repr::PropertyValues>> for super::OneOf<PropertyValues> {
    type Error = ParseOneOfError;

    fn try_from(one_of_repr: OneOf<repr::PropertyValues>) -> Result<Self, Self::Error> {
        let inner = one_of_repr
            .possibilities
            .into_iter()
            .map(|ele| ele.try_into().map_err(ParseOneOfError::PropertyValuesError))
            .collect::<Result<Vec<_>, Self::Error>>()?;

        Self::new(inner).map_err(ParseOneOfError::ValidationError)
    }
}

impl<T, R> From<super::OneOf<T>> for OneOf<R>
where
    R: From<T>,
{
    fn from(one_of: super::OneOf<T>) -> Self {
        let possibilities = one_of
            .possibilities
            .into_iter()
            .map(std::convert::Into::into)
            .collect();
        Self { possibilities }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    mod one_of {
        use super::*;
        use crate::utils::tests::{
            check_repr_serialization_from_value, ensure_repr_failed_deserialization,
        };

        #[test]
        fn one() {
            check_repr_serialization_from_value(
                json!({
                    "oneOf": ["A"]
                }),
                Some(OneOf {
                    possibilities: ["A".to_owned()].to_vec(),
                }),
            );
        }

        #[test]
        fn multiple() {
            check_repr_serialization_from_value(
                json!({
                    "oneOf": ["A", "B"]
                }),
                Some(OneOf {
                    possibilities: ["A".to_owned(), "B".to_owned()].to_vec(),
                }),
            );
        }

        #[test]
        fn additional_properties() {
            ensure_repr_failed_deserialization::<OneOf<()>>(json!({
                "oneOf": ["A", "B"],
                "additional": 10,
            }));
        }
    }
}
