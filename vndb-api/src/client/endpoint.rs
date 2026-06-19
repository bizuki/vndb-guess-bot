#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VndbEndpoint {
    Schema,
    Stats,
    User,
    AuthInfo,
    Vn,
    Release,
    Producer,
    Character,
    Staff,
    Tag,
    Trait,
    Quote,
}

impl VndbEndpoint {
    pub const fn path(self) -> &'static str {
        match self {
            Self::Schema => "schema",
            Self::Stats => "stats",
            Self::User => "user",
            Self::AuthInfo => "authinfo",
            Self::Vn => "vn",
            Self::Release => "release",
            Self::Producer => "producer",
            Self::Character => "character",
            Self::Staff => "staff",
            Self::Tag => "tag",
            Self::Trait => "trait",
            Self::Quote => "quote",
        }
    }
}
