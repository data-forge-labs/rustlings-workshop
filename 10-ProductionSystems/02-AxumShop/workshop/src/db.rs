use sha2::Digest;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

/// Create the SQLite connection pool with WAL mode support.
pub async fn create_pool() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:shop.db?mode=rwc")
        .await
        .expect("Failed to create database pool")
}

/// Create all tables (admins, customers, categories, products, orders, order_items).
/// Enables WAL mode for better concurrent read/write performance.
pub async fn run_migrations(pool: &SqlitePool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS admins (
            id INTEGER NOT NULL,
            username VARCHAR(50) NOT NULL,
            password_hash VARCHAR(128) NOT NULL,
            created_at DATETIME,
            PRIMARY KEY (id),
            UNIQUE (username)
        );
        CREATE TABLE IF NOT EXISTS customers (
            id INTEGER NOT NULL,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(100) NOT NULL,
            phone VARCHAR(20),
            created_at DATETIME,
            PRIMARY KEY (id),
            UNIQUE (email)
        );
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER NOT NULL,
            name VARCHAR(100) NOT NULL,
            description TEXT,
            PRIMARY KEY (id),
            UNIQUE (name)
        );
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER NOT NULL,
            name VARCHAR(200) NOT NULL,
            description TEXT,
            price FLOAT NOT NULL,
            stock INTEGER NOT NULL DEFAULT 0,
            category_id INTEGER NOT NULL,
            created_at DATETIME,
            PRIMARY KEY (id),
            FOREIGN KEY(category_id) REFERENCES categories (id)
        );
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER NOT NULL,
            customer_id INTEGER NOT NULL,
            order_date DATETIME,
            total_amount FLOAT NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'pending',
            PRIMARY KEY (id),
            FOREIGN KEY(customer_id) REFERENCES customers (id)
        );
        CREATE TABLE IF NOT EXISTS order_items (
            id INTEGER NOT NULL,
            order_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            unit_price FLOAT NOT NULL,
            PRIMARY KEY (id),
            FOREIGN KEY(order_id) REFERENCES orders (id),
            FOREIGN KEY(product_id) REFERENCES products (id)
        );
        "#
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query("PRAGMA journal_mode=WAL;")
        .execute(pool)
        .await
        .unwrap();
}

/// Seed the database with sample data matching the original FastAPI seed.py.
pub async fn seed_database(pool: &SqlitePool) {
    // Check if admins already exist (skip seeding)
    let admin_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM admins")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    if admin_count.0 > 0 {
        return;
    }

    // Admin: username "admin", password "admin123"
    let admin_hash = hex::encode(sha2::Sha256::digest(b"admin123"));
    sqlx::query("INSERT INTO admins (username, password_hash, created_at) VALUES (?, ?, datetime('now'))")
        .bind("admin")
        .bind(&admin_hash)
        .execute(pool)
        .await
        .unwrap();

    // Customers
    let customers = vec![
        ("Alice Johnson", "alice@example.com", "+1234567890"),
        ("Bob Smith", "bob@example.com", "+1234567891"),
        ("Charlie Brown", "charlie@example.com", "+1234567892"),
        ("Diana Prince", "diana@example.com", "+1234567893"),
        ("Eve Adams", "eve@example.com", "+1234567894"),
    ];
    for (name, email, phone) in &customers {
        sqlx::query("INSERT INTO customers (name, email, phone, created_at) VALUES (?, ?, ?, datetime('now'))")
            .bind(name).bind(email).bind(phone)
            .execute(pool).await.unwrap();
    }

    // Categories
    let categories = vec![
        ("Electronics", "Devices and gadgets"),
        ("Clothing", "Apparel and accessories"),
        ("Books", "Printed and digital books"),
        ("Home & Garden", "Home improvement and gardening"),
        ("Sports", "Sports equipment and gear"),
    ];
    for (name, desc) in &categories {
        sqlx::query("INSERT INTO categories (name, description) VALUES (?, ?)")
            .bind(name).bind(desc)
            .execute(pool).await.unwrap();
    }

    // Products
    let products = vec![
        ("Laptop", "High-performance laptop", 1200.0, 50, 1),
        ("Smartphone", "Latest model smartphone", 800.0, 100, 1),
        ("Headphones", "Noise-cancelling headphones", 150.0, 200, 1),
        ("Tablet", "Portable tablet computer", 400.0, 75, 1),
        ("Smartwatch", "Fitness tracking smartwatch", 250.0, 150, 1),
        ("T-Shirt", "Cotton t-shirt", 20.0, 500, 2),
        ("Jeans", "Denim jeans", 60.0, 300, 2),
        ("Jacket", "Winter jacket", 120.0, 100, 2),
        ("Sneakers", "Running sneakers", 90.0, 200, 2),
        ("Dress", "Summer dress", 50.0, 150, 2),
        ("Rust Programming", "Learn Rust from scratch", 45.0, 300, 3),
        ("Data Engineering", "Data pipelines and ETL", 55.0, 200, 3),
        ("Machine Learning", "ML fundamentals", 65.0, 150, 3),
        ("Cookbook", "Recipe collection", 25.0, 400, 3),
        ("Garden Tools", "Complete garden tool set", 80.0, 80, 4),
        ("Plant Pots", "Ceramic plant pots set of 3", 35.0, 120, 4),
        ("Yoga Mat", "Premium yoga mat", 40.0, 250, 5),
        ("Dumbbells", "Adjustable dumbbells set", 150.0, 60, 5),
        ("Bicycle", "Mountain bicycle", 500.0, 30, 5),
        ("Tennis Racket", "Professional tennis racket", 120.0, 90, 5),
        ("Swimming Goggles", "Anti-fog swimming goggles", 25.0, 300, 5),
    ];
    for (name, desc, price, stock, cat_id) in &products {
        sqlx::query("INSERT INTO products (name, description, price, stock, category_id, created_at) VALUES (?, ?, ?, ?, ?, datetime('now'))")
            .bind(name).bind(desc).bind(price).bind(stock).bind(cat_id)
            .execute(pool).await.unwrap();
    }

    // Orders
    let orders = vec![
        (1, 450.0, "completed"),
        (2, 120.0, "completed"),
        (1, 800.0, "completed"),
        (3, 250.0, "completed"),
        (4, 150.0, "pending"),
        (5, 90.0, "pending"),
        (2, 600.0, "shipped"),
        (3, 45.0, "shipped"),
    ];
    for (customer_id, total, status) in &orders {
        sqlx::query("INSERT INTO orders (customer_id, order_date, total_amount, status) VALUES (?, datetime('now', ?), ?, ?)")
            .bind(customer_id).bind(format!("-{} hours", orders.iter().position(|o| o == &(*customer_id, *total, *status)).unwrap_or(0)))
            .bind(total).bind(status)
            .execute(pool).await.unwrap();
    }

    // Order Items
    let order_items = vec![
        (1, 1, 2, 1200.0), (1, 2, 1, 800.0),
        (2, 6, 3, 20.0),   (2, 7, 1, 60.0),
        (3, 2, 1, 800.0),
        (4, 5, 1, 250.0),
        (5, 3, 1, 150.0),
        (6, 9, 1, 90.0),
        (7, 1, 1, 1200.0), (7, 13, 1, 65.0),
        (8, 11, 1, 45.0),
    ];
    for (order_id, product_id, qty, price) in &order_items {
        sqlx::query("INSERT INTO order_items (order_id, product_id, quantity, unit_price) VALUES (?, ?, ?, ?)")
            .bind(order_id).bind(product_id).bind(qty).bind(price)
            .execute(pool).await.unwrap();
    }

    println!("Database seeded successfully.");
}
