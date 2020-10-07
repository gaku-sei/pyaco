module Printer (save) where

import Prelude
import AppM (AppM)
import Control.Monad.Reader (ask, asks)
import Data.Either (either)
import Data.Traversable (traverse)
import Effect.Exception (error)
import Lang (Lang(..))
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile, writeTextFile)
import Node.Path (FilePath)
import Parser (PseudoAst(..), parseFromFile)
import Text.Handlebars as Handlebars
import Utils (catchAndKill, kill)

formatFromFile :: FilePath -> AppM String
formatFromFile outputFile =
  asks _.cssOutput
    >>= parseFromFile
    >>= traverse formatFile
    >>= either (kill <<< error <<< show) pure
  where
  formatFile (PseudoAst nodes) = do
    template <- catchAndKill $ readTextFile UTF8 outputFile
    pure $ Handlebars.compile template { nodes }

-- FIXME: Normalize and resolve path
save :: AppM Unit
save = do
  { lang, output } <- ask
  case lang of
    PureScript -> formatFromFile "./templates/purs.hbs" >>= saveFile (output <> "/Tailwind.purs")
    Elm -> formatFromFile "./templates/elm.hbs" >>= saveFile (output <> "/Tailwind.elm")
    ReasonML -> do
      code <- formatFromFile "./templates/re.hbs"
      codei <- formatFromFile "./templates/rei.hbs"
      saveFile (output <> "/tailwind.re") code
      saveFile (output <> "/tailwind.rei") codei
    TypeScript -> formatFromFile "./templates/ts.hbs" >>= saveFile (output <> "/tailwind.ts")
    TypeScriptTypeLevel -> formatFromFile "./templates/ts-type-level.hbs" >>= saveFile (output <> "/tailwind.ts")
  where
  saveFile path = catchAndKill <<< writeTextFile UTF8 path
