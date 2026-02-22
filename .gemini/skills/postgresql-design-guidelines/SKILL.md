---
name: postgresql-design-guidelines
description: PostgreSQL design guidelines for schema management, naming conventions, data types, indexing, partitioning, and multi-tenancy. Use when designing DB tables, writing migrations, or auditing existing database structures to ensure adherence to professional engineering standards.
---

# PostgreSQL Design Guidelines

This skill provides expert guidance for PostgreSQL database design based on established best practices for OLTP systems.

## Key Principles

- **Standardization**: Adhere to consistent naming and data type policies to reduce cognitive load and maintenance costs.
- **Performance**: Use appropriate indexing (B-tree by default) and partitioning for large datasets.
- **Integrity**: Prioritize `NOT NULL` constraints and application-level validation over complex DB-level constraints (CHECK, DOMAIN, TRIGGER).

## Guidelines Reference

For detailed rules, always refer to [references/guidelines.md](references/guidelines.md).

### Naming Conventions

- **Tables**: Use prefixes to identify type:
  - `m_` (Master), `t_` (Transaction), `w_` (Work), `h_` (History).
  - Use singular nouns (e.g., `t_order` instead of `t_orders`).
- **Columns**: Use descriptive suffixes:
  - `_id` (Surrogate key), `_code` (Natural key/Business key), `_at` (Timestamp), `_date` (Date), `_typ` (Category/Classification).
- **Boolean**: Prefer `is_` or `has_` prefixes, but consider using timestamps (e.g., `verified_at`) to capture more context.

### Recommended Data Types

| Data Type | Usage | Notes |
| :--- | :--- | :--- |
| `bigint` | Surrogate IDs | Use `GENERATED ALWAYS AS IDENTITY`. |
| `varchar(n)` | Codes, Names | Avoid `char(n)` and `text` unless specific needs arise. |
| `timestamptz` | Timestamps | Always include time zone. |
| `numeric(p, s)`| Amounts, Rates | Avoid `float` and `double` for precision-critical data. |
| `boolean` | Flags | Use `NOT NULL DEFAULT false`. |

### Database Constraints

- **Primary Key**: Use surrogate keys (`bigint` identity) for transactions.
- **Unique**: Use `CREATE UNIQUE INDEX` instead of `UNIQUE` constraints for better maintenance.
- **Foreign Keys**: Design logically but avoid physical enforcement in highly dynamic or high-scale environments where maintenance overhead is a concern.
- **NOT NULL**: Apply strictly. Use "Unknown" codes for classification if necessary.

## Design Workflow

When asked to design a new table or modify an existing one:

1. **Classify**: Identify the table type (`m_`, `t_`, etc.).
2. **Define Columns**: Map business requirements to standard data types and naming conventions.
3. **Common Columns**: Always include audit columns:
   - `created_at`, `created_by`, `created_pgm_cd`, `created_tx_id`
   - `updated_at`, `updated_by`, `updated_pgm_cd`, `updated_tx_id`
   - `lock_no` (Optimistic locking)
4. **Index Design**: Identify primary access paths and create B-tree indexes.
5. **Validation**: Check against `references/guidelines.md` for specific prohibitions (e.g., no `TRIGGER`, no `DOMAIN`, no `serial`).

## Common Anti-Patterns to Avoid

- Using `char(n)` due to padding issues.
- Using `serial` instead of `IDENTITY` columns.
- Using `JSON/JSONB` for data that should be normalized.
- Defining business logic in `TRIGGER` or `FUNCTION`.
