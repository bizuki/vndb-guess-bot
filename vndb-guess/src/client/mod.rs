use std::sync::Arc;

use vn::Vndb;

pub struct VndbClient {
    pub(crate) client: Arc<Vndb>
}
