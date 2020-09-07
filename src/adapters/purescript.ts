import fs from "fs";
import path from "path";

import { Adapter, Class } from "../types";

const fileName = "Tailwind.purs";

const generate = (classes: Class[]): string => {
  const purify = ({ className, name }: Class): string => `
${name} :: Tailwind
${name} =
  wrap "${className}"
`;

  return `module Tailwind (
  Tailwind,
  make,
${classes.map(cl => `  ${cl.name}`).join(",\n")}
  ) where

-- Generated by tailwind-generator, be careful when editing this file

-- Examples:
--
-- Simple list of tailwind classes:
--   [ rounded, borderRed100 ]
--
-- Add a class conditionally:
--   [ if true then textBlue500 else textRed500 ]
--
-- Add a class only if a condition is met, do nothing otherwise:
--   [ guard true textBlue500 ]
--
-- Handle Maybe, and other Foldable values:
--   [ rounded, fold $ Nothing ]
--   [ rounded, fold $ Right wFull ]
--
-- Build the class name string:
--   make [ rounder, borderRed100 ]

import Data.Foldable (foldl)
import Data.Newtype (class Newtype, wrap)
import Prelude (class Eq, class Ord, class Semigroup, class Monoid, otherwise, (==), (<>))

newtype Tailwind
  = Tailwind String

derive instance newtypeTailwind :: Newtype Tailwind _

derive instance eqTailwind :: Eq Tailwind

derive instance ordTailwind :: Ord Tailwind

derive newtype instance semigroupTailwind :: Semigroup Tailwind

derive newtype instance monoidTailwind :: Monoid Tailwind

make :: Array Tailwind -> String
make =
  foldl
    ( \\acc (Tailwind className) -> case className of
        "" -> acc
        c
          | acc == "" -> c
          | otherwise -> acc <> " " <> c
    )
    ""
${classes.map(purify).join("")}
  `;
};

export const save: Adapter["save"] = (dir, classes) => {
  fs.writeFileSync(path.join(dir, fileName), generate(classes));
};
