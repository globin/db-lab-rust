#![feature(slicing_syntax)]
#![feature(if_let)]
#![feature(phase)]

extern crate collections;
extern crate time;

#[phase(plugin, link)]
extern crate log;

use std::collections::HashMap;
use std::io::{BufferedReader, File};
use std::str::CharSplits;
use time::precise_time_ns;

use collections::hash::Hash;

use numeric::Numeric;

mod numeric;

trait PrimaryKey<T> {
    fn primary_key(&self) -> T;
}

trait FromRow {
    fn from_row(mut row: CharSplits<char>) -> Self;
}

#[deriving(Show)]
struct Table<Data, Index: Eq + Hash> {
    rows: Vec<Data>,
    index: HashMap<Index, uint>
}

impl<Data: PrimaryKey<Index> + Clone, Index: Eq + Hash> Table<Data, Index> {
    fn new() -> Table<Data, Index> {
        Table {
            rows: Vec::new(),
            index: HashMap::new()
        }
    }

    fn insert(&mut self, data: Data) {
        self.rows.push(data.clone());
        self.index.insert(data.primary_key(), self.rows.len() - 1);
    }

    fn size(&self) -> uint {
        self.rows.len()
    }
}

#[deriving(Clone, Show)]
struct Warehouse {
    w_id: i32,
    w_name: String,
    w_street_1: String,
    w_street_2: String,
    w_city: String,
    w_state: String,
    w_zip: String,
    w_tax: Numeric, // numeric(4, 4)
    w_ytd: Numeric, // numeric(12, 2)
}

impl PrimaryKey<i32> for Warehouse {
    fn primary_key(&self) -> i32 {
        self.w_id
    }
}

impl FromRow for Warehouse {
    fn from_row(mut row: CharSplits<char>) -> Warehouse {
        Warehouse {
            w_id: from_str(row.next().unwrap()).unwrap(),
            w_name: row.next().unwrap().into_string(),
            w_street_1: row.next().unwrap().into_string(),
            w_street_2: row.next().unwrap().into_string(),
            w_city: row.next().unwrap().into_string(),
            w_state: row.next().unwrap().into_string(),
            w_zip: row.next().unwrap().into_string(),
            w_tax: Numeric::from_str(row.next().unwrap(), 4, 4).unwrap(),
            w_ytd: Numeric::from_str(row.next().unwrap(), 12, 2).unwrap(),
        }
    }
}

type WarehouseTable = Table<Warehouse, i32>;


#[deriving(Clone, Show)]
struct District {
    d_id: i32,
    d_w_id: i32,
    d_name: String,
    d_street_1: String,
    d_street_2: String,
    d_city: String,
    d_state: String,
    d_zip: String,
    d_tax: Numeric, // numeric(4, 4)
    d_ytd: Numeric, // numeric(12,2)
    d_next_o_id: i32,
}

impl PrimaryKey<(i32, i32)> for District {
    fn primary_key(&self) -> (i32, i32) {
        (self.d_w_id, self.d_id)
    }
}

impl FromRow for District {
    fn from_row(mut row: CharSplits<char>) -> District {
        District {
            d_id: from_str(row.next().unwrap()).unwrap(),
            d_w_id: from_str(row.next().unwrap()).unwrap(),
            d_name: row.next().unwrap().into_string(),
            d_street_1: row.next().unwrap().into_string(),
            d_street_2: row.next().unwrap().into_string(),
            d_city: row.next().unwrap().into_string(),
            d_state: row.next().unwrap().into_string(),
            d_zip: row.next().unwrap().into_string(),
            d_tax: Numeric::from_str(row.next().unwrap(), 4, 4).unwrap(),
            d_ytd: Numeric::from_str(row.next().unwrap(), 12, 2).unwrap(),
            d_next_o_id: from_str(row.next().unwrap()).unwrap(),
        }
    }
}

type DistrictTable = Table<District, (i32, i32)>;


#[deriving(Clone, Show)]
struct Customer {
    c_id: i32,
    c_d_id: i32,
    c_w_id: i32,
    c_first: String,
    c_middle: String,
    c_last: String,
    c_street_1: String,
    c_street_2: String,
    c_city: String,
    c_state: String,
    c_zip: String,
    c_phone: String,
    c_since: i32, // Timestamp
    c_credit: String,
    c_credit_lim: Numeric, // numeric(12,2)
    c_discount: Numeric, // numeric(4, 4)
    c_balance: Numeric, // numeric(12,2)
    c_ytd_paymenr: Numeric, // numeric(12,2)
    c_payment_cnt: Numeric, // numeric(4,0)
    c_delivery_cnt: Numeric, // numeric(4,0)
    c_data: String,
}

