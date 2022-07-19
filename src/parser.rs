// Copyright: Elliot Xu<hack00mind@gmail.com>
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "vim.pest"]
pub struct VimParser;
