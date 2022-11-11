# Binding Development Documentation

**我们以一个例子来说明binding的开发流程**

**stunning-wasm** **integer-sum 一个简单的求和案例**

## 从调用开始往上分析

### 这是一个调用宏，我们来看看他的参数

```
program!(
    entrypoint,
    vec![FieldType::NumericField, FieldType::NumericField]
);

```

第一个参数是 `entrypoint`，第二个参数是一个 `vec` 动态数组

`entrypoint` 定义有点长，不急我们一个个来看

```
fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {//首先传入一个类型为ValuePresenter的数组
    let first = inputs.get(0).unwrap(); /将传入数组分开
    let second = inputs.get(1).unwrap(); //获得两个值分别为first和second

    let sum: Number = add(extract_number(first), extract_number(second)); //在这里调用了add函数返回一个类型为Number的值

    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::NumericField(NumericFieldValue::Value(sum)),
    )]) //利用Outputs build包装后返回一个Outpust类型
}

```

现在我们来一个个解释上面的代码，首先第一个问题 `ValuePresenter` 类型是什么

```
pub enum ValuePresenter {
    Literal(LiteralValuePresenter), //一个自定义的枚举类型，但是又引入啦一个新的问题LiteralValuePresenter是什么
}
//下面就是LiteralValuePresenter，哇偶又是一个枚举，不慌我们继续
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValuePresenter {
    BooleanField(BooleanFieldValue),
    CascaderField(CascaderFieldValue),
    CheckboxField(CheckboxFieldValue),
    DateTimeField(DateTimeFieldValue),
    FileField(FileFieldValue),
    MultipleLineField(MultipleLineFieldValue),
    NumericField(NumericFieldValue), //我们的例子是加法，所以我们会用到这个
    RadioButtonField(RadioButtonFieldValue),
    RelationField(RelationFieldValue),
    SingleLineField(SingleLineFieldValue),
    TableRowField(TableRowFieldValue),
    UserBoundaryField(UserBoundaryFieldValue),

    // list field
    BooleanListField(BooleanListFieldValue),
    CascaderListField(CascaderListFieldValue),
    DateTimeListField(DateTimeListFieldValue),
    FileListField(FileListFieldValue),
    MultipleLineListField(MultipleLineListFieldValue),
    NumericListField(NumericListFieldValue),
    RelationListField(RelationListFieldValue),
    SingleLineListField(SingleLineListFieldValue),
    TableRowListField(TableRowListFieldValue),
}
//以我们用到的例子为例，我们继续往下挖，是不是有点兴奋呢
//oh my god! 又是一个枚举,但是我们离真相已经很近啦
pub enum NumericFieldValue {
    Value(Number),
    Nil,
}
//我们继续一点点就行
//Number的定义就是整数和浮点数
pub enum Number {
    Integer(i64),
    Float(f64),
}

```

好了接下来我们就该解释 sum 这个返回值啦，我们继续看代码

```
let sum: Number = add(extract_number(first), extract_number(second));

//我们一层一层的看，首先extract_number传入我们上面解释过的ValuePresenter，返回一个Number，Number的定义我们也看见啦
fn extract_number(value_presenter: &ValuePresenter) -> Number {
    match value_presenter {
        ValuePresenter::Literal(LiteralValuePresenter::NumericField(NumericFieldValue::Value(
            number,
        ))) => number.clone(),
        ValuePresenter::Literal(LiteralValuePresenter::NumericField(NumericFieldValue::Nil)) => {
            Number::Integer(0)
        }
        _ => unreachable!("Unexpected value presenter: {:?}", value_presenter),
    }
}

//现在我们在看 add 是不是很清楚，传入我们上面获取的两个类型为 Number 的值，因为 Number 有两个值，所以我们需要match进行匹配
fn add(a: Number, b: Number) -> Number {
    match (a, b) {
        (Number::Integer(first), Number::Integer(second)) => Number::Integer(first + second),
        (Number::Integer(first), Number::Float(second)) => Number::Float(first as f64 + second),
        (Number::Float(first), Number::Integer(second)) => Number::Float(first + second as f64),
        (Number::Float(first), Number::Float(second)) => Number::Float(first + second),
    }
}

```

我们已经基本理清楚啦基础逻辑，现在就是最后一点返回值 `Outputs`

```
 Outputs::build(vec![ValuePresenter::Literal(
     LiteralValuePresenter::NumericField(NumericFieldValue::Value(sum)),
 )]) //利用Outputs build包装后返回一个Outpust类型

```

