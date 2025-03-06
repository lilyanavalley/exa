
// This file is part of EXA.
// EXA is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as 
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// EXA is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with EXA. If not, see 
// <https://www.gnu.org/licenses/>.

//! Game components.
//! 
//! A 'Game Component' is either a game object or engine script for this game.
//! 
//! The contents include:
//! - [`dialog::Dialog`] component and [`dialog::DialogPoint`] script.
//! - [`fluent::FluentCache`] (aka. Localization) component.

/// Game/character dialog component.
pub mod dialog;

/// [Fluent](https://projectfluent.org/) implementation.
pub mod fluent;

/// Font collection manager.
pub mod fonts;
