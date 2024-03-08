// RustyDO
//
// Copyright 2022. JungHyun Kim (jidoc01).
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

pub use nosqlite::*;
use serde::{Serialize, Deserialize};

pub const USER_TBL: &str = "user";
pub const POST_TBL: &str = "post";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserSchema {
    pub id: String,
    pub pw: String, // it should be hashed.
    pub name: String,
    pub level: u8,
    pub is_female: bool,
    pub money: u32,
    pub items: Vec<u8>, // total: 4
    pub exps: Vec<u32>, // total: 8
    pub is_admin: bool,
    pub is_muted: bool,
    pub is_banned: bool,

    pub setting: SettingSchema
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SettingSchema {
    pub key_binding: u8,
    pub bgm_volume: u8,
    pub bgm_mute: bool,
    pub bgm_echo: bool,
    pub sound_volume: u8,
    pub sound_mute: bool,
    pub macros: Vec<String>, // total: 8
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DateTimeSchema {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub min: u32,
}

impl DateTimeSchema {
    pub fn now() -> Self {
        use chrono::prelude::*;

        let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
        let date = local.date();
        let time = local.time();
        
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
            hour: time.hour(),
            min: time.minute()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PostSchema {
    pub post_id: u32,
    pub parent_post_id: Option<u32>,
    pub writer_id: String,
    pub title: String,
    pub text: String,
    pub datetime: DateTimeSchema,
    pub is_deleted: bool,
    pub is_notice: bool,
    pub views: u16,
}

/*

pub struct Db {
    conn: Connection
}

impl Db {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self {
            conn: conn
        })
    }

    pub fn insert<T: Serialize>(&self, coll_name: &str, data: T) {
        let conn = &self.conn;
        let coll = conn.table(coll_name).unwrap();
        coll.insert(data, conn);
    }
    
    pub fn select<A: Filter>(&self, coll_name: &str, filter: A) -> Iterator<I, (), ()> {
        let conn = &self.conn;
        let coll = conn.table(coll_name).unwrap();
        coll
            .iter()
            .filter(field)
            .set(field, value, connection)
    }
}
*/

//use hotpot_db::*;

// Just export pub uses.
/* 
pub use rusqlite::{Connection};

pub const TBL_USER: &str = "user";
pub const TBL_BOARD: &str = "board";


pub struct Db {
    conn: Connection
}

impl Db {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;


        Ok(Self {
            conn: conn
        })
    }
}
*/

/*
use std::io::BufRead;

use rocksdb::{DB, Options};

pub struct Db {
    db: rocksdb::DBWithThreadMode<rocksdb::SingleThreaded>,
}

const STRING_DELIMITER: u8 = 0xff;

impl Db {
    pub fn open(path: &str) -> Result<Db> {
        let conn = DB::open_default(path)?;
        let db = Db {
            db: conn,
        };
        Ok(db)
    }

    pub fn test(&self) {
        let db = &self.db;
        let mut it = db.iterator(rocksdb::IteratorMode::Start);
        while let Some((k, v)) = it.next() {
            print!("{} ", String::from_utf8(k.to_vec()).unwrap());
            println!("{:?} {:?}", k, v);
        }
    }

    pub fn get_string(&self, key: &str) -> Result<String> {
        let db = &self.db;
        let val = match db.get(key)? {
            Some(val) => val,
            _ => bail!("Not exists")
        };
        let str = String::from_utf8(val).unwrap();
        Ok(str)
    }

    pub fn get_i64(&self, key: &str) -> Result<i64> {
        let db = &self.db;
        let val = match db.get_pinned(key)? {
            Some(val) => val,
            _ => bail!("Not exists")
        };
        let mut fixed_val: [u8; 8] = Default::default();
        for (i, elem) in val.iter().enumerate() {
            fixed_val[i] = *elem;
        }
        let ret = i64::from_le_bytes(fixed_val);
        Ok(ret)
    }
    
    pub fn get_i64_vec(&self, key: &str) -> Result<Vec<i64>> {
        let db = &self.db;
        let val = match db.get_pinned(key)? {
            Some(val) => val,
            _ => bail!("Not exists")
        };
        let len = val.len();
        assert_eq!(len % 8, 0);
        let count = len / 8;
        let ret = (0..count)
            .map(|i| {
                let mut fixed_val: [u8; 8] = Default::default();
                let s = i * 8;
                let curr_val = &val[s..s+8];
                for (j, elem) in curr_val.iter().enumerate() {
                    fixed_val[i] = *elem;
                }
                let word = i64::from_le_bytes(fixed_val);
                word
            })
            .collect();
        Ok(ret)
    }

    pub fn get_string_vec(&self, key: &str) -> Result<Vec<String>> {
        let db = &self.db;
        let val = match db.get(key)? {
            Some(val) => val,
            _ => bail!("Not exists")
        };
        let ret = val
            .split(|ch| *ch == STRING_DELIMITER)
            .map(|bytes| String::from_utf8(Vec::from(bytes)).unwrap())
            .collect();
        Ok(ret)
    }
    
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        let db = &self.db;
        let val = match db.get_pinned(key)? {
            Some(val) => val,
            _ => bail!("Not exists")
        };
        let ret = match val[0] {
            1 => true,
            _ => false,
        };
        Ok(ret)
    }

    /// TODO: It always induces memory copy for returning the value.
    pub fn get_bytes(&self, key: &str) -> Result<Vec<u8>> {
        let db = &self.db;
        let val = match db.get(key)? {
            Some(val) => val,
            _ => bail!("Not exists")
        };
        Ok(val)
    }

    pub fn put_string(&self, key: &str, val: &str) {
        let db = &self.db;
        let _ = db.put(key, val);
    }
    
    pub fn put_i64(&self, key: &str, val: i64) {
        let db = &self.db;
        let fixed_val: [u8; 8] = val.to_le_bytes();
        let _ = db.put(key, fixed_val);
    }
    
    pub fn put_bool(&self, key: &str, val: bool) {
        let db = &self.db;
        let mut fixed_val: [u8; 1] = Default::default();
        if val {
            fixed_val[0] = 1;
        } else {
            fixed_val[0] = 0;
        }
        let _ = db.put(key, fixed_val);
    }

    // TODO: Reduce overhead.
    pub fn put_i64_vec(&self, key: &str, val: &Vec<i64>) {
        let db = &self.db;
        let mut acc = vec!();
        val
            .iter()
            .for_each(|elem| {
                let mut v = elem.to_le_bytes().to_vec();
                acc.append(&mut v);
            });
        let _ = db.put(key, acc);
    }
    
    pub fn put_string_vec(&self, key: &str, val: &Vec<String>) {
        let db = &self.db;
        let mut acc = vec!();
        val
            .iter()
            .enumerate()
            .for_each(|(i, elem)| {
                let mut v = elem.as_bytes().to_vec();
                if i != 0 {
                    acc.push(STRING_DELIMITER);
                }
                acc.append(&mut v);
            });
        let _ = db.put(key, acc);
    }
    
    pub fn put_bytes(&self, key: &str, bytes: Vec<u8>) {
        let db = &self.db;
        let _ = db.put(key, &bytes);
    }
}
*/