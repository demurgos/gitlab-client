# Next

- **[Breaking change]** Require `UserAgent` to be specified when using the `http` client.
- **[Breaking change]** Add `last` field to `Page`
- **[Breaking change]** Use `CompactString` instead of `String` as the default type for `AsRef<str>` generic type
  parameters
- **[Feature]** Add `GetTreeRecordListQuery` support

# 0.14.2 (2024-04-26)

- **[Fix]** Fix `ProjectPermissions::group_access` type (it is an object, not a string).

# 0.14.1 (2024-04-24)

- **[Fix]** Fix `Project::shared_with_groups` type.
- **[Fix]** Fix `ProjectPermissions::project_access` type (it can be `null`).
- **[Fix]** Fix `Project::marked_for_deletion_at` and `Project::marked_for_deletion_on`.
- **[Fix]** Fix `ProjectLinks` type (fields may be missing).
- **[Fix]** Fix `AccessLevel` serialization.

# 0.14.0 (2024-04-24)

- **[Breaking change]** Implement handlers as tower services.
- **[Feature]** Start of CHANGELOG
