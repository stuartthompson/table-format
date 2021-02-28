# Table-Format

This crate formats data as a text table, suitable for printing to the terminal 
or for inclusion in logs.

## Changelog

v0.0.1 - Initial files. Tests are passing. Table header formats.

## Roadmap

v0.0.2 - Format table data rows. Expand testing.
v0.0.3 - Documentation cleanup pass. Get what is there clean and tidy.

## Examples

The following code prints a two-column table from a vector of strings:

```
let mut data = VecDataSource::from(
    vec!("Fish", "3", "Apples", "5", "Pizza", "13"));

let columns = vec!(
    TableColumn::fixed("Food".to_string(), 15),
    TableColumn::fixed("Count".to_string(), 15),
);
let table = Table::from(&mut data, columns);

let output = table.format(80);
```

Output:

```
+-------------------------------+
|Food           |Count          |
+-------------------------------+
```