# Rusty Store - Inventory Management System

A robust command-line inventory management system built with Rust, designed to help businesses track products, sales, and purchases efficiently.

## 🚀 Features

- **Product Management**
  - Add new products with details (name, description, quantity, price)
  - Edit existing product information
  - Delete products from inventory
  
- **Transaction Recording**
  - Record product sales with quantities and prices
  - Track product purchases and stock updates
  - Automatic inventory adjustment on sales/purchases

- **Reporting System**
  - Generate detailed inventory reports
  - View comprehensive sales history
  - Access purchase transaction records

## 📋 Prerequisites

To run this project, you need to have Rust and Cargo installed on your system. If you haven't installed them yet, follow these steps:

1. Install Rust by following the official guide at [rust-lang.org](https://www.rust-lang.org/tools/install)
2. Verify your installation by running:
   ```bash
   rustc --version
   cargo --version
   ```

## ⚡ Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/qbit-glitch/rusty-store.git
   cd rusty-store
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run
   ```

## 💻 Usage

When you run the program, you'll be presented with a menu of options:

```
Inventory Management System
Choose an option:
1. "add" - Add Product
2. "edit" - Edit Product
3. "delete" - Delete Product
4. "record-sale" - Record Sale
5. "record-purchase" - Record Purchase
6. "inventory-report" - Generate Inventory Report
7. "sales-report" - Generate Sales Report
8. "purchase-report" - Generate Purchase Report
9. "exit" - Exit Program
```

### Adding a Product
```bash
Enter Product Details:
Product Name: Widget
Description: A fantastic widget
Quantity: 100
Price: 19.99
```

### Recording a Sale
```bash
Enter product name: Widget
Enter quantity to sell: 5
Enter sale price: 24.99
```

### Generating Reports
Choose options 6-8 to generate different types of reports:
- Inventory Report: Shows current stock levels and product details
- Sales Report: Displays all sales transactions
- Purchase Report: Lists all purchase transactions

## 🏗️ Project Structure

```
src/
├── main.rs          # Main application code
└── lib.rs           # Library code (if applicable)

Cargo.toml           # Project dependencies and metadata
README.md           # Project documentation
```

## 🛠️ Technical Details

### Data Structures
- `Product`: Stores product information
- `Sale`: Records sale transactions
- `Purchase`: Tracks purchase transactions
- `Inventory`: Manages product collection
- `RecordSales`: Maintains sales history
- `RecordPurchases`: Stores purchase records

### Key Functions
- `add_product()`: Adds new products to inventory
- `update_product()`: Modifies existing product details
- `delete_product()`: Removes products from inventory
- `record_sale_from_user()`: Records sales transactions
- `record_purchase_from_user()`: Records purchase transactions
- `generate_*_report()`: Generates various reports

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Rust Programming Language Team
- All contributors who participate in this project

## 📧 Contact

Your Name - [@qbit-glitch](https://twitter.com/qbit-glitch)

Project Link: [https://github.com/qbit-glitch/rusty-store](https://github.com/qbit-glitch/rusty-store)

---

⭐️ Star this repository if you find it helpful!