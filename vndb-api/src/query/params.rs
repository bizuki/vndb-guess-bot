use crate::ids::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryParams {
    pub reverse: bool,
    pub results: usize,
    pub page: usize,
    pub user: Option<UserId>,
    pub count: bool,
    pub compact_filters: bool,
    pub normalized_filters: bool,
}

impl Default for QueryParams {
    fn default() -> Self {
        Self {
            reverse: false,
            results: 10,
            page: 1,
            user: None,
            count: false,
            compact_filters: false,
            normalized_filters: false,
        }
    }
}

impl QueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    pub fn with_results(mut self, results: usize) -> Self {
        self.results = results;
        self
    }

    pub fn with_page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    pub fn with_user(mut self, user: UserId) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_count(mut self, count: bool) -> Self {
        self.count = count;
        self
    }

    pub fn with_compact_filters(mut self, compact_filters: bool) -> Self {
        self.compact_filters = compact_filters;
        self
    }

    pub fn with_normalized_filters(mut self, normalized_filters: bool) -> Self {
        self.normalized_filters = normalized_filters;
        self
    }
}
