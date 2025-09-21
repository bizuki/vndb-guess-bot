use vn::{SortVisualNovelBy, VisualNovel, Vndb};

use crate::client::VndbClient;

pub enum SortType {
    Bayesian,
    Votecount
}

impl Into<SortVisualNovelBy> for SortType {
    fn into(self) -> SortVisualNovelBy {
        match self {
            Self::Bayesian => SortVisualNovelBy::Rating,
            Self::Votecount => SortVisualNovelBy::VoteCount,
        }
    }
}

pub struct RandomVnQuery {
    sort_by: Option<SortType>,
    fields: Vec<vn::VisualNovelField>,
    limit: Option<u16>,
    user: Option<u8>,
    reverse: bool,
    filters: Option<serde_json::Value>,
}

pub trait RandomVnClient {
    async fn get_vn(&self, query: RandomVnQuery, position: u16) -> Result<VisualNovel, vn::error::Error>;
    async fn vn_count(&self, query: RandomVnQuery) -> Result<u32, vn::error::Error>;

    async fn random_visual_novels(&self, query: RandomVnQuery, count: i32) -> Result<Vec<VisualNovel, vn::error::Error>> {

    }
}

impl RandomVnClient for VndbClient {
    async fn get_vn(&self, query: RandomVnQuery, position: u16) -> Result<VisualNovel, vn::error::Error> {        
        let mut vn_query = self.client.post()
            .visual_novel()
            .page(position)
            .results(1);

        if let Some(sort) = query.sort_by {
            vn_query = vn_query.sort(sort.into())
        }

        if let Some(user) = query.user {
            vn_query = vn_query.user(user)
        }

        if query.reverse {
            vn_query = vn_query.reverse()
        }

        if let Some(filters) = query.filters {
            vn_query = vn_query.filters(filters.into())
        }

        Ok(
            vn_query
                .send()
                .await?
                .results
                .into_iter()
                .next()
                .unwrap() 
        )
    }
    
    async fn vn_count(&self, query: RandomVnQuery) -> Result<u32, vn::error::Error> {
        let mut vn_query = self.client.post()
            .visual_novel()
            .results(1)
            .count();

        self.client
            .random_release()

        if let Some(user) = query.user {
            vn_query = vn_query.user(user)
        }

        if let Some(filters) = query.filters {
            vn_query = vn_query.filters(filters.into())
        }

        vn_query
            .send()
            .await
            .map(|resp| resp.count.unwrap())
    }
}
