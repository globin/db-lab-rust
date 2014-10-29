use super::Type;

#[deriving(Show)]
pub enum ParserState {
    Init,
    Create,
    Table,
    CreateTableBegin,
    CreateTableEnd,
    TableName,
    Primary,
    Index,
    IndexName,
    IndexTableName,
    IndexColumns,
    IndexColumnBegin,
    IndexColumnName,
    IndexEnd,
    Key,
    KeyListBegin,
    KeyName,
    KeyListEnd,
    AttributeName,
    AttributeType(Type),
    AttributeTypeArgs(Type),
    AttributeTypeArgsEnd,
    AttributeEnd,
    NumericBegin,
    NumericValue1,
    NumericSeparator,
    NumericValue2,
    NumericEnd,
    Not,
    Null,
    Separator,
    Semicolon,
    Err(String)
}

