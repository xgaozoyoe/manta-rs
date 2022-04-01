// Copyright 2019-2022 Manta Network.
// This file is part of manta-rs.
//
// manta-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// manta-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with manta-rs.  If not, see <http://www.gnu.org/licenses/>.

//! Hash Functions

use crate::constraint::Native;

/// Hash Function
pub trait HashFunction<COM, const ARITY: usize> {
    /// Input Type
    type Input: ?Sized;

    /// Output Type
    type Output;

    /// Computes the hash over `input` in the given `compiler`.
    fn hash_in(&self, input: [&Self::Input; ARITY], compiler: &mut COM) -> Self::Output;

    /// Computes the hash over `input`.
    #[inline]
    fn hash(&self, input: [&Self::Input; ARITY]) -> Self::Output
    where
        COM: Native,
    {
        self.hash_in(input, &mut COM::compiler())
    }
}

/// Unary Hash Function
pub trait UnaryHashFunction<COM = ()> {
    /// Input Type
    type Input: ?Sized;

    /// Output Type
    type Output;

    /// Computes the hash over `input` in the given `compiler`.
    fn hash_in(&self, input: &Self::Input, compiler: &mut COM) -> Self::Output;

    /// Computes the hash over `input`.
    #[inline]
    fn hash(&self, input: &Self::Input) -> Self::Output
    where
        COM: Native,
    {
        self.hash_in(input, &mut COM::compiler())
    }
}

/// Binary Hash Function
pub trait BinaryHashFunction<COM = ()> {
    /// Left Input Type
    type Left: ?Sized;

    /// Right Input Type
    type Right: ?Sized;

    /// Output Type
    type Output;

    /// Computes the hash over `lhs` and `rhs` in the given `compiler`.
    fn hash_in(&self, lhs: &Self::Left, rhs: &Self::Right, compiler: &mut COM) -> Self::Output;

    /// Computes the hash over `lhs` and `rhs`.
    #[inline]
    fn hash(&self, lhs: &Self::Left, rhs: &Self::Right) -> Self::Output
    where
        COM: Native,
    {
        self.hash_in(lhs, rhs, &mut COM::compiler())
    }
}