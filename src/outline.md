```txt
ALGORITHM: Rusty Store Inventory Management System

DEFINE the following data structures:
    Product
        - name: String
        - description: String
        - price: f64
        - quantity: u32
    Sale
        - product: Product
        - quantity: u32
        - sale_price: f64
    Purchase
        - product: Product
        - quantity: u32
        - purchase_price: f64
    Inventory
        - products: HashMap<String, Product>
    Sales
        - transactions: Vec<Sale>
    Purchases
        - transactions: Vec<Purchase>

DEFINE the following functions:

FUNCTION add_product(inventory: &mut Inventory, product: Product)
    IF inventory.products.contains_key(&product.name)
        RETURN error("Product already exists")
    ELSE
        inventory.products.insert(product.name.clone(), product)
        RETURN success()

FUNCTION update_product(inventory: &mut Inventory, product: Product)
    IF inventory.products.contains_key(&product.name)
        inventory.products.insert(product.name.clone(), product)
        RETURN success()
    ELSE
        RETURN error("Product does not exist")

FUNCTION delete_product(inventory: &mut Inventory, product_name: &str)
    IF inventory.products.contains_key(product_name)
        inventory.products.remove(product_name)
        RETURN success()
    ELSE
        RETURN error("Product does not exist")

FUNCTION record_sale(sales: &mut Sales, product: &Product, quantity: u32, sale_price: f64)
    IF product.quantity < quantity
        RETURN error("Not enough stock")
    ELSE
        product.quantity -= quantity
        sales.transactions.push(Sale {
            product: product.clone(),
            quantity,
            sale_price
        })
        RETURN success()

FUNCTION record_purchase(purchases: &mut Purchases, product: &mut Product, quantity: u32, purchase_price: f64)
    product.quantity += quantity
    purchases.transactions.push(Purchase {
        product: product.clone(),
        quantity,
        purchase_price
    })
    RETURN success()

FUNCTION generate_inventory_report(inventory: &Inventory) -> String
    INITIALIZE report = ""
    FOR each product in inventory.products
        report += format!("{}: {} (${:.2}, {} in stock)\n", product.name, product.description, product.price, product.quantity)
    RETURN report

FUNCTION generate_sales_report(sales: &Sales) -> String
    INITIALIZE report = ""
    FOR each sale in sales.transactions
        report += format!("Sold {} x {} for ${:.2} (total: ${:.2})\n", sale.quantity, sale.product.name, sale.sale_price, sale.quantity * sale.sale_price)
    RETURN report

FUNCTION generate_purchases_report(purchases: &Purchases) -> String
    INITIALIZE report = ""
    FOR each purchase in purchases.transactions
        report += format!("Bought {} x {} for ${:.2} (total: ${:.2})\n", purchase.quantity, purchase.product.name, purchase.purchase_price, purchase.quantity * purchase.purchase_price)
    RETURN report

MAIN FUNCTION
    INITIALIZE inventory = Inventory { products: HashMap::new() }
    INITIALIZE sales = Sales { transactions: Vec::new() }
    INITIALIZE purchases = Purchases { transactions: Vec::new() }

    WHILE true
        DISPLAY menu options
        GET user input
        SWITCH on user input
            CASE "add product"
                GET product details from user
                add_product(&mut inventory, product)
            CASE "update product"
                GET product details from user
                update_product(&mut inventory, product)
            CASE "delete product"
                GET product name from user
                delete_product(&mut inventory, product_name)
            CASE "record sale"
                GET product, quantity, and sale price from user
                record_sale(&mut sales, product, quantity, sale_price)
            CASE "record purchase"
                GET product, quantity, and purchase price from user
                record_purchase(&mut purchases, product, quantity, purchase_price)
            CASE "generate inventory report"
                DISPLAY generate_inventory_report(&inventory)
            CASE "generate sales report"
                DISPLAY generate_sales_report(&sales)
            CASE "generate purchases report"
                DISPLAY generate_purchases_report(&purchases)
            CASE "exit"
                BREAK
            DEFAULT
                DISPLAY "Invalid option. Please try again."
```