// This file is part of senpy-rs <https://github.com/senpy-club/senpy-rs>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

//! # senpy-rs
//!
//! A wrapper of The Senpy Club API, which is a wrapper of the
//! [cat-milk/Anime-Girls-Holding-Programming-Books](https://github.com/cat-milk/Anime-Girls-Holding-Programming-Books)
//! repository.

#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]
#![recursion_limit = "128"]
#![doc(
  html_logo_url = "https://senpy.club/favicon.png",
  html_favicon_url = "https://senpy.club/favicon.png"
)]

/// The base URL to The Senpy Club API
pub const SENPY_CLUB_API_BASE_URL: &str = "https://api.senpy.club";
/// The current API version of The Senpy Club API
pub const SENPY_CLUB_API_CURRENT_VERSION: u32 = 2;
/// The API URL to The Senpy Club API
#[allow(clippy::useless_transmute)]
pub const SENPY_CLUB_API_URL: &str = const_format::formatcp!(
  "{}/v{}",
  SENPY_CLUB_API_BASE_URL,
  SENPY_CLUB_API_CURRENT_VERSION
);

/// The response of the <https://api.senpy.club/v2/random> route
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Random {
  pub language: String,
  pub image:    String,
}

/// If all is good with no errors; returns a `Vec` of languages
///
/// # Errors
/// if The Senpy Club API is unavailable
pub fn languages() -> reqwest::Result<Vec<String>> {
  reqwest::blocking::Client::new()
    .get(format!("{}/languages", SENPY_CLUB_API_URL))
    .header("User-Agent", env!("GIT_COMMIT_HASH"))
    .send()?
    .json::<Vec<String>>()
}

/// If all is good with no errors; returns a `Vec` of images of language
/// `language`
///
/// # Errors
/// if The Senpy Club API is unavailable
pub fn language(language: &str) -> reqwest::Result<Vec<String>> {
  reqwest::blocking::Client::new()
    .get(format!("{}/language/{}", SENPY_CLUB_API_URL, language))
    .header("User-Agent", env!("GIT_COMMIT_HASH"))
    .send()?
    .json::<Vec<String>>()
}

/// If all is good with no errors; returns a `Random`
///
/// # Errors
/// if The Senpy Club API is unavailable
pub fn random() -> reqwest::Result<Random> {
  reqwest::blocking::Client::new()
    .get(format!("{}/random", SENPY_CLUB_API_URL))
    .header("User-Agent", env!("GIT_COMMIT_HASH"))
    .send()?
    .json::<Random>()
}

/// If all is good with no errors; returns `true` if live, returns `false` if
/// down.
///
/// # Errors
/// if `reqwest` cannot send the request to The Senpy API
pub fn status() -> reqwest::Result<bool> {
  Ok(
    reqwest::blocking::Client::new()
      .get(SENPY_CLUB_API_URL)
      .header("User-Agent", env!("GIT_COMMIT_HASH"))
      .send()?
      .status()
      == 200,
  )
}