第一步当然是看一下 `build`,原来是 `Outputs` 结构体的，`build` 是他实现的方法，但是传入的是 `vec` 数组

所以我们需要将他转化为 `vec` 类型，但是由于是 `ValuePresenter`，所以我们需要将他进行重新包装

```
pub struct Outputs(pub Vec<ValuePresenter>);

impl Outputs {
    pub fn build(value_presenters: Vec<ValuePresenter>) -> Self {
        Self(value_presenters)
    }

    pub fn to_json(&self) -> Value {
        Value::Array(self.0.iter().map(|vp| vp.to_json()).collect())
    }
}

```

现在我们应该清楚的知道我们的函数的作用啦

```
program!(
    entrypoint,//传入的是一个函数，返回值是outputs
    vec![FieldType::NumericField, FieldType::NumericField]//这个是不是很眼熟，
);
//没错就是这个，证明我们传入的参数是什么类型的
pub enum NumericFieldValue {
    Value(Number),
    Nil,
}

```

## 现在我们开始往下分析

```
#[macro_export]
macro_rules! program {
    ($entrypoint:ident, $types:expr) => {
        #[no_mangle]
        pub fn run(inputs: &str) {
            jet_programmable_rust_binding::wrap_run(inputs, $entrypoint, $types)
        }
    };
}

```

宏里面 调用了 `run` 这个函数，我们继续看看 `wrap_run`，他有三个参数，第一个就是 `run` 函数输入的字符串引用，第二个是我们传入的函数，第三个就是我们传入的类型

```
#[doc(hidden)]
pub fn wrap_run<F>(inputs: &str, entrypoint: F, types: Vec<FieldType>)
where
    F: Fn(Vec<ValuePresenter>) -> Outputs, //规定F是实现了这个
{
    let json: Value = match serde_json::from_str(inputs) { //将输入的值类型转化为Value
        Ok(json) => json,
        Err(err) => panic!("Failed to parse inputs: {}", err),
    };

    let outputs: Outputs = match parse(&json, types) {
        Ok(inputs) => entrypoint(inputs),
        Err(err) => panic!("Failed to decode inputs: {:?}", err),
    };

    let str = outputs.to_json().to_string();

    unsafe {
        hostcall_set_outputs(str.as_ptr(), str.len());
    }
}

```

我们先来看看第一个

```
let json: Value = match serde_json::from_str(inputs) {
        Ok(json) => json,
        Err(err) => panic!("Failed to parse inputs: {}", err),
 };
//下面是serde_json::from_str库的代码
pub fn from_str<'a, T>(s: &'a str) -> Result<T> //这里返回的是一个Result
where
    T: de::Deserialize<'a>,
{
    from_trait(read::StrRead::new(s))
}
//下面是库 Value 定义，我们删除掉部分注释，现在就是非常的清楚明了啦
pub enum Value {

    Null,
    Bool(bool),
    Number(Number),
    String(String),
    /// Represents a JSON array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(["an", "array"]);
    /// ```
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

```

下面的 `outputs` 调用了 `parse` 这个函数

```
    let outputs: Outputs = match parse(&json, types) {
        Ok(inputs) => entrypoint(inputs),
        Err(err) => panic!("Failed to decode inputs: {:?}", err),
    };

```

我们看看传入的两个参数，一个是 `json` 他的数据类型是 `Value`，另一个是 `type` `FieldType::NumericField` 返回值是一个 `Result`

```
pub fn parse(args: &Value, types: Vec<FieldType>) -> Result<Vec<ValuePresenter>, DecodeError> {
    match args {
        Value::Array(list) => { //匹配出args的值 list是一个 vec数组 类型 &Vec<Value>
            let types_len = types.len(); //获取types长度

            let pairs = types.into_iter().zip(list.iter());
            //上面这句的意思就是将她一一对应，类似变成了元组结构体
            //下面就是看看长度是不是相等
            if pairs.len() != types_len {
                panic!("Invalid number of inputs");
            }
            //创建一个可变的 result vec 数组值的类型为 ValuePresenter
            let mut result: Vec<ValuePresenter> = Vec::new();

            for (field_type, value) in pairs { //利用 for 循环拆开
                if value.is_object() {
                    match ValuePresenter::from_json(value) { //这个是枚举的方法
                        Ok(vp) => {
                            if vp.get_field_type() == field_type {
                                result.push(vp); //更新我们创建的 vec 数组将值添加进去
                            } else {
                                return Err(DecodeError::MismatchedFieldType {
                                    value_presenter: vp,
                                    field_type,
                                });
                            }
                        }
                        Err(error) => return Err(error),
                    }
                } else {
                    return Err(DecodeError::InvalidJsonObject(value));
                }
            }

            Ok(result) //所以这里本质返回的就是一个Vec<ValuePresenter>数组
        }
        value => Err(DecodeError::InvalidJsonObject(value)),
    }

}

