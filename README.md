# csv, excel toolkit written in Rust

**rsv** is a command line tool to deal with small and big CSV, TXT, EXCEL files (especially >10G). **rsv** has following features:

- written in Rust
- fast and parallel data processing (based on Rayon)
- real-time progress bar
- simple usage
- support command pipelines

## Usage

download **rsv.exe** from release tab, and append the file directory to system path.

## Available commands

- **head** - Show head n lines of CSV, TXT or EXCEL file.
- **header** - Show file headers.
- **count** - Count the number of lines of file :running:.
- **estimate** - Fast estimate the number of lines.
- **clean** - Clean file with escape char (e.g. ") or other strings :running:.
- **frequency** - Show frequency table for column(s) :running: :star:.
- **split** - Split file into separate files sequentially or based on column value :running: :star:.
- **select** - Select rows and columns by filter :running:.
- **flatten** - Prints flattened records to view records one by one.
- **slice** - Prints a slice of rows from file.
- **search** - Search with regexes :running: :star:.
- **stats** - Statistics for column(s), including min, max, mean, unique, null :running: :star:.
- **excel2csv** - Convert excel to csv.
- **table** - Format data as an aligned table.

Tips 1:

- :running: means the command is supported with a real-time progress bar.
- :star: means the command is supported with parallel data processing.

Tips 2:

All commands, except "clean" and "excel2csv", are allowed to be chained.

Tips 3:

You can always check usage of each command by **rsv command --help** or **rsv command -h**,
for example, rsv frequency --help.

## Basic Usage

- **rsv head**

```shell
rsv head data.csv                   # print as the file is
rsv head --tabled data.csv          # tabled
rsv head -t data.csv                # tabled too
rsv head -s \\t -t data.csv         # CSV file with a tab separator
rsv head data.xlsx                  # EXCEL file
rsv head --help                     # help info on all flags
```

- **rsv header**

```shell
rsv headers data.csv                # separator "," (default)
rsv headers -s \t data.csv          # separator tab
rsv headers data.xlsx               # EXCEL file
rsv headers --help                  # help info on all flags
```

- **rsv count**

```shell
rsv count data.csv                  # plain-text file
rsv count data.xlsx                 # EXCEL file
rsv count --no-header data.csv
rsv count --help                    # help info on all flags
```

- **rsv estimate**

```shell
rsv estimate data.csv
rsv estimate data.xlsx
rsv estimate --help                 # help info on all flags
```

- **rsv clean**

```shell
rsv clean data.csv                               # default to clean escape char "
rsv clean -e \"content-to-delete\" data.csv      # escape is a str, clean str to empty
rsv clean -o new-file.csv data.csv               # save to new-file.csv, the default is data-cleaned.csv
rsv clean --help                                 # help info on all flags
```

- **rsv frequency**

```shell
rsv frequency -c 0 data.csv              # default to the first column, descending order
rsv frequency -c 0 data.xlsx             # EXCEL file
rsv frequency -c 0,1,2,5 data.csv        # columns 0, 1, 2, and 5
rsv frequency -c 0-2,5 data.csv          # same as above
rsv frequency -c 0-2 --export data.csv   # export result to data-frequency.csv
rsv frequency -n 10 data.csv             # keep top 10 frequent items
rsv frequency -a 10 data.csv             # in ascending order
rsv frequency --help                     # help info on all flags

column selection syntax:
-c 0,1,2,5   -->    cols [0,1,2,5]
-c 0-2,5     -->    same as cols [0,1,2,5]
```

- **rsv split**

```shell
rsv split data.csv                # default to first column and field separator of ,
rsv split data.xlsx               # EXCEL file
rsv split -s \\t data.csv         # tab separator
rsv split -c 1 data.csv           # split based on second column
rsv split -c 0 -s \\t data.csv    # first column, \t separator
rsv split --size 1000 data.xlsx   # Sequential split, 1000 records in a file.
rsv split --help                  # help info on all flags
```

- **rsv select**

```shell
rsv select -f 0=a,b,c data.csv               # first column has values of a, b, or c
rsv select -f 0=a,b,c data.xlsx              # EXCEL file, sheet can be specified with the --sheet flag
rsv select -f "0=a,b&1=c" data.csv           # first column is a or b, AND the second column equals c
rsv select -f "0=a,b&1=c" --export data.csv  # export result
rsv select -s \\t -f 0=a,b data.csv          # tab separator
rsv select -c 0-4 -f 0=a,b data.csv          # select column
rsv select --help                            # help info on other options

filter syntax:
0=a,b,c         -->  the first column has values of a, b, or c
0=a,b&1=c       -->  the first column is a or b, **AND** the second column equals c

NOTE: 1. filters are all treated as strings.
      2. only & (AND) operation is supported, | (OR) operation is not supported.
      3. The filter option can be omitted to select all rows.

column selection syntax:
-c 0,1,2,5   -->    cols [0,1,2,5]
-c 0-2,5     -->    same as cols [0,1,2,5]
```

- **rsv flatten**

```shell
rsv flatten data.csv                       # default to show first 5 records
rsv flatten -n 50 data.csv                 # show 50 records
rsv flatten data.xls                       # EXCEL file
rsv flatten --delimiter \"--\" data.csv    # change line delimiter to anything
rsv flatten --help                         # help info on all flags
```

- **rsv slice**

```shell
rsv slice -s 100 -e 150 data.csv           # set start and end index
rsv slice -s 100 -l 50 data.csv            # set start index and the length
rsv slice -s 100 -l 50 data.xlsx           # EXCEL FILE
rsv slice -s 100 -l 50 --export data.csv   # export to data-slice.csv
rsv slice -e 10 --export data.csv          # set end index and export data
rsv slice -i 9 data.csv                    # the 10th line sliced only
rsv slice --help                           # help info on all flags
```

- **rsv search**

```shell
rsv search PATTERN data.csv                # search PATTERN
rsv search "^\d{4}-\d{2}-\d{2}$" data.csv  # search dates
rsv search --export PATTERN data.csv       # export result
rsv search PATTERN data.xlsx               # search EXCEL file
rsv slice --help                           # help info on all flags
```

- **rsv stats**

```shell
rsv stats data.csv                       # all columns, statistics include: min, max, mean, unique, null
rsv stats data.xlsx                      # EXCEL FILE
rsv stats -c 0,1 data.csv                # first two columns
rsv stats -c 0,1 --export data.csv       # export to data-stats.csv
rsv slice --help                         # help info on all flags
```

- **rsv excel2csv**

```shell
rsv excel2csv data.xlsx                 # apply to xlsx file, default to first sheet (or sheet1)
rsv excel2csv data.xls                  # apply also to xls file
rsv excel2csv --sheet 1 data.xls        # second sheet, e.g., sheet 2
rsv excel2csv -S 1 data.xls             # same as above
```

- **rsv table**

```shell
rsv head data.csv | rsv table                   # convert result to an aligned table
rsv slice -s 10 -e 15 data.csv | rsv table      # convert result to an aligned table
```

## Command pipeline

- **two commands pipelined**

```shell
rsv search "^\d{4}-\d{2}-\d{2}$" data.csv | rsv table     # search date and print in an aligned table
rsv select -f 0=a,b data.csv | rsv frequency -c 0         # filter rows and get its frequency table
rsv select -f "0=a,b&2=c" data.csv | rsv head -n 5        # filter rows, and show head 5 records
rsv select -f "0=a,b&2=c" -c 0-4 data.csv | rsv stats     # filter rows, select columns and make statistics
```

- **more commands pipelined**

```shell
rsv search pattern1 data.csv | rsv search pattern2 | rsv count    # two searches and count
rsv select -f 0=a,b data.csv | rsv search pattern | rsv stats     # select, search, and make statistics
rsv select -f 0=a,b data.csv | rsv search pattern | rsv table     # select, search, and print in a table
```

## Bug report and suggestion

[219352261 QQ Chat Room](https://jq.qq.com/?_wv=1027&k=MyU6ynI1)

## Next

new features will be added in the future.