impl PrimaryKey<(i32, i32, i32)> for Customer {
    fn primary_key(&self) -> (i32, i32, i32) {
        (self.c_w_id, self.c_d_id, self.c_id)
    }
}

impl FromRow for Customer {
    fn from_row(mut row: CharSplits<char>) -> Customer {
        Customer {
            c_id: from_str(row.next().unwrap()).unwrap(),
            c_d_id: from_str(row.next().unwrap()).unwrap(),
            c_w_id: from_str(row.next().unwrap()).unwrap(),
            c_first: row.next().unwrap().into_string(),
            c_middle: row.next().unwrap().into_string(),
            c_last: row.next().unwrap().into_string(),
            c_street_1: row.next().unwrap().into_string(),
            c_street_2: row.next().unwrap().into_string(),
            c_city: row.next().unwrap().into_string(),
            c_state: row.next().unwrap().into_string(),
            c_zip: row.next().unwrap().into_string(),
            c_phone: row.next().unwrap().into_string(),
            c_since: from_str(row.next().unwrap()).unwrap(), // Timestamp
            c_credit: row.next().unwrap().into_string(),
            c_credit_lim: Numeric::from_str(row.next().unwrap(), 12, 2).unwrap(), // numeric(12,2)
            c_discount: Numeric::from_str(row.next().unwrap(), 4, 4).unwrap(), // numeric(4, 4)
            c_balance: Numeric::from_str(row.next().unwrap(), 12, 2).unwrap(), // numeric(12,2)
            c_ytd_paymenr: Numeric::from_str(row.next().unwrap(), 12, 2).unwrap(), // numeric(12,2)
            c_payment_cnt: Numeric::from_str(row.next().unwrap(), 4, 0).unwrap(), // numeric(4,0)
            c_delivery_cnt: Numeric::from_str(row.next().unwrap(), 4, 0).unwrap(), // numeric(4,0)
            c_data: row.next().unwrap().into_string(),
        }
    }
}

type CustomerTable = Table<Customer, (i32, i32, i32)>;


#[deriving(Clone, Show)]
struct Neworder {
    no_o_id: i32,
    no_d_id: i32,
    no_w_id: i32,
}

impl PrimaryKey<(i32, i32, i32)> for Neworder {
    fn primary_key(&self) -> (i32, i32, i32) {
        (self.no_w_id, self.no_d_id, self.no_o_id)
    }
}

impl FromRow for Neworder {
    fn from_row(mut row: CharSplits<char>) -> Neworder {
        Neworder {
            no_o_id: from_str(row.next().unwrap()).unwrap(),
            no_d_id: from_str(row.next().unwrap()).unwrap(),
            no_w_id: from_str(row.next().unwrap()).unwrap(),
        }
    }
}

type NeworderTable = Table<Neworder, (i32, i32, i32)>;


#[deriving(Clone, Show)]
struct Order {
    o_id: i32,
    o_d_id: i32,
    o_w_id: i32,
    o_c_id: i32,
    o_entry_d: i32, // Timestamp
    o_carrier_id: i32,
    o_ol_cnt: Numeric, // numeric(2,0)
    o_all_local: Numeric, // numeric(1, 0)
}

impl PrimaryKey<(i32, i32, i32)> for Order {
    fn primary_key(&self) -> (i32, i32, i32) {
        (self.o_w_id, self.o_d_id, self.o_id)
    }
}

impl FromRow for Order {
    fn from_row(mut row: CharSplits<char>) -> Order {
        Order {
            o_id: from_str(row.next().unwrap()).unwrap(),
            o_d_id: from_str(row.next().unwrap()).unwrap(),
            o_w_id: from_str(row.next().unwrap()).unwrap(),
            o_c_id: from_str(row.next().unwrap()).unwrap(),
            o_entry_d: from_str(row.next().unwrap()).unwrap(),
            o_carrier_id: from_str(row.next().unwrap()).unwrap(),
            o_ol_cnt: Numeric::from_str(row.next().unwrap(), 2, 0).unwrap(),
            o_all_local: Numeric::from_str(row.next().unwrap(), 1, 0).unwrap(),
        }
    }
}

type OrderTable = Table<Order, (i32, i32, i32)>;


#[deriving(Clone, Show)]
struct Orderline {
    ol_o_id: i32,
    ol_d_id: i32,
    ol_w_id: i32,
    ol_number: i32,
    ol_i_id: i32,
    ol_supply_w_id: i32,
    ol_delivery_d: i32,
    ol_quantity: Numeric, // numeric(2,0)
    ol_amount: Numeric, // numeric(6, 2)
    ol_dist_info: String,
}

impl PrimaryKey<(i32, i32, i32, i32)> for Orderline {
    fn primary_key(&self) -> (i32, i32, i32, i32) {
        (self.ol_w_id, self.ol_d_id, self.ol_o_id, self.ol_number)
    }
}

impl FromRow for Orderline {
    fn from_row(mut row: CharSplits<char>) -> Orderline {
        Orderline {
            ol_o_id: from_str(row.next().unwrap()).unwrap(),
            ol_d_id: from_str(row.next().unwrap()).unwrap(),
            ol_w_id: from_str(row.next().unwrap()).unwrap(),
            ol_number: from_str(row.next().unwrap()).unwrap(),
            ol_i_id: from_str(row.next().unwrap()).unwrap(),
            ol_supply_w_id: from_str(row.next().unwrap()).unwrap(),
            ol_delivery_d: from_str(row.next().unwrap()).unwrap(),
            ol_quantity: Numeric::from_str(row.next().unwrap(), 2, 0).unwrap(),
            ol_amount: Numeric::from_str(row.next().unwrap(), 6, 2).unwrap(),
            ol_dist_info: row.next().unwrap().into_string(),
        }
    }
}

type OrderlineTable = Table<Orderline, (i32, i32, i32, i32)>;


#[deriving(Clone, Show)]
struct Item {
    i_id: i32,
    i_im_id: i32,
    i_name: String,
    i_price: Numeric, // numeric(5,2)
    i_data: String,
}

impl PrimaryKey<i32> for Item {
    fn primary_key(&self) -> i32 {
        self.i_id
    }
}

impl FromRow for Item {
    fn from_row(mut row: CharSplits<char>) -> Item {
        Item {
            i_id: from_str(row.next().unwrap()).unwrap(),
            i_im_id: from_str(row.next().unwrap()).unwrap(),
            i_name: row.next().unwrap().into_string(),
            i_price: Numeric::from_str(row.next().unwrap(), 5, 2).unwrap(),
            i_data: row.next().unwrap().into_string(),
        }
    }
}

type ItemTable = Table<Item, i32>;


#[deriving(Clone, Show)]
struct Stock {
    s_i_id: i32,
    s_w_id: i32,
    s_quantity: Numeric, // numeric(4,0)
    s_dist_01: String,
    s_dist_02: String,
    s_dist_03: String,
    s_dist_04: String,
    s_dist_05: String,
    s_dist_06: String,
    s_dist_07: String,
    s_dist_08: String,
    s_dist_09: String,
    s_dist_10: String,
    s_ytd: Numeric, // numeric(8,0)
    s_order_cnt: Numeric, // numeric(4, 0)
    s_remote_cnt: Numeric, // numeric(4,0)
    s_data: String,
}

impl PrimaryKey<(i32, i32)> for Stock {
    fn primary_key(&self) -> (i32, i32) {
        (self.s_w_id, self.s_i_id)
    }
}

impl FromRow for Stock {
    fn from_row(mut row: CharSplits<char>) -> Stock {
        Stock {
            s_i_id: from_str(row.next().unwrap()).unwrap(),
            s_w_id: from_str(row.next().unwrap()).unwrap(),
            s_quantity: Numeric::from_str(row.next().unwrap(), 4, 0).unwrap(),
            s_dist_01: row.next().unwrap().into_string(),
            s_dist_02: row.next().unwrap().into_string(),
            s_dist_03: row.next().unwrap().into_string(),
            s_dist_04: row.next().unwrap().into_string(),
            s_dist_05: row.next().unwrap().into_string(),
            s_dist_06: row.next().unwrap().into_string(),
            s_dist_07: row.next().unwrap().into_string(),
            s_dist_08: row.next().unwrap().into_string(),
            s_dist_09: row.next().unwrap().into_string(),
            s_dist_10: row.next().unwrap().into_string(),
            s_ytd: Numeric::from_str(row.next().unwrap(), 8, 0).unwrap(),
            s_order_cnt: Numeric::from_str(row.next().unwrap(), 4, 0).unwrap(),
            s_remote_cnt: Numeric::from_str(row.next().unwrap(), 4, 0).unwrap(),
            s_data: row.next().unwrap().into_string(),
        }
    }
}

type StockTable = Table<Stock, (i32, i32)>;


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
