# Table-Format

[<img alt="github" src="https://img.shields.io/badge/github-stuartthompson/table--format-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/stuartthompson/table-format)
[<img alt="crates.io" src="https://img.shields.io/crates/v/table-format.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/table-format)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/stuartthompson/table-format/CI/master?style=for-the-badge" height="20">](https://github.com/stuartthompson/table-format/actions?query=branch%3Amaster)

This crate formats data as a text table, suitable for printing to the terminal 
or for inclusion in logs.

## Changelog

v0.0.1 - Initial files. Tests are passing. Table header formats.
v0.0.2 - Table macros. Base cell styles. Color codes for initial elements.

## Roadmap
v0.0.3 - Color codes supported across all elements.
v0.0.5 - Format table data rows. Expand testing.
v0.1.0 - Documentation cleanup pass. Get what is there clean and tidy.

## Examples

The following code prints a two-column table from a vector of strings:

```
let table = 
    table!(
        "{B^:12:}" => "Food", "{G^:7:}" => "Count";
        "Fish", "15", "Pizza", "10", "Tomato", "24"
    );

let output = table.format();
```

Output:
*(color codes not shown)*
```
+--------------------+
|    Food    | Count |
+--------------------+
|Fish        |15     |
+--------------------+
|Pizza       |10     |
+--------------------+
|Tomato      |24     |
+--------------------+
```