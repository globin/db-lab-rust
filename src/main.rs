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

mod numeric {
    use std::char::{is_whitespace, to_digit};
    use std::num;

    #[deriving(Clone, Show, Eq)]
    pub struct Numeric {
        value: i64,
        len: uint,
        precision: uint
    }

    impl Numeric {
        pub fn from_str(s: &str, len: uint, precision: uint) -> Option<Numeric> {
            let mut s = s.trim_chars(is_whitespace);
            let mut value = 0i64;
            let mut negative = false;
            let mut fraction = false;
            if s[0..1] == "-" {
                negative = true;
                s = s[1..];
            }

            if s.contains_char('.') {
                while s.chars().last() == Some('0') {
                    s = s[..s.len() - 1]
                }
            }

            let mut digits_seen = 0u;
            let mut digits_seen_fraction = 0u;
            for c in s.chars() {
                if let Some(n) = to_digit(c, 10) {
                    value = value * 10 + n as i64;
                    if fraction {
                        digits_seen_fraction += 1;
                    } else {
                        digits_seen += 1;
                    }
                } else if c == '.' {
                    fraction = match fraction {
                        true => return None,
                        false => true
                    };
                } else {
                    return None;
                }
            }

            if negative {
                value *= -1;
            }

            if digits_seen > len - precision || digits_seen_fraction > precision {
                None
            } else {
                Some(Numeric {
                    value: value * num::pow(10, precision - digits_seen_fraction),
                    len: len,
                    precision: precision,
                })
            }
        }
    }

    impl PartialEq for Numeric {
        fn eq(&self, other: &Numeric) -> bool {
            self.value == other.value &&
                self.len == other.len &&
                self.precision == other.precision
        }
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Numeric::from_str("50", 2, 0), Some(Numeric {value: 50, len: 2, precision: 0}))
        assert_eq!(Numeric::from_str("-50", 2, 0), Some(Numeric {value: -50, len: 2, precision: 0}))
        assert_eq!(Numeric::from_str("50.25", 4, 2), Some(Numeric {value: 5025, len: 4, precision: 2}))
        assert_eq!(Numeric::from_str("-50.25", 4, 2), Some(Numeric {value: -5025, len: 4, precision: 2}))
        assert_eq!(Numeric::from_str("-50.250", 4, 2), Some(Numeric {value: -5025, len: 4, precision: 2}))
        assert_eq!(Numeric::from_str("-50.25", 5, 3), Some(Numeric {value: -50250, len: 5, precision: 3}))
        assert_eq!(Numeric::from_str("10.2.1", 4, 0), None)
        assert_eq!(Numeric::from_str("abc", 4, 0), None)
    }
}


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
    w_tax: Numeric, // FIXME: numeric(4, 4)
    w_ytd: Numeric, // FIXME: numeric(12, 2)
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


struct Tables {
    warehouse_table: WarehouseTable,
    district_table: DistrictTable,
    customer_table: CustomerTable,
    neworder_table: NeworderTable,
    order_table: OrderTable,
}

impl Tables {
    fn new() -> Tables {
        Tables {
            warehouse_table: Table::new(),
            district_table: Table::new(),
            customer_table: Table::new(),
            neworder_table: Table::new(),
            order_table: Table::new(),
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

    println!("insert {}s", ((precise_time_ns() - time) as f64) / 1e9f64);
}
