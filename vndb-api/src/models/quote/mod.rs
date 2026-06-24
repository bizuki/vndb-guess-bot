mod filter;
mod model;

pub use filter::QuoteFilters;
pub use model::*;

pub type QuoteQueryBuilder<'client, Client> = crate::client::EndpointQueryBuilder<
    'client,
    Client,
    Quote,
    QuoteFilters,
    QuoteFields,
    QuoteSort,
>;
pub type QuoteQuery = crate::query::VndbQuery<QuoteFilters, QuoteFields, QuoteSort>;
pub type QuoteResult = crate::query::VndbQueryResponse<Quote>;
