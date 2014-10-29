#![feature(slicing_syntax)]
#![feature(if_let)]
#![feature(globs)]
#![feature(phase)]


extern crate collections;
extern crate rand;
extern crate time;

#[phase(plugin, link)]
extern crate log;
#[phase(plugin)]
extern crate sql_generator;

//use std::io::{BufferedReader, File};
//use std::num::abs;

//use collections::hash::Hash;
//use time::precise_time_ns;

//use numeric::Numeric;
//use tables::*;
//use table::{FromRow, PrimaryKey, Table};


//mod numeric;
//mod table;
//mod tables;


//fn urand<Rng: std::rand::Rng>(min: i32, max: i32, rng: &mut Rng) -> i32 {
    //abs(rng.gen::<i32>() % (max - min + 1)) + min
//}

//fn nurand<Rng: std::rand::Rng>(a: i32, x: i32, y: i32, rng: &mut Rng) -> i32 {
    //(((abs(rng.gen::<i32>() % a) | (abs(rng.gen::<i32>() % (y - x + 1)) + x)) + 42)
        //% (y - x + 1)) + x
//}

//fn urandexcept<Rng: std::rand::Rng>(min: i32, max: i32, v: i32, rng: &mut Rng) -> i32 {
    //if max <= min {
        //return min;
    //}
    //let r = abs(rng.gen::<i32>() % (max - min)) + min;
    //if r >= v {
        //r + 1
    //} else {
        //r
    //}
//}


//struct Tables {
    //warehouse_table: WarehouseTable,
    //district_table: DistrictTable,
    //customer_table: CustomerTable,
    //neworder_table: NeworderTable,
    //order_table: OrderTable,
    //orderline_table: OrderlineTable,
    //item_table: ItemTable,
    //stock_table: StockTable,
//}

//impl Tables {
    //fn new() -> Tables {
        //Tables {
            //warehouse_table: Table::new(),
            //district_table: Table::new(),
            //customer_table: Table::new(),
            //neworder_table: Table::new(),
            //order_table: Table::new(),
            //orderline_table: Table::new(),
            //item_table: Table::new(),
            //stock_table: Table::new(),
        //}
    //}
//}


//fn read_table<Data: Clone + FromRow + PrimaryKey<Index>, Index: Eq + Hash>
        //(filename: &str, table: &mut Table<Data, Index>) {
    //let path = Path::new(filename);
    //let mut file = BufferedReader::new(File::open(&path));
    //for line in file.lines() {
        //let row = FromRow::from_row(line.unwrap()[].trim_chars('\n').split('|'));
        //table.insert(row);
    //}
//}

//fn read_tables(tables: &mut Tables) {
    //read_table::<Warehouse, i32>("../data/tpcc_warehouse.tbl", &mut tables.warehouse_table);
    //read_table::<District, (i32, i32)>("../data/tpcc_district.tbl", &mut tables.district_table);
    //read_table::<Customer, (i32, i32, i32)>("../data/tpcc_customer.tbl", &mut tables.customer_table);
    //read_table::<Neworder, (i32, i32, i32)>("../data/tpcc_neworder.tbl", &mut tables.neworder_table);
    //read_table::<Order, (i32, i32, i32)>("../data/tpcc_order.tbl", &mut tables.order_table);
    //read_table::<Orderline, (i32, i32, i32, i32)>("../data/tpcc_orderline.tbl", &mut tables.orderline_table);
    //read_table::<Item, i32>("../data/tpcc_item.tbl", &mut tables.item_table);
    //read_table::<Stock, (i32, i32)>("../data/tpcc_stock.tbl", &mut tables.stock_table);
//}



//const NUM_WAREHOUSES: i32 = 5;

