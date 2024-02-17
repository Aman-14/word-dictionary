# Word Dictionary with Indexes

This is a proof-of-concept implementation demonstrating how indexes can significantly improve read performance, using a word dictionary as an example.
The implementation exposes an HTTP server that allows users to retrieve the definition of any word. The storage consists of a file with a specific format, which contains all definitions and indexes.

## File Format

The file is divided into three parts:

### Header (70 bytes)

The header contains the following information:

- Version (version)
- Indexes size (index_size)
- Exact byte position of indexes (index_posi)
- Number of bytes to read in one read call to get definition, its basically the size of the biggest definition (read_size)

### Definitions

This section contains word definitions, each separated by a newline character.

### Indexes

The indexes section is a set of key-value pair where the key is the word and the value is the exact index position of the definition of that word.

## Server

The server exposes an endpoint at `/definition` that allows users to fetch the definition of any word. The definition is returned as plain text.