```

上面的 `from_json` 的具体实现他是一个 `ValuePresenter` 枚举的实现方法,返回的是自己本身相当于做一个验证

```
pub fn from_json(json: &Value) -> Result<Self, DecodeError> {
        if !json.is_object() {
            return Err(DecodeError::InvalidJsonObject(json)); //判断是不是一个对象
        }

        match json.get("type") {
            Some(value) => match value {
                Value::String(ref type_name) => match type_name.as_str() {
                    "literal" | "LITERAL" => match LiteralValuePresenter::from_json(json) {
                        Ok(literal_vp) => Ok(ValuePresenter::Literal(literal_vp)),
                        Err(error) => Err(error),
                    },
                    _ => Err(DecodeError::UnsupportedType(json)),
                },
                _ => Err(DecodeError::UnsupportedType(json)),
            },
            None => Err(DecodeError::NoType),
        }
    }

```

我们继续往下看

将我们的返回值传入 `entrypoint` 这个我们穿进来的函数

```
let outputs: Outputs = match parse(&json, types) {
        Ok(inputs) => entrypoint(inputs),
        Err(err) => panic!("Failed to decode inputs: {:?}", err),
    };

```

这就是我们第一部分提到的会返回一个 `Outputs`

```
pub struct Outputs(pub Vec<ValuePresenter>);

```

继续往下走

```
let str = outputs.to_json().to_string();
//最后调用hostcall_set_outputs传入地址和长度
    unsafe {
        hostcall_set_outputs(str.as_ptr(), str.len());
    }

```

这个是 `to_json()` 方法，他是 `Outputs`结构体的方法，返回值是一个 `Value`，是我们上面提到的 `serde_json` 定义的 `Value`

Outpust 是一个 `vec` 数组

```
pub fn to_json(&self) -> Value {
        Value::Array(self.0.iter().map(|vp| vp.to_json()).collect())
    }

```

## 现在开始总结

我们现在大致的知道，`binding` 的库的作用。他会接受一个你编写完成的函数，这个函数的返回值一个个 `Outputs`，里面的类型 `ValuePresenter` 是属于自己自定义的类型，也就是说，我们在编写传入函数的时候所使用的类型的是 `binding` 封装的类型，并不是基础类型。同时我们也需要传入自己定义的参数在 `binding` 封装的是类型是什么，然后就会调用 `run` 函数输入参数，程序就是一个数据处理的过程。接下来就是 `binding` 利用传入的数据和你需要的类型，然后对其进行封装。传入函数处理数据。

然后我们看一下文件列表，进行分析

整个 `value_presenter` 都是对数据的定义和封装，包括实现的方法，我们在向上查询的时候起始已经对整个文件的进行了一个线性的查看。

`mod` 定义了总的数据结构 `ValuePresenter`，并编写了关键的方法，例如 `from_json`

继续深入就是 `literal_value_presenter.rs`. `LiteralValuePresenter` 定义了子面值的呈现并为其实现了相应的组件方法，`native` 和`list` 就是相当于最后一层封装，定义对应了方法

`value` 是每个类型的具体实现

```
├── Binding Development Documentation.md
├── Cargo.toml
├── README.md
└── src
    ├── hostcalls
    │   └── mod.rs
    ├── inputs.rs
    ├── lib.rs
    ├── outputs.rs
    ├── scaffolding.rs
    └── value_presenter
        ├── error.rs
        ├── field_type.rs
        ├── literal_list_value.rs
        ├── literal_naive_value.rs
        ├── literal_value.rs
        ├── literal_value_presenter.rs
        ├── mod.rs
        └── value
            ├── cascader_value.rs
            ├── file_object.rs
            ├── json_codec.rs
            ├── mod.rs
            ├── naive_date_time.rs
            ├── number.rs
            ├── options_value.rs
            ├── prosemirror.rs
            ├── relation_value.rs
            ├── user_boundary.rs
            └── uuid.rs

```
