// Copyright 2022 JungHyun Kim
// This file is part of RustyDO.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

pub mod room;
pub mod timer;
pub mod session;

use std::net::SocketAddr;

use super::{component::*, conn::{MsgToConnSender, MsgToConn}};
use crate::prelude::*;

#[derive(PartialEq)]
pub enum EntityKind {
    Client,
    Room,
    Timer,
}