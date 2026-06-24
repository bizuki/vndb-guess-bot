# vndb-api workspace

This repository contains the VNDB API crates:

- `vndb-api`: typed models, query builders, clients, filters, fields, and sorts.
- `vndb-api-derive`: derive macros used to generate field, filter, and sort enums.
- `vndb-api-macros-support`: runtime support types used by generated code.

Downstream projects can reference the API crate directly from git:

```toml
vndb-api = { git = "https://github.com/<owner>/<repo>.git", package = "vndb-api" }
```

See [`vndb-api/README.md`](vndb-api/README.md) for usage examples and feature
flags.