//fn new_order(tables: &mut Tables, w_id: i32, d_id: i32, c_id: i32, ol_cnt: i32,
             //supware: &[i32], itemid: &[i32], qty: &[i32], now: i64) {
    //let w_tax = tables.warehouse_table.lookup(w_id).w_tax;
    //let c_discount = tables.customer_table.lookup((w_id, d_id, c_id)).c_discount;
    //let district = tables.district_table.lookup_mut((w_id, d_id));
    //let o_id = district.d_next_o_id;
    //let d_tax = district.d_tax;

    //district.d_next_o_id = o_id + 1;

    //let mut all_local = 1i64;
    //for i in range(0, ol_cnt as uint) {
        //if w_id != supware[i] {
            //all_local = 0;
        //}
    //}

    //tables.order_table.insert(Order {
        //o_id: o_id, o_d_id: d_id, o_w_id: w_id, o_c_id: c_id, o_entry_d: now,
        //o_carrier_id: 0, o_ol_cnt: Numeric::new(ol_cnt as i64, 1, 0),
        //o_all_local: Numeric::new(all_local, 1, 0)
    //});
    //tables.neworder_table.insert(Neworder { no_o_id: o_id, no_d_id: d_id, no_w_id: w_id });

    //for i in range(0, ol_cnt as uint) {
        //let i_price = tables.item_table.lookup(itemid[i]).i_price;

        //let stock = tables.stock_table.lookup_mut((supware[i], itemid[i]));
        //let s_quantity = stock.s_quantity;
        //let s_remote_cnt = stock.s_remote_cnt;
        //let s_order_cnt = stock.s_order_cnt;
        //let s_dist = match d_id {
            //1 => &stock.s_dist_01,
            //2 => &stock.s_dist_02,
            //3 => &stock.s_dist_03,
            //4 => &stock.s_dist_04,
            //5 => &stock.s_dist_05,
            //6 => &stock.s_dist_06,
            //7 => &stock.s_dist_07,
            //8 => &stock.s_dist_08,
            //9 => &stock.s_dist_09,
            //10 => &stock.s_dist_10,
            //_ => fail!("invalid d_id: {}", d_id)
        //};

        //let qty = Numeric::new(qty[i] as i64, 4, 0);
        //stock.s_quantity = if s_quantity > qty {
            //stock.s_quantity - qty
        //} else {
            //stock.s_quantity + Numeric::new(91, 4, 0) - qty
        //};

        //if supware[i] != w_id {
            //stock.s_remote_cnt = stock.s_remote_cnt + s_remote_cnt;
        //} else {
            //stock.s_order_cnt = s_order_cnt + Numeric::new(1, 4, 0);
        //}

        //let ol_amount = qty * i_price * (Numeric::new(1, 1, 0) + w_tax + d_tax) *
            //(Numeric::new(1, 1, 0) - c_discount);
        //tables.orderline_table.insert(Orderline {
            //ol_o_id: o_id, ol_d_id: d_id, ol_w_id: w_id, ol_number: i as i32 + 1, ol_i_id: itemid[i],
            //ol_supply_w_id: supware[i], ol_delivery_d: 0, ol_quantity: qty, ol_amount: ol_amount,
            //ol_dist_info: s_dist.clone()
        //})
    //}
//}

//fn new_order_random<Rng: std::rand::Rng>(tables: &mut Tables, now: i64, w_id: i32, rng: &mut Rng) {
    //let d_id = urand(1, 1, rng);
    //let c_id = nurand(1023, 1, 3000, rng);
    //let ol_cnt = urand(5, 15, rng);

    //let mut supware = [0i32, ..15];
    //let mut itemid = [0i32, ..15];
    //let mut qty = [0i32, ..15];

    //for i in range(0, ol_cnt as uint) {
        //supware[i] = if urand(1, 100, rng) > 1 {
            //w_id
        //} else {
            //urandexcept(1, NUM_WAREHOUSES, w_id, rng)
        //};
        //itemid[i] = nurand(8191, 1, 100000, rng);
        //qty[i] = urand(1, 10, rng);
    //}

    //new_order(tables, w_id, d_id, c_id, ol_cnt, supware, itemid, qty, now)
//}

//fn print_tables(tables: &Tables) {
    //println!("Warehouse: {}", tables.warehouse_table.size());
    //println!("District: {}", tables.district_table.size());
    //println!("Customer: {}", tables.customer_table.size());
    //println!("Neworder: {}", tables.neworder_table.size());
    //println!("Order: {}", tables.order_table.size());
    //println!("Orderline: {}", tables.orderline_table.size());
    //println!("Item: {}", tables.item_table.size());
    //println!("Stock: {}\n", tables.stock_table.size());
