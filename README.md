# Table-Format

[<img alt="github" src="https://img.shields.io/badge/github-stuartthompson/table--format-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/stuartthompson/table-format)
[<img alt="crates.io" src="https://img.shields.io/crates/v/table-format.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/table-format)
[<img alt="last commit" src="https://img.shields.io/github/last-commit/stuartthompson/table-format?logo=GitHub&style=for-the-badge" height="20">](https://github.com/stuartthompson/table-format/commits/master)
[<img alt="ci status" src="https://img.shields.io/github/workflow/status/stuartthompson/table-format/CI?label=Build&logo=GitHub%20Actions&logoColor=%23ffffff&style=for-the-badge" height="20">](https://github.com/stuartthompson/table-format/actions/workflows/ci.yml)

Formats tables for printing to the terminal or inclusion in logs. Offers
concise syntax for simple use cases and advanced options to address more
complex needs.

## Documentation

See the complete documentation here:
[Table-Format Documentation]()


#### What does it do?

Table-format is a library written in the Rust programming language that helps
with formatting data into text tables. It offers control over the styling of
header and body elements independently and includes some more advanced use
cases including row headers (horizontally-placed headers), partial row support
(where the data source does not contain enough values for a complete final
row), and per-cell styling options, including multi-line, per-line formatting
for more complex visualization use cases.

#### Why this library vs. others?

I built this library to solve a specific formatting problem that occurred
regularly enough in my programming journey to warrant a specific solution; a
way to print and visualize data structures at the terminal. There are many
other really good table formatters out there. None of them met my specific
needs and so I built my own. I hope that it may be useful to others.

## Simple Tables

Create a simple table using the table! macro, describing the header columns,
and providing an array of data or a vector for the body.

```
println!("{}",
    table!(
        "{^:10:}" => "Food", "{^:10:}" => "Count";
        "Fish", "15", "Pizza", "10", "Steak", "6"
    ).format()
);
```

```
+---------------------+
|   Food   |  Count   |
+---------------------+
|Fish      |15        |
+---------------------+
|Pizza     |10        |
+---------------------+
|Steak     |6         |
+---------------------+
```

## More Advanced Use Cases

#### Colors

[include images with color]

#### Alignment

[table showing multiple different alignments]

#### Wrapping

[wrapped text vs. truncated text]

#### Multi-Line Cells

[table with multi-line text cells]

#### Per-Line Formatting

[multi-line formatting example]

#### Iterator Data Sources

[code showing use of an iterator data source]

Create highly customized table output using the advanced formatting options.

## Changelog & Roadmap

See the
[CHANGELOG](https://github.com/stuartthompson/table-format/CHANGELOG.md) and
[ROADMAP](https://github.com/stuartthompson/table-format/ROADMAP.md) for
details.

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
format strings are specified as follows:

```
{alignment|color|width|wrap}
```

#### Alignment

Alignment is specified using one of: < ^ >
```
* <   - *(default)* left aligned
* ^   - center aligned
* \>  - right aligned
```

#### Color

Color is specified inside square brackets [] and consists of:

* A single color code, specifying only foreground color.
* Two color codes, specifying foreground and background color.
* A single color code following a -, specifying only background color.

Lower-case color codes indicating regular (dark) colors. Upper-case codes 
indicate bright colors.

Example color codes:
```
c   - Cyan on black  (foreground color specified only)
rG  - Dark red on bright breen
Wb  - Bright white on dark blue
-g  - White on dark green  (background color specified only)
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
:15:  - Fixed width of 15 chars
|10|  - Minimum width of 10 chars
@     - (default) Sized according to content
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
{-g}  - Left-aligned, white on dark green, content width, truncated
{;}     - Left-aligned, white on black, content width, wrapped
{:15:}  - Left-aligned, white on black, fixed 15 char width, truncated
```

### Examples

Example 1: Left-aligned, cyan on black, fixed 15-char width, truncate:
```
{<[c]:15:}
```

Example 2: Center-aligned, bright yellow on dark green background, content 
width, wrap content:
```
{^[Yg]@;}
```
*Note: The @ above is optional and can be omitted.*
