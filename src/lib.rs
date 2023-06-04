mod class;
mod enum_member;
mod enumeration;
mod property;
pub use class::*;
pub use enum_member::*;
pub use enumeration::*;
pub use property::*;

pub enum Type {
    Class(Class),
    Enum(Enumeration),
    DataType(DataType),
}
pub enum DataType {
    Boolean,
    Integer,
    Date,
    DateTime,
    Number,
    Text,
    Time,
    CssSelectorType,
    PronounceableText,
    URL,
    XPathType,
    Float,
    DataType,
}

impl DataType {
    pub fn range_include(&self) -> Option<Vec<DataType>> {
        match self {
            DataType::Number => Some(vec![DataType::Integer, DataType::Float]),
            DataType::DataType => Some(vec![
                DataType::Boolean,
                DataType::Integer,
                DataType::Date,
                DataType::DateTime,
                DataType::Number,
                DataType::Text,
                DataType::Time,
                DataType::CssSelectorType,
                DataType::PronounceableText,
                DataType::URL,
                DataType::XPathType,
                DataType::Float,
            ]),
            DataType::Text => Some(vec![
                DataType::CssSelectorType,
                DataType::PronounceableText,
                DataType::URL,
                DataType::XPathType,
            ]),
            DataType::Boolean
            | DataType::Integer
            | DataType::Date
            | DataType::DateTime
            | DataType::Time
            | DataType::CssSelectorType
            | DataType::PronounceableText
            | DataType::URL
            | DataType::XPathType
            | DataType::Float => None,
        }
    }
}
pub trait HasChild {
    fn has_child(&self) -> bool;
}
impl HasChild for DataType {
    fn has_child(&self) -> bool {
        match self {
            DataType::Number | DataType::DataType | DataType::Text => true,
            DataType::Boolean
            | DataType::Integer
            | DataType::Date
            | DataType::DateTime
            | DataType::Time
            | DataType::CssSelectorType
            | DataType::PronounceableText
            | DataType::URL
            | DataType::XPathType
            | DataType::Float => false,
        }
    }
}
