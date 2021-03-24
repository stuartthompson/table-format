# Table-Format

[<img alt="github" src="https://img.shields.io/badge/github-stuartthompson/table--format-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/stuartthompson/table-format)
[<img alt="crates.io" src="https://img.shields.io/crates/v/table-format.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/table-format)
[<img alt="last commit" src="https://img.shields.io/github/last-commit/stuartthompson/table-format?logo=GitHub&style=for-the-badge" height="20">](https://github.com/stuartthompson/table-format/commits/master)
[<img alt="ci status" src="https://img.shields.io/github/workflow/status/stuartthompson/table-format/CI?label=Build&logo=GitHub%20Actions&logoColor=%23ffffff&style=for-the-badge" height="20">](https://github.com/stuartthompson/table-format/actions/workflows/ci.yml)

This crate formats data as a text table, suitable for printing to the terminal 
or for inclusion in logs.

## Changelog

* v0.0.1 - Initial files. Tests are passing. Table header formats.
* v0.0.2 - Table macros. Base cell styles. Color codes for initial elements.
* v0.0.3 - ColumnBreak->CellWidth. Now a part of content style.
* v0.0.4 - Linting, bug fixes, test cleanup. More test coverage.

## Roadmap
* v0.0.5 - Examples and README improvement.
* v0.1.0 - Documentation cleanup pass. Get what is there clean and tidy.

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

## Content Style

The contents of a table cell can be styled using either a style directive or a 
manually built ContentStyle struct. A content style consists of the following 
elements:

* Alignment (left, center, right)
* Colors (foreground, background)
* Width (fixed, minimum-width, content)
* Wrap (wrap or truncate)

### Style Directives

A short-hand for specifying content styles is to use a style directive. These 
strings are formatted as follows:

```
{alignment[color]|width|wrap}
```

#### Alignment

Alignment is specified using one of: < ^ >
* <  *(default)* left aligned
* ^  center aligned
* >  right aligned

#### Color

Color is specified inside square brackets [] and consists of:

* A single color code, specifying only foreground color.
* Two color codes, specifying foreground and background color.
* A single color code following a -, specifying only background color.

Lower-case color codes indicating regular (dark) colors. Upper-case codes 
indicate bright colors.

Example color codes:
```
[c]   - Cyan on black  (foreground color specified only)
[rG]  - Dark red on bright breen
[Wb]  - Bright white on dark blue
[-g]  - White on dark green  (background color specified only)
```

#### Width

The width of a cell is used when describing headers. The cells within the 
table header row describe the width of those headers and are used to determine 
column breaks for the table, and where other content should be wrapped or 
truncated.

Width is specified between pipes ||.

There are three width rules that can be specified:

* Fixed width - The content will be wrapped or truncated to fit the width.
* Minimum width - Will always be minimum width, but otherwise will grow to fit.
* Content - The column is sized based upon the size of the content.

Example width specifiers:

```
|f15|  - Fixed width of 15 chars
|m10|  - Minimum width of 10 chars
|c|    - (default) Sized according to content
```

#### Wrap

When content is too big to fit into a cell it will either be wrapped to 
multiple lines or truncated. The wrap mode specifier specifies whether to wrap 
or truncate. This specifier is a single character at the end of the style 
directive.

Wrapping is indicated by including a semi-colon ; at the end of the 
directive. Truncation (default) requires no specifier.

Example of a style directive that will wrap content:
```
{<c;}  - Left-aligned, cyan, wrapped
```

#### All Fields are Optional

All of the four parts of the style directive are optional. The empty style 
directive {} simply means "left aligned, white on black, content width, 
truncate". It is not necessary to include an empty style directive.

Examples of partial directives:
```
{>;}    - Right-aligned, white on black, content width, wrapped
{[-g]}  - Left-aligned, white on dark green, content width, truncated
{;}     - Left-aligned, white on black, content width, wrapped
{|f15|} - Left-aligned, white on black, fixed 15 char width, truncated
```

### Examples

Example 1: Left-aligned, cyan on black, fixed 15-char width, truncate:
```
{<[c]|f15|}
```

Example 2: Center-aligned, bright yellow on dark green background, content 
width, wrap content:
```
{^[Yg]|c|;}
```
*Note: The |c| above is optional and can be omitted.*
