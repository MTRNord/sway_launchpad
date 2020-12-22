use crate::utils::globals::SHOWING_NUMBER;
use crate::utils::numbers::{INNER_KEYS, NUMBERS};
use midir::MidiOutputConnection;
use std::convert::TryInto;
use std::sync::atomic::Ordering;

pub mod globals;
mod numbers;

pub fn show_number(conn_out: &mut MidiOutputConnection, number: usize) {
    SHOWING_NUMBER.store(true, Ordering::SeqCst);
    conn_out.send(&[176, 0, 0]).unwrap();
    show_array(conn_out, NUMBERS[number]);
    conn_out.send(&[144, 24, 63]).unwrap();
    conn_out.send(&[144, 8, 63]).unwrap();
    conn_out.send(&[144, 40, 63]).unwrap();
}

pub fn show_array(conn_out: &mut MidiOutputConnection, array: [[bool; 8]; 8]) {
    for (row_n, row) in array.iter().enumerate() {
        for (column_n, column) in row.iter().enumerate() {
            if *column {
                let key = INNER_KEYS[row_n][column_n];
                conn_out
                    .send(&[144, (key as i32).try_into().unwrap(), 60])
                    .unwrap();
            }
        }
    }
}
