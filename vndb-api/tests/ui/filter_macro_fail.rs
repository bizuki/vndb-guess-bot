use vndb_api_derive::VndbFiltersEnum;

#[derive(Debug, Clone, VndbFiltersEnum)]
enum TestCharacterFilters {
    #[vndb_filter(value = vndb_api::filter::StringValue)]
    Name,
    #[vndb_filter(value = vndb_api::filter::IntegerValue, ordered)]
    Age,
    #[vndb_filter(value = vndb_api::filter::Birthday, nullable)]
    Birthday,
}

#[derive(Debug, Clone, VndbFiltersEnum)]
enum TestReleaseFilters {
    #[vndb_filter(value = vndb_api::filter::Resolution, ordered)]
    Resolution,
}

#[derive(Debug, Clone, VndbFiltersEnum)]
enum TestVnFilters {
    #[vndb_filter(value = vndb_api::filter::TagFilterValue)]
    Tag,
}

fn main() {
    use vndb_api::models::producer::ProducerFilters;

    let _ = TestCharacterFilters!(name = "x");
    let _ = TestCharacterFilters!("name");
    let _ = TestCharacterFilters!(missing);
    let _ = ProducerFilters!(description);
    let _ = TestCharacterFilters!(age).eq("old");
    let _ = TestCharacterFilters!(name).gt("x");
    let _ = TestCharacterFilters!(name).is_null();
    let _ = TestCharacterFilters!(birthday).eq([1, 23]);
    let _ = TestReleaseFilters!(resolution).eq([1920, 1080]);
    let _ = TestVnFilters!(tag).eq(serde_json::json!("g505"));
}
