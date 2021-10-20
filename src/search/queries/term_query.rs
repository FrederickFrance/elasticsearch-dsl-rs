use super::params::*;
use super::Query;
use crate::util::*;
use crate::OptionalScalar;
use serde::ser::{Serialize, SerializeMap, Serializer};

/// Returns documents that contain an **exact** term in a provided field.
///
/// You can use the term query to find documents based on a precise value such as a price, a product ID, or a username.
///
/// To create a term query with numeric values:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// TermQuery::new("test", 123);
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::term("test", 123);
/// ```
/// To create a term query with string values and optional fields:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// TermQuery::new("test", "username")
///     .boost(2)
///     .name("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::term("test", "username")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-term-query.html>
#[derive(Debug, Clone, PartialEq)]
pub struct TermQuery {
    field: String,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    value: OptionalScalar,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [TermQuery](TermQuery)
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Term you wish to find in the provided field.
    /// To return a document, the term must exactly match the field value, including whitespace and capitalization.
    pub fn term(field: impl Into<String>, value: impl Into<OptionalScalar>) -> TermQuery {
        TermQuery::new(field, value)
    }
}

impl TermQuery {
    /// Creates an instance of [TermQuery](TermQuery)
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Term you wish to find in the provided field.
    /// To return a document, the term must exactly match the field value, including whitespace and capitalization.
    pub fn new(field: impl Into<String>, value: impl Into<OptionalScalar>) -> Self {
        Self {
            field: field.into(),
            inner: Inner {
                value: value.into(),
                boost: None,
                _name: None,
            },
        }
    }

    add_boost_and_name!();
}

impl ShouldSkip for TermQuery {
    fn should_skip(&self) -> bool {
        self.inner.value.should_skip()
    }
}

impl Serialize for TermQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("term", &hash)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            TermQuery::new("test", 123),
            json!({
                "term": {
                    "test": {
                        "value": 123
                    }
                }
            })
        );

        with_all_fields(
            TermQuery::new("test", 123).boost(2).name("test"),
            json!({
                "term": {
                    "test": {
                        "value": 123,
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            })
        );

        with_none(
            Query::bool().filter(TermQuery::new("test", None::<String>)),
            json!({ "bool": {} })
        )
    }
}