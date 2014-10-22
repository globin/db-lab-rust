#![feature(slicing_syntax)]
#![feature(if_let)]
#![feature(globs)]
#![feature(phase)]


extern crate collections;
extern crate rand;
extern crate time;

#[phase(plugin, link)]
extern crate log;


use std::io::{BufferedReader, File};
use std::num::abs;

use collections::hash::Hash;
use time::precise_time_ns;

use numeric::Numeric;
use tables::*;
use table::{FromRow, PrimaryKey, Table};


mod numeric;
mod table;
mod tables;


fn urand<Rng: std::rand::Rng>(min: i32, max: i32, rng: &mut Rng) -> i32 {
    abs(rng.gen::<i32>() % (max - min + 1)) + min
}

fn nurand<Rng: std::rand::Rng>(a: i32, x: i32, y: i32, rng: &mut Rng) -> i32 {
    (((abs(rng.gen::<i32>() % a) | (abs(rng.gen::<i32>() % (y - x + 1)) + x)) + 42)
        % (y - x + 1)) + x
}

fn urandexcept<Rng: std::rand::Rng>(min: i32, max: i32, v: i32, rng: &mut Rng) -> i32 {
    if max <= min {
        return min;
    }
    let r = abs(rng.gen::<i32>() % (max - min)) + min;
    if r >= v {
        r + 1
    } else {
        r
    }
}


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



const NUM_WAREHOUSES: i32 = 5;

fn new_order(tables: &mut Tables, w_id: i32, d_id: i32, c_id: i32, ol_cnt: i32,
             supware: &[i32], itemid: &[i32], qty: &[i32], now: i64) {
    let w_tax = tables.warehouse_table.lookup(w_id).w_tax;
    let c_discount = tables.customer_table.lookup((w_id, d_id, c_id)).c_discount;
    let district = tables.district_table.lookup_mut((w_id, d_id));
    let o_id = district.d_next_o_id;
    let d_tax = district.d_tax;

    district.d_next_o_id = o_id + 1;

    let mut all_local = 1i64;
    for i in range(0, ol_cnt as uint) {
        if w_id != supware[i] {
            all_local = 0;
        }
    }

    tables.order_table.insert(Order {
        o_id: o_id, o_d_id: d_id, o_w_id: w_id, o_c_id: c_id, o_entry_d: now,
        o_carrier_id: 0, o_ol_cnt: Numeric::new(ol_cnt as i64, 1, 0),
        o_all_local: Numeric::new(all_local, 1, 0)
    });
    tables.neworder_table.insert(Neworder { no_o_id: o_id, no_d_id: d_id, no_w_id: w_id });

    for i in range(0, ol_cnt as uint) {
        let i_price = tables.item_table.lookup(itemid[i]).i_price;

        let stock = tables.stock_table.lookup_mut((supware[i], itemid[i]));
        let s_quantity = stock.s_quantity;
        let s_remote_cnt = stock.s_remote_cnt;
        let s_order_cnt = stock.s_order_cnt;
        let s_dist = match d_id {
            1 => &stock.s_dist_01,
            2 => &stock.s_dist_02,
            3 => &stock.s_dist_03,
            4 => &stock.s_dist_04,
            5 => &stock.s_dist_05,
            6 => &stock.s_dist_06,
            7 => &stock.s_dist_07,
            8 => &stock.s_dist_08,
            9 => &stock.s_dist_09,
            10 => &stock.s_dist_10,
            _ => fail!("invalid d_id: {}", d_id)
        };

        let qty = Numeric::new(qty[i] as i64, 4, 0);
        stock.s_quantity = if s_quantity > qty {
            stock.s_quantity - qty
        } else {
            stock.s_quantity + Numeric::new(91, 4, 0) - qty
        };

        if supware[i] != w_id {
            stock.s_remote_cnt = stock.s_remote_cnt + s_remote_cnt;
        } else {
            stock.s_order_cnt = s_order_cnt + Numeric::new(1, 4, 0);
        }

        let ol_amount = qty * i_price * (Numeric::new(1, 1, 0) + w_tax + d_tax) *
            (Numeric::new(1, 1, 0) - c_discount);
        tables.orderline_table.insert(Orderline {
            ol_o_id: o_id, ol_d_id: d_id, ol_w_id: w_id, ol_number: i as i32 + 1, ol_i_id: itemid[i],
            ol_supply_w_id: supware[i], ol_delivery_d: 0, ol_quantity: qty, ol_amount: ol_amount,
            ol_dist_info: s_dist.clone()
        })
    }
}

fn new_order_random<Rng: std::rand::Rng>(tables: &mut Tables, now: i64, w_id: i32, rng: &mut Rng) {
    let d_id = urand(1, 1, rng);
    let c_id = nurand(1023, 1, 3000, rng);
    let ol_cnt = urand(5, 15, rng);

    let mut supware = [0i32, ..15];
    let mut itemid = [0i32, ..15];
    let mut qty = [0i32, ..15];

    for i in range(0, ol_cnt as uint) {
        supware[i] = if urand(1, 100, rng) > 1 {
            w_id
        } else {
            urandexcept(1, NUM_WAREHOUSES, w_id, rng)
        };
        itemid[i] = nurand(8191, 1, 100000, rng);
        qty[i] = urand(1, 10, rng);
    }

    new_order(tables, w_id, d_id, c_id, ol_cnt, supware, itemid, qty, now)
}

fn print_tables(tables: &Tables) {
    println!("Warehouse: {}", tables.warehouse_table.size());
    println!("District: {}", tables.district_table.size());
    println!("Customer: {}", tables.customer_table.size());
    println!("Neworder: {}", tables.neworder_table.size());
    println!("Order: {}", tables.order_table.size());
    println!("Orderline: {}", tables.orderline_table.size());
    println!("Item: {}", tables.item_table.size());
    println!("Stock: {}\n", tables.stock_table.size());
}

fn main() {
    let time = precise_time_ns();

    let tables: &mut Tables =  &mut Tables::new();
    read_tables(tables);

    println!("insert {}s", ((precise_time_ns() - time) as f64) / 1e9f64);
    print_tables(tables);

    let time = precise_time_ns();

    let rng = &mut std::rand::task_rng();
    for _ in range(0u, 1000000) {
        new_order_random(tables, time::get_time().sec, urand(1, NUM_WAREHOUSES, rng), rng)
    }

    println!("1000000 neworder iterations {}s",
             ((precise_time_ns() - time) as f64) / 1e9f64);
    print_tables(tables);
}