//}
rn!(create table warehouse (
   w_id integer not null,
   w_name varchar(10) not null,
   w_street_1 varchar(20) not null,
   w_street_2 varchar(20) not null,
   w_city varchar(20) not null,
   w_state char(2) not null,
   w_zip char(9) not null,
   w_tax numeric(4,4) not null,
   w_ytd numeric(12,2) not null,
   primary key (w_id)
);

create table district (
   d_id integer not null,
   d_w_id integer not null,
   d_name varchar(10) not null,
   d_street_1 varchar(20) not null,
   d_street_2 varchar(20) not null,
   d_city varchar(20) not null,
   d_state char(2) not null,
   d_zip char(9) not null,
   d_tax numeric(4,4) not null,
   d_ytd numeric(12,2) not null,
   d_next_o_id integer not null,
   primary key (d_w_id,d_id)
);

create table customer (
   c_id integer not null,
   c_d_id integer not null,
   c_w_id integer not null,
   c_first varchar(16) not null,
   c_middle char(2) not null,
   c_last varchar(16) not null,
   c_street_1 varchar(20) not null,
   c_street_2 varchar(20) not null,
   c_city varchar(20) not null,
   c_state char(2) not null,
   c_zip char(9) not null,
   c_phone char(16) not null,
   c_since timestamp not null,
   c_credit char(2) not null,
   c_credit_lim numeric(12,2) not null,
   c_discount numeric(4,4) not null,
   c_balance numeric(12,2) not null,
   c_ytd_paymenr numeric(12,2) not null,
   c_payment_cnt numeric(4,0) not null,
   c_delivery_cnt numeric(4,0) not null,
   c_data varchar(500) not null,
   primary key (c_w_id,c_d_id,c_id)
);

create table history (
   h_c_id integer not null,
   h_c_d_id integer not null,
   h_c_w_id integer not null,
   h_d_id integer not null,
   h_w_id integer not null,
   h_date timestamp not null,
   h_amount numeric(6,2) not null,
   h_data varchar(24) not null
);

create table neworder (
   no_o_id integer not null,
   no_d_id integer not null,
   no_w_id integer not null,
   primary key (no_w_id,no_d_id,no_o_id)
);

create table "order" (
   o_id integer not null,
   o_d_id integer not null,
   o_w_id integer not null,
   o_c_id integer not null,
   o_entry_d timestamp not null,
   o_carrier_id integer not null,
   o_ol_cnt numeric(2,0) not null,
   o_all_local numeric(1,0) not null,
   primary key (o_w_id,o_d_id,o_id)
);

create table orderline (
   ol_o_id integer not null,
   ol_d_id integer not null,
   ol_w_id integer not null,
   ol_number integer not null,
   ol_i_id integer not null,
   ol_supply_w_id integer not null,
   ol_delivery_d timestamp not null,
   ol_quantity numeric(2,0) not null,
   ol_amount numeric(6,2) not null,
   ol_dist_info char(24) not null,
   primary key (ol_w_id,ol_d_id,ol_o_id,ol_number)
);

create table item (
   i_id integer not null,
   i_im_id integer not null,
   i_name varchar(24) not null,
   i_price numeric(5,2) not null,
   i_data varchar(50) not null,
   primary key (i_id)
);

create table stock (
   s_i_id integer not null,
   s_w_id integer not null,
   s_quantity numeric(4,0) not null,
   s_dist_01 char(24) not null,
   s_dist_02 char(24) not null,
   s_dist_03 char(24) not null,
   s_dist_04 char(24) not null,
   s_dist_05 char(24) not null,
   s_dist_06 char(24) not null,
   s_dist_07 char(24) not null,
   s_dist_08 char(24) not null,
   s_dist_09 char(24) not null,
   s_dist_10 char(24) not null,
   s_ytd numeric(8,0) not null,
   s_order_cnt numeric(4,0) not null,
   s_remote_cnt numeric(4,0) not null,
   s_data varchar(50) not null,
   primary key (s_w_id,s_i_id)
);
    )
fn main() {
    println!("{}", Test{ a: 1});
    //let time = precise_time_ns();

    //let tables: &mut Tables =  &mut Tables::new();
    //read_tables(tables);

    //println!("insert {}s", ((precise_time_ns() - time) as f64) / 1e9f64);
    //print_tables(tables);

    //let time = precise_time_ns();

    //let rng = &mut std::rand::task_rng();
    //for _ in range(0u, 1000000) {
        //new_order_random(tables, time::get_time().sec, urand(1, NUM_WAREHOUSES, rng), rng)
    //}

    //println!("1000000 neworder iterations {}s",
             //((precise_time_ns() - time) as f64) / 1e9f64);
    //print_tables(tables);
}
