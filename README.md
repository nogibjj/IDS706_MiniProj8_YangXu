# IDS706_MiniProj8_YangXu

Rust version:[This Repositories](https://github.com/nogibjj/IDS706_MiniProj8_YangXu)<br>
[![Rust CI/CD Pipeline](https://github.com/nogibjj/IDS706_MiniProj8_YangXu/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/IDS706_MiniProj8_YangXu/actions/workflows/cicd.yml)


Python version:[IDS706_MiniProj7_YangXu](https://github.com/nogibjj/IDS706_MiniProj7_YangXu)<br>
[![Python CI](https://github.com/nogibjj/IDS706_MiniProj7_YangXu/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/IDS706_MiniProj7_YangXu/actions/workflows/cicd.yml)

## Project Overview

This project rewrites a previous Python script ([IDS706_MiniProj7_YangXu](https://github.com/nogibjj/IDS706_MiniProj7_YangXu)) into Rust, showcasing complex SQL queries using an external MySQL database. The goal is to demonstrate proficiency in constructing SQL queries in Rust and to highlight performance improvements over the Python counterpart. This Rust application also integrates Continuous Integration (CI) for automated testing.

## Table of Contents

- [Project Structure](#project-structure)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Running Tests](#running-tests)
- [Performance Comparison](#performance-comparison)
- [Acknowledgements](#acknowledgements)

## Project Structure

```bash
IDS706_MiniProj8_YangXu (Root Directory)
│
├── .devcontainer
│   ├── Dockerfile
│   └── devcontainer.json
│
├── .github
│   └── workflows
│       └── cicd.yml
│
├── mini_proj_8
│   ├── config.rs
│   ├── main.rs
│   ├── lib.rs
│   └── dataset_sample.csv
│
├── tests
│   └── test_main.rs
│   └── dataset_sample.csv
│
├── dataset_sample.csv
│
├── Cargo.toml
│
├── .env
│
├── env.example
│
├── requirements.txt
│
├── Makefile
│
├── README.md
│
└── .gitignore
```

## Requirements

The Rust project leverages several crates:

- csv: To handle CSV data.
- mysql_async: For asynchronous database operations.
- tokio: An asynchronous runtime.
- dotenv: To manage environment variables.
- futures: For future combinators.

## Installation

1. Clone this repository.
2. Create a .env file using env.example as a template. Fill in your database credentials.
3. Build the project using Cargo:
    ```bash
    cargo build --release
    ```

## Usage

After building, you can run the program using:
    ```bash
    cargo run --release
    ```

## Running Tests

To run the tests for the project, use:
    ```bash
    cargo test
    ```

## Complex Query Explanation

The complex SQL query used in this project performs the following operations:

1. Joins the main data table (`week6_mini`) with the discounts table (`week6_mini_discounts`) using the "Product" column.
2. Calculates the total revenue by multiplying price and quantity, considering any available discount.
3. Sorts the results by the calculated total revenue in descending order.

    ```sql
    SELECT w.Date, w.Product, w.Price, w.Quantity, d.Discount,
        (w.Price * w.Quantity) * (1 - d.Discount) AS Total_Revenue
    FROM week6_mini w
    LEFT JOIN week6_mini_discounts d ON w.Product = d.Product
    ORDER BY Total_Revenue DESC;
    ```

## Sample Output

- **Descriptive Statistics**:

    ```bash
    ("2023-09-03", "Apple", 1.1, 55, None, 60.500001311302185)
    ("2023-09-01", "Apple", 1.2, 50, None, 60.00000238418579)
    ("2023-09-02", "Apple", 1.3, 45, None, 58.49999785423279)
    ("2023-09-02", "Cherry", 2.4, 22, None, 52.800002098083496)
    ("2023-09-01", "Cherry", 2.5, 20, None, 50.0)
    ("2023-09-03", "Cherry", 2.6, 19, None, 49.3999981880188)
    ("2023-09-02", "Banana", 0.6, 50, None, 30.000001192092896)
    ("2023-09-03", "Banana", 0.7, 42, None, 29.399999499320984)
    ("2023-09-01", "Banana", 0.5, 40, None, 20.0)
    ```

## Performance Comparison

The performance improvements of the Rust project over the Python counterpart can be determined through several methods:

1. **Time Execution**: Measure the time it takes for both the Python and Rust projects to execute.
2. **Memory Usage**: Measure the RAM usage of both the Python and Rust projects during execution.

### Python version test result:
![Python Version Test Result](ver_py_testresult.png)

## Acknowledgements

This Rust project is a rewrite of the Python project, [IDS706_MiniProj7_YangXu](https://github.com/nogibjj/IDS706_MiniProj7_YangXu). The aim is to enhance speed and resource efficiency.

1. **Time Execution**: Can use the `time` command in Linux to measure the time it takes for both scripts to execute.

**For Python**:
   ```bash
   time python3 mini_proj_7/mian.py
   ```
![Python Version real execution time](ver_py_runtime.png)

**For Rust**:
   ```bash
   time cargo run --release
   ```
![Rust Version real execution time](ver_rs_runtime.png)

### Comparison of Execution Times:
**Python Version**: `Took approximately 2.129 seconds for execution.`
**Rust Version**: `Took approximately 1.071 seconds for execution.`
### Runtime Result:
`The Rust version executed almost twice as fast, showing a significant improvement in performance over the Python version.`
<br>

2. **Memory Usage**: Can use the memory-profiler package in Python and the heaptrack tool for Rust.

**For Python**:
   ```bash
   mprof run mini_proj_7/mian.py
   mprof plot
   ```

**For Rust**:
   ```bash
   heaptrack cargo run --release
   heaptrack_gui heaptrack.week8mini.12345.gz
   ```
This will give insights into the memory allocation of both scripts.


