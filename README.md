# jlogs

Simple tool for summarizing logs.
Counting bytes and occurences of json logs tagged with "type" 
Supports single json object per line, in file.

It doesn't support other log inside a log, e.g.:
`{"type": "A", "value": { "type": "B", "value": 5}}` will still count as one occurence of log type A, without processing the log of type B


## How to run:

`cargo run --release -- json.file`

## Example Output:

```
Log Type  Count  Total Bytes
G         1      137
J         1      143
X         1      266
T         1      269
B         1      287
I         3      419
U         1      289
Z         1      224
M         1      260
```

## Error handling:

Whenever program processes the log that is not a valid json object, or doesn't have a type field, it logs into stderr and continues processing.
