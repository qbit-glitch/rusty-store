use serde::Deserialize;
use serde_json::Value;
use prettytable::{Table, Row, Cell};

#[derive(Deserialize)]
struct Employee {
    id: i32,
    name: String,
    department: String,
}

#[derive(Deserialize)]
struct Product {
    id: i32,
    name: String,
    price: f64,
}

#[derive(Deserialize)]
struct Order {
    id: i32,
    employee_id: i32,
    product_id: i32,
}

fn main() {
    let json_data = r#"
    {
        "employees": [
            {"id": 1, "name": "Alice", "department": "Engineering"},
            {"id": 2, "name": "Bob", "department": "Marketing"}
        ],
        "products": [
            {"id": 101, "name": "Widget", "price": 19.99},
            {"id": 102, "name": "Gadget", "price": 29.99}
        ],
        "orders": [
            {"id": 1001, "employee_id": 1, "product_id": 101},
            {"id": 1002, "employee_id": 2, "product_id": 102}
        ]
    }
    "#;

    // Parse the JSON data
    let parsed: Value = serde_json::from_str(json_data).expect("Failed to parse JSON");

    // Print each table
    print_employees(&parsed["employees"]);
    print_products(&parsed["products"]);
    print_orders(&parsed["orders"]);
}

fn print_employees(array: &Value) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Employees")]));

    // Add headers
    table.add_row(Row::new(vec![Cell::new("ID"), Cell::new("Name"), Cell::new("Department")]));

    // Add rows
    for item in array.as_array().unwrap() {
        let emp: Employee = serde_json::from_value(item.clone()).unwrap();
        table.add_row(Row::new(vec![
            Cell::new(&emp.id.to_string()),
            Cell::new(&emp.name),
            Cell::new(&emp.department),
        ]));
    }

    table.printstd();
}

fn print_products(array: &Value) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Products")]));

    // Add headers
    table.add_row(Row::new(vec![Cell::new("ID"), Cell::new("Name"), Cell::new("Price")]));

    // Add rows
    for item in array.as_array().unwrap() {
        let prod: Product = serde_json::from_value(item.clone()).unwrap();
        table.add_row(Row::new(vec![
            Cell::new(&prod.id.to_string()),
            Cell::new(&prod.name),
            Cell::new(&prod.price.to_string()),
        ]));
    }

    table.printstd();
}

fn print_orders(array: &Value) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Orders")]));

    // Add headers
    table.add_row(Row::new(vec![Cell::new("ID"), Cell::new("Employee ID"), Cell::new("Product ID")]));

    // Add rows
    for item in array.as_array().unwrap() {
        let order: Order = serde_json::from_value(item.clone()).unwrap();
        table.add_row(Row::new(vec![
            Cell::new(&order.id.to_string()),
            Cell::new(&order.employee_id.to_string()),
            Cell::new(&order.product_id.to_string()),
        ]));
    }

    table.printstd();
}
