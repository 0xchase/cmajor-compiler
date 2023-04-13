module Lib
    ( someFunc
    ) where

import Data.Text ( Text )
import Data.Char ( chr )

someFunc :: IO ()
someFunc = putStrLn "someFunc"

data Op = Add
    | Sub
    | Mult
    | Div
    deriving (Show, Eq)

data Uop = Neg
    | Not
    deriving (Show, Eq)

data Expr = Literal Int
    | Strlit Text
    | CharLit Int
    | Fliteral Double
    | BoolLit Bool
    deriving (Show, Eq)

data Statement = Expr Expr
    | Block [Statement]
    | Return Expr
    | If Expr Statement Statement
    | For Expr Expr Expr Statement
    | While Expr Statement
    deriving (Show, Eq)

data Type = Pointer Type
    | TyInt
    | TyBool
    | TyFloat
    | TyChar
    deriving (Show, Eq)

