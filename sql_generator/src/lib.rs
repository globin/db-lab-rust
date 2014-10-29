#![feature(plugin_registrar)]
#![feature(slicing_syntax)]
#![feature(if_let)]
#![feature(quote)]
#![feature(phase)]

extern crate syntax;
extern crate rustc;
#[phase(plugin, link)] extern crate log;

use std::char;

use syntax::codemap::{Span, Spanned};
use syntax::parse::parser::Parser;
use syntax::parse::token::{mod, Token};
use syntax::ast::{mod, Ident, LitInt, LitStr, ItemStruct, Public, StructField_, NamedField, TokenTree};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacItems};
use rustc::plugin::Registry;

use state::ParserState;

mod state;

#[deriving(PartialEq, Show)]
enum Type {
    Integer,
    Numeric(uint, uint),
    Timestamp,
    Char(uint),
    Varchar(uint),
}
#[deriving(PartialEq)]
enum SqlKeyword {
    Primary,
    Key,
    Create,
    Table,
    Index,
    On,
    TypeKeyword(Type),
    Not,
    Null,
}
impl SqlKeyword {
    fn parse_keywords(tok: &Token) -> Option<SqlKeyword> {
        match token::to_string(tok)[] {
            "primary" => Some(Primary),
            "key" => Some(Key),
            "create" => Some(Create),
            "table" => Some(Table),
            "index" => Some(Index),
            "on" => Some(On),
            "integer" => Some(TypeKeyword(Integer)),
            "timestamp" => Some(TypeKeyword(Timestamp)),
            "numeric" => Some(TypeKeyword(Numeric(0, 0))),
            "not" => Some(Not),
            "null" => Some(Null),
            "char" => Some(TypeKeyword(Char(0))),
            "varchar" => Some(TypeKeyword(Varchar(0))),
            _ => None,
        }
    }

    fn is_keyword(tok: &Token, keyword: SqlKeyword) -> bool {
        SqlKeyword::parse_keywords(tok) == Some(keyword)
    }
}

trait SqlKeywordParser {
    fn is_sql_keyword(&self, keyword: SqlKeyword) -> bool;
    fn eat_sql_keyword(&mut self, keyword: SqlKeyword) -> bool;
    fn eat_sql_type_keyword(&mut self) -> Option<Type>;
}
impl<'a> SqlKeywordParser for Parser<'a> {
    fn is_sql_keyword(&self, keyword: SqlKeyword) -> bool {
        SqlKeyword::is_keyword(&self.token, keyword)
    }
    fn eat_sql_keyword(&mut self, keyword: SqlKeyword) -> bool {
        if self.is_sql_keyword(keyword) {
            self.bump();
            true
        } else {
            false
        }
    }
    fn eat_sql_type_keyword(&mut self) -> Option<Type> {
        match SqlKeyword::parse_keywords(&self.token) {
            Some(TypeKeyword(ty)) => {
                self.bump();
                Some(ty)
            }
            _ => None
        }
    }
}

#[deriving(Show)]
struct Column {
    name: Ident,
}
impl Column {
    fn new(name: Ident) -> Column {
        Column {
            name: name,
        }
    }
}

#[deriving(Show)]
struct Relation {
    name: Ident,
    columns: Vec<Column>,
}
impl Relation {
    fn new(name: Ident) -> Relation {
        Relation {
            name: name,
            columns: vec![],
        }
    }
}

#[deriving(Show)]
struct Schema {
    relations: Vec<Relation>,
}

