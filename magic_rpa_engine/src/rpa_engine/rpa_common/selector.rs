#[warn(unused_imports)]
pub struct SelectorProps {
    name: String,
    element_type: String,
    propertys: Vec<Props>,
    select: bool,
}
#[warn(unused_imports)]
enum Pattern {
    //equal
    EQUAL,  
    //contain
    CONTAIN,
    //regular
    REGULAR, //正则
}
#[warn(unused_imports)]
pub struct Props
{
    name: String,
    value: String,
    select: bool,
    pattern: Pattern,
}
