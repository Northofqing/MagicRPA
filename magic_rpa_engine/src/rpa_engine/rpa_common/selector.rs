#[warn(unused_imports)]
pub struct SelectorProps {
    name: String,
    element_type: String,
    propertys: Vec<Propertys>,
    select: bool,
}
enum Pattern {
    //equal
    EQUAL,  
    //contain
    CONTAIN,
    //regular
    REGULAR, //正则
}
pub struct Propertys
{
    name: String,
    value: String,
    select: bool,
    pattern: Pattern,
}