fn parse_sql_schema(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {

    let parser = &mut cx.new_parser_from_tts(args);
    let mut state = state::Init;
    let schema = &mut Schema { relations: vec![] };

    while parser.token != token::EOF {
        if let state::Err(err) = state {
            cx.span_err(sp, "Could not parse SQL schema");
            cx.span_err(parser.span, err[]);
            return DummyResult::any(parser.span);
        }

        debug!("{}", parser.this_token_to_string());
        state = parse_next_token(parser, state, schema);
    }

    debug!("{}", schema);

    let structs = schema.relations.iter().map(|r| {
        let name = r.name;
        let mut strukt = (quote_item!(cx,
            #[deriving(Show)]
            struct $name;
        )).unwrap();
        strukt.map(|mut s| {
            match s.node {
                ItemStruct(ref mut s_def, _) => {
                    s_def.fields.push(Spanned {
                        node: StructField_ {
                            kind: NamedField(r.columns[0].name, Public),
                            id: ast::DUMMY_NODE_ID,
                            ty: quote_ty!(cx, uint),
                            attrs: vec![]
                        },
                        span: sp
                    })
                }
                _ => ()
            };
            s
        })
    });

    MacItems::new(structs)
}

fn parse_next_token(parser: &mut Parser, state: ParserState, schema: &mut Schema) -> ParserState {
    match state {
        state::Semicolon | state::Init if parser.is_sql_keyword(Create) => {
            parser.bump();
            state::Create
        }
        state::Create if parser.is_sql_keyword(Table) => {
            parser.bump();
            state::Table
        }
        state::Create if parser.is_sql_keyword(Index) => {
            parser.bump();
            state::Index
        }
        state::Table => {
            let name = match parser.bump_and_get() {
                token::IDENT(i, _) => format!("{}", token::get_ident(i)),
                ref tok@token::LIT_STR(_) => {
                    if let LitStr(s, _) = parser.lit_from_token(tok) {
                        format!("{}", s)
                    } else {
                        return err(parser, state);
                    }
                }
                _ => return err(parser, state)
            };
            let name = format!("{}{}", char::to_uppercase(name[].char_at(0)), name[1..]);
            schema.relations.push(Relation::new(token::str_to_ident(name[])));
            parser.bump();
            state::TableName
        }
        state::TableName => {
            parser.eat(&token::LPAREN);
            state::CreateTableBegin
        }
        state::CreateTableBegin | state::Separator => {
            if parser.eat(&token::RPAREN) {
                state::CreateTableEnd
            } else if parser.eat_sql_keyword(Primary) {
                state::Primary
            } else {
                schema.relations.last_mut().unwrap().columns.push(Column::new(parser.parse_ident()));
                state::AttributeName
            }
        }
        state::AttributeName => {
            match parser.eat_sql_type_keyword() {
                Some(ty) => {
                    debug!("colum_type: {}", ty);
                    state::AttributeType(ty)
                }
                None => err(parser, state)
            }
        }
        state::AttributeType(ty@Numeric(..)) | state::AttributeType(ty@Char(_)) |
            state::AttributeType(ty@Varchar(_)) => {
            if parser.eat(&token::LPAREN) {
                state::AttributeTypeArgs(ty)
            } else {
                err(parser, state)
            }
        }
        state::AttributeType(_) | state::AttributeTypeArgsEnd => {
            if parser.eat(&token::COMMA) {
                state::Separator
            } else if parser.eat_sql_keyword(Not) {
                state::Not
            } else if parser.eat(&token::RPAREN) {
                state::CreateTableEnd
            } else {
                err(parser, state)
            }
        }
        state::AttributeTypeArgs(ty) => {
            let tok = parser.bump_and_get();
            if let LitInt(n, _) = parser.lit_from_token(&tok) {
                debug!("char/varchar len = {}", n);
                if parser.eat(&token::RPAREN) {
                    return state::AttributeTypeArgsEnd;
                } else if parser.eat(&token::COMMA) {
                    return state::AttributeTypeArgs(ty)
                }
            }
            err(parser, state)
        }
        state::Not => {
            if parser.eat_sql_keyword(Null) {
                debug!("column is not null");
                state::Null
            } else {
                err(parser, state)
            }
        }
        state::Null => {
            if parser.eat(&token::COMMA) {
                state::Separator
            } else if parser.eat(&token::RPAREN) {
                state::CreateTableEnd
            } else {
                err(parser, state)
            }
        }
        state::CreateTableEnd if parser.token == token::SEMI => {
            parser.bump();
            state::Semicolon
        }
        state::Primary if parser.is_sql_keyword(Key) => {
            parser.bump();
            state::Key
        }
        state::Key => {
            if parser.eat(&token::LPAREN) {
                state::KeyListBegin
            } else {
                err(parser, state)
            }
        }
        state::KeyListBegin => {
            let ident = parser.parse_ident();
            debug!("table_ident: {}", ident);
            state::KeyName
        }
        state::KeyName => {
            if parser.eat(&token::COMMA) {
                state::KeyListBegin
            } else if parser.eat(&token::RPAREN) {
                state::KeyListEnd
            } else {
                err(parser, state)
            }
        }
        state::KeyListEnd => {
            if parser.eat(&token::COMMA) {
                state::Separator
            } else if parser.eat(&token::RPAREN) {
                state::CreateTableEnd
            } else {
                err(parser, state)
            }
        }
        _ => err(parser, state)
    }
}

fn err(parser: &mut Parser, state: ParserState) -> ParserState {
    state::Err(format!("Unexpected token '{}' at state '{}'",
                       parser.this_token_to_string(),
                       state))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("rn", parse_sql_schema);
}
