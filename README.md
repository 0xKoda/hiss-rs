# DNS History CLI

This is a command-line interface (CLI) tool that performs DNS history lookups. It retrieves passive DNS history data for domains, IP addresses, or nameservers.

## Features

- Lookup passive DNS history for domains, IP addresses, or nameservers
- Display results in a formatted table
- Show first seen and last seen dates for each record

## Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- Cargo (Rust's package manager, typically installed with Rust)

## Installation

1. Clone this repository:
`git clone https://github.com/yourusername/dns-lookup-cli.git`
`cd dns-lookup-cli`

2. Build the project:
`cargo build`

## Usage

Run the tool using Cargo:
`cargo run -- <query>`

Replace `<query>` with the domain, IP address, or nameserver you want to look up.

Example:

`cargo run -- example.com`

This will output a table with the following columns:
- First Seen: The date when the DNS record was first observed
- Last Seen: The date when the DNS record was last observed
- Query: The queried domain, IP, or nameserver
- Answer: The resolved IP address or other DNS record data
- Type: The DNS record type (e.g., A, CNAME, MX)

## Sample Output
+---------------------+---------------------+-------------+---------------+------+
| First Seen          | Last Seen           | Query       | Answer        | Type |
+---------------------+---------------------+-------------+---------------+------+
| 2023-03-15 12:34:56 | 2024-07-20 09:45:30 | example.com | 93.184.216.34 | A    |
| 2023-03-15 12:34:56 | 2024-07-20 09:45:30 | example.com |1893:25c8:1946 | AAAA |
+---------------------+---------------------+-------------+---------------+------+
