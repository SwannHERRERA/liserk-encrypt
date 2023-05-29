use serde::{Deserialize, Serialize};

/// Specifies the type of a `CompoundQuery`, defining how its `Query`s are combined.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum QueryType {
    And,
    Or,
}

/// The root Query type, which can be either a `SingleQuery` or a `CompoundQuery`.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Query {
    Single(SingleQuery),
    Compound(CompoundQuery),
}

/// Represents a single query on a collection for a given use case.
///
/// This is the basic unit of querying in this system.
/// A `SingleQuery` operates on one collection and one use case.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SingleQuery {
    pub collection: String,
    pub usecase: String,
}

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
    /// Returns a `SingleQuery`.
    pub fn new(collection: String, usecase: String) -> Self {
        SingleQuery { collection, usecase }
    }
}

/// Represents a compound query composed of multiple `Query`s.
///
/// A `CompoundQuery` allows for complex query logic by combining multiple `Query`s
/// using a specified `QueryType` (e.g., And, Or).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CompoundQuery {
    pub query_type: QueryType,
    pub queries: Vec<Query>,
}

/// Builder for `SingleQuery`
#[derive(Debug, Default)]
pub struct SingleQueryBuilder {
    collection: String,
    usecase: String,
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

    pub fn build(self) -> SingleQuery {
        SingleQuery { collection: self.collection, usecase: self.usecase }
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
