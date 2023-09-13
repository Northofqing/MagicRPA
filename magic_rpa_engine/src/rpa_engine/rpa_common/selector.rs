#[derive(Debug)]
pub struct SelectorProps {
    name: String,
    control_type: String,
    propertys: Vec<Props>,
    select: bool,
}
impl SelectorProps {
    pub fn new(
        _name: String,
        _control_type: String,
        _propertys: Vec<Props>,
        _select: bool,
    ) -> SelectorProps {
        Self {
            name: _name,
            control_type: _control_type,
            propertys: _propertys,
            select: _select,
        }
    }
}
#[derive(Debug)]
pub enum Pattern {
    //equal
    EQUAL,
    //contain
    CONTAIN,
    //regular
    REGULAR, //正则
}
#[derive(Debug)]
pub struct Props {
    name: String,
    value: String,
    select: bool,
    pattern: Pattern,
}
impl Props {
    pub fn new(_name: String, _value: String, _select: bool, _pattern: Pattern) -> Props {
        Self {
            name: _name,
            value: _value,
            select: _select,
            pattern: _pattern,
        }
    }
}
