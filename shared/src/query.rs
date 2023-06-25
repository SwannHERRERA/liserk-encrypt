use serde::{Deserialize, Serialize};

/// Specifies the type of a `CompoundQuery`, defining how its `Query`s are combined.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum QueryType {
    And,
    Or,
}

/// The root Query type, which can be either a `SingleQuery` or a `CompoundQuery`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Query {
    Single(SingleQuery),
    Compound(CompoundQuery),
    GetById { id: String, collection: String },
    GetByIds { ids: Vec<String>, collection: String },
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single(l0), Self::Single(r0)) => l0 == r0,
            (Self::Compound(l0), Self::Compound(r0)) => l0 == r0,
            (
                Self::GetById { id: l_id, collection: l_collection },
                Self::GetById { id: r_id, collection: r_collection },
            ) => l_id == r_id && l_collection == r_collection,
            (
                Self::GetByIds { ids: l_ids, collection: l_collection },
                Self::GetByIds { ids: r_ids, collection: r_collection },
            ) => l_ids == r_ids && l_collection == r_collection,
            _ => false,
        }
    }
}

impl Eq for Query {}

/// Represents a single query on a collection for a given use case.
///
/// This is the basic unit of querying in this system.
/// A `SingleQuery` operates on one collection and one use case.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SingleQuery {
    pub collection: String,
    pub usecase: String,
    pub upper_limit: Option<f64>,
    pub lower_limit: Option<f64>,
}

impl PartialEq for SingleQuery {
    fn eq(&self, other: &Self) -> bool {
        self.collection == other.collection
            && self.usecase == other.usecase
            && self.upper_limit == other.upper_limit
            && self.lower_limit == other.lower_limit
    }
}

impl Eq for SingleQuery {}

impl SingleQuery {
    /// Constructs a new `SingleQuery`.
    ///
    /// # Arguments
    ///
    /// * `collection` - A string specifying the collection to be queried.
    /// * `usecase` - A string specifying the use case for the query.
    ///
    /// # Returns
    ///
    /// Returns a `SingleQuery` without OPE.
    pub fn new(collection: String, usecase: String) -> Self {
        SingleQuery {
            collection,
            usecase,
            upper_limit: None,
            lower_limit: None,
        }
    }
}

/// Represents a compound query composed of multiple `Query`s.
///
/// A `CompoundQuery` allows for complex query logic by combining multiple `Query`s
/// using a specified `QueryType` (e.g., And, Or).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompoundQuery {
    pub query_type: QueryType,
    pub queries: Vec<Query>,
}

impl PartialEq for CompoundQuery {
    fn eq(&self, other: &Self) -> bool {
        self.query_type == other.query_type && self.queries == other.queries
    }
}

impl Eq for CompoundQuery {}

/// Builder for `SingleQuery`
#[derive(Debug, Default)]
pub struct SingleQueryBuilder {
    collection: String,
    usecase: String,
    upper_limit: Option<f64>,
    lower_limit: Option<f64>,
}

impl SingleQueryBuilder {
    pub fn with_collection(mut self, collection: String) -> Self {
        self.collection = collection;
        self
    }

    pub fn with_usecase(mut self, usecase: String) -> Self {
        self.usecase = usecase;
        self
    }

    pub fn with_encrypted_field_less_than(mut self, value: f64) -> Self {
        self.upper_limit = Some(value);
        self
    }

    pub fn with_encrypted_field_higher_than(mut self, value: f64) -> Self {
        self.lower_limit = Some(value);
        self
    }

    pub fn build(self) -> SingleQuery {
        SingleQuery {
            collection: self.collection,
            usecase: self.usecase,
            upper_limit: self.upper_limit,
            lower_limit: self.lower_limit,
        }
    }
}

impl CompoundQuery {
    /// Constructs a new `CompoundQuery`.
    ///
    /// # Arguments
    ///
    /// * `query_type` - A `QueryType` specifying how to combine the `queries`.
    /// * `queries` - A Vec of `Query`s to be combined.
    ///
    /// # Returns
    ///
    /// Returns a `CompoundQuery`.
    pub fn new(query_type: QueryType, queries: Vec<Query>) -> Self {
        CompoundQuery { query_type, queries }
    }
}

/// Builder for `CompoundQuery`
#[derive(Debug)]
pub struct CompoundQueryBuilder {
    query_type: QueryType,
    queries: Vec<Query>,
}

impl Default for CompoundQueryBuilder {
    fn default() -> Self {
        Self {
            query_type: QueryType::And,
            queries: Default::default(),
        }
    }
}

impl CompoundQueryBuilder {
    pub fn with_query_type(mut self, query_type: QueryType) -> Self {
        self.query_type = query_type;
        self
    }

    pub fn with_query(mut self, query: Query) -> Self {
        self.queries.push(query);
        self
    }

    pub fn build(self) -> CompoundQuery {
        CompoundQuery { query_type: self.query_type, queries: self.queries }
    }
}
