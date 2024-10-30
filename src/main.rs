use std::collections::HashMap;
use std::io::{self, Write};

// Defining the Structures
#[derive(Debug, Clone)]
struct Product {
    name: String,
    description: String,
    quantity: i32,
    price: f64,
}

#[derive(Debug)]
struct Sale {
    product: Product,
    quantity: i32,
    sale_price: f64,
}

#[derive(Debug)]
struct Purchase {
    product: Product,
    quantity: i32,
    purchase_price: f64,
}

#[derive(Debug)]
struct Inventory {
    products: HashMap<String, Product>,
}

#[derive(Debug)]
struct RecordSales {
    transactions: Vec<Sale>,
}

#[derive(Debug)]
struct RecordPurchases {
    transactions: Vec<Purchase>,
}

fn main() {
    let mut inventory = Inventory { products: HashMap::new() };
    let mut sales = RecordSales { transactions: Vec::new() };
    let mut purchases = RecordPurchases { transactions: Vec::new() };

    let menu = format!(
        "\nInventory Management System\n\
        Choose an option:\n\
        1. \"add\" - Add Product\n\
        2. \"edit\" - Edit Product\n\
        3. \"delete\" - Delete Product\n\
        4. \"record-sale\" - Record Sale\n\
        5. \"record-purchase\" - Record Purchase\n\
        6. \"inventory-report\" - Generate Inventory Report\n\
        7. \"sales-report\" - Generate Sales Report\n\
        8. \"purchase-report\" - Generate Purchase Report\n\
        9. \"exit\" - Exit Program\n\
        Enter your choice: "
    );

    loop {
        print!("{}", menu);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_lowercase();

        match input.as_str() {
            "add" => {
                match get_product_details_from_user() {
                    Ok(product) => {
                        match add_product(&mut inventory, product) {
                            Ok(_) => println!("Product added successfully!"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "edit" => {
                match get_product_details_from_user() {
                    Ok(product) => {
                        match update_product(&mut inventory, product) {
                            Ok(_) => println!("Product updated successfully!"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "delete" => {
                print!("Enter product name to delete: ");
                io::stdout().flush().unwrap();
                let product_name = get_string_input();
                match delete_product(&mut inventory, &product_name) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "record-sale" => {
                if let Some(report) = record_sale_from_user(&mut inventory, &mut sales) {
                    println!("{}", report);
                }
            }
            "record-purchase" => {
                if let Some(report) = record_purchase_from_user(&mut inventory, &mut purchases) {
                    println!("{}", report);
                }
            }
            "inventory-report" => {
                if let Some(report) = generate_inventory_report(&inventory) {
                    println!("\nInventory Report:\n{}", report);
                }
            }
            "sales-report" => {
                if let Some(report) = generate_sales_report(&sales) {
                    println!("\nSales Report:\n{}", report);
                }
            }
            "purchase-report" => {
                if let Some(report) = generate_purchase_report(&purchases) {
                    println!("\nPurchase Report:\n{}", report);
                }
            }
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please try again."),
        }
    }
}

fn get_string_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_number_input<T>() -> Result<T, String>
where
    T: std::str::FromStr,
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
        .trim()
        .parse()
        .map_err(|_| "Invalid number input".to_string())
}

fn get_product_details_from_user() -> Result<Product, String> {
    println!("\nEnter Product Details:");
    
    print!("Product Name: ");
    io::stdout().flush().unwrap();
    let name = get_string_input();

    print!("Description: ");
    io::stdout().flush().unwrap();
    let description = get_string_input();

    print!("Quantity: ");
    io::stdout().flush().unwrap();
    let quantity: i32 = get_number_input()?;

    print!("Price: ");
    io::stdout().flush().unwrap();
    let price: f64 = get_number_input()?;

    Ok(Product {
        name,
        description,
        quantity,
        price,
    })
}

fn add_product(inventory: &mut Inventory, product: Product) -> Result<(), String> {
    if inventory.products.contains_key(&product.name) {
        Err(format!("Product '{}' already exists in inventory.", product.name))
    } else {
        inventory.products.insert(product.name.clone(), product);
        Ok(())
    }
}

fn update_product(inventory: &mut Inventory, product: Product) -> Result<(), String> {
    if inventory.products.contains_key(&product.name) {
        inventory.products.insert(product.name.clone(), product);
        Ok(())
    } else {
        Err(format!("Product '{}' not found in inventory", product.name))
    }
}

fn delete_product(inventory: &mut Inventory, product_name: &str) -> Result<String, String> {
    if inventory.products.remove(product_name).is_some() {
        Ok(format!("Product '{}' removed successfully", product_name))
    } else {
        Err(format!("Product '{}' not found in inventory", product_name))
    }
}

fn record_sale_from_user(inventory: &mut Inventory, sales: &mut RecordSales) -> Option<String> {
    print!("Enter product name: ");
    io::stdout().flush().unwrap();
    let product_name = get_string_input();

    if let Some(product) = inventory.products.get_mut(&product_name) {
        print!("Enter quantity to sell: ");
        io::stdout().flush().unwrap();
        if let Ok(quantity) = get_number_input::<i32>() {
            print!("Enter sale price: ");
            io::stdout().flush().unwrap();
            if let Ok(sale_price) = get_number_input::<f64>() {
                if product.quantity >= quantity {
                    product.quantity -= quantity;
                    sales.transactions.push(Sale {
                        product: product.clone(),
                        quantity,
                        sale_price,
                    });
                    return Some(format!("Sale recorded successfully for '{}'", product_name));
                } else {
                    println!("Error: Insufficient quantity in stock");
                }
            }
        }
    } else {
        println!("Error: Product not found");
    }
    None
}

fn record_purchase_from_user(inventory: &mut Inventory, purchases: &mut RecordPurchases) -> Option<String> {
    print!("Enter product name: ");
    io::stdout().flush().unwrap();
    let product_name = get_string_input();

    if let Some(product) = inventory.products.get_mut(&product_name) {
        print!("Enter quantity purchased: ");
        io::stdout().flush().unwrap();
        if let Ok(quantity) = get_number_input::<i32>() {
            print!("Enter purchase price: ");
            io::stdout().flush().unwrap();
            if let Ok(purchase_price) = get_number_input::<f64>() {
                product.quantity += quantity;
                purchases.transactions.push(Purchase {
                    product: product.clone(),
                    quantity,
                    purchase_price,
                });
                return Some(format!("Purchase recorded successfully for '{}'", product_name));
            }
        }
    } else {
        println!("Error: Product not found");
    }
    None
}

fn generate_inventory_report(inventory: &Inventory) -> Option<String> {
    let mut report = String::new();
    for (_, product) in &inventory.products {
        report.push_str(&format!(
            "{}\t{} - [${:.2}\t{}] in Stock\n",
            product.name, product.description, product.price, product.quantity
        ));
    }
    Some(report)
}

fn generate_sales_report(sales: &RecordSales) -> Option<String> {
    let mut report = String::new();
    for sale in &sales.transactions {
        report.push_str(&format!(
            "Sold {} * {} for ${:.2} each [Total - ${:.2}]\n",
            sale.quantity,
            sale.product.name,
            sale.sale_price,
            sale.sale_price * sale.quantity as f64
        ));
    }
    Some(report)
}

fn generate_purchase_report(purchases: &RecordPurchases) -> Option<String> {
    let mut report = String::new();
    for purchase in &purchases.transactions {
        report.push_str(&format!(
            "Purchased {} * {} for ${:.2} each [Total - ${:.2}]\n",
            purchase.quantity,
            purchase.product.name,
            purchase.purchase_price,
            purchase.purchase_price * purchase.quantity as f64
        ));
    }
    Some(report)
}