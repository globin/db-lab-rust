#![feature(slicing_syntax)]
#![feature(if_let)]
#![feature(globs)]
#![feature(phase)]

extern crate collections;
extern crate time;

#[phase(plugin, link)]
extern crate log;

use std::io::{BufferedReader, File};
use time::precise_time_ns;

use collections::hash::Hash;

use tables::*;
use table::{FromRow, PrimaryKey, Table};

mod numeric;
mod table;
mod tables;


struct Tables {
    warehouse_table: WarehouseTable,
    district_table: DistrictTable,
    customer_table: CustomerTable,
    neworder_table: NeworderTable,
    order_table: OrderTable,
    orderline_table: OrderlineTable,
    item_table: ItemTable,
    stock_table: StockTable,
}

impl Tables {
    fn new() -> Tables {
        Tables {
            warehouse_table: Table::new(),
            district_table: Table::new(),
            customer_table: Table::new(),
            neworder_table: Table::new(),
            order_table: Table::new(),
            orderline_table: Table::new(),
            item_table: Table::new(),
            stock_table: Table::new(),
        }
    }
}

fn read_table<Data: Clone + FromRow + PrimaryKey<Index>, Index: Eq + Hash>
        (filename: &str, table: &mut Table<Data, Index>) {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));
    for line in file.lines() {
        let row = FromRow::from_row(line.unwrap()[].trim_chars('\n').split('|'));
        table.insert(row);
    }
}

fn read_tables(tables: &mut Tables) {
    read_table::<Warehouse, i32>("../data/tpcc_warehouse.tbl", &mut tables.warehouse_table);
    read_table::<District, (i32, i32)>("../data/tpcc_district.tbl", &mut tables.district_table);
    read_table::<Customer, (i32, i32, i32)>("../data/tpcc_customer.tbl", &mut tables.customer_table);
    read_table::<Neworder, (i32, i32, i32)>("../data/tpcc_neworder.tbl", &mut tables.neworder_table);
    read_table::<Order, (i32, i32, i32)>("../data/tpcc_order.tbl", &mut tables.order_table);
    read_table::<Orderline, (i32, i32, i32, i32)>("../data/tpcc_orderline.tbl", &mut tables.orderline_table);
    read_table::<Item, i32>("../data/tpcc_item.tbl", &mut tables.item_table);
    read_table::<Stock, (i32, i32)>("../data/tpcc_stock.tbl", &mut tables.stock_table);
}

fn main() {
    let time = precise_time_ns();

    let tables: &mut Tables =  &mut Tables::new();
    read_tables(tables);
    println!("Warehouse: {}", tables.warehouse_table.size());
    println!("District: {}", tables.district_table.size());
    println!("Customer: {}", tables.customer_table.size());
    println!("Neworder: {}", tables.neworder_table.size());
    println!("Order: {}", tables.order_table.size());
    println!("Orderline: {}", tables.orderline_table.size());
    println!("Item: {}", tables.item_table.size());
    println!("Stock: {}", tables.stock_table.size());

    println!("insert {}s", ((precise_time_ns() - time) as f64) / 1e9f64);
}
