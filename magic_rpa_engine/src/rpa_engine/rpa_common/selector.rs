#[allow(dead_code)]
#[allow(unused_variables)]
pub struct SelectorProps {
    name: String,
    element_type: String,
    propertys: Vec<Props>,
    select: bool,
}
#[allow(dead_code)]
#[allow(unused_variables)]
enum Pattern {
    //equal
    EQUAL,  
    //contain
    CONTAIN,
    //regular
    REGULAR, //正则
}
#[allow(dead_code)]
#[allow(unused_variables)]
pub struct Props
{
    name: String,
    value: String,
    select: bool,
    pattern: Pattern,
}
