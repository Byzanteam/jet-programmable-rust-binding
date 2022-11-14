# Binding Development Documentation

**我们以一个例子来说明binding的开发流程**

**stunning-wasm integer-sum 一个简单的求和案例**

## integer-sum

入口函数及参数介绍

- program! rust宏 具有两个参数可以把它具象为一个函数
  
- entrypoint 定义传入参数，是一个函数
  
- vec![FieldType::NumericField, FieldType::NumericField] `vec!`动态数组
  

```rust
program!(
    entrypoint,
    vec![FieldType::NumericField, FieldType::NumericField]
);
```

## Program! 参数分析

rust宏 具有两个参数

- entrypoint
  
- vec![FieldType::NumericField, FieldType::NumericField]
  

### Entrypoint

- ValuePresenter
  
- extract_number 函数
  
- add 函数
  
- Outputs
  

总体概览，传入一个类型为`ValuePresenter`的`Vec`动态数组，返回值为`Outputs`

```rust
fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {// 首先传入一个类型为ValuePresenter的数组
    let first = inputs.get(0).unwrap(); // 将传入数组分开
    let second = inputs.get(1).unwrap(); // 获得两个值分别为first和second

    let sum: Number = add(extract_number(first), extract_number(second)); // 在这里调用了add函数返回一个类型为Number的值

    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::NumericField(NumericFieldValue::Value(sum)),
    )]) // 利用Outputs build包装后返回一个Outpust类型
}
```

#### Entrypoint 参数 ValuePresenter

ValuePresenter是一个自定义的枚举类型，类型为LiteralValuePresenter

```rust
pub enum ValuePresenter {
    Literal(LiteralValuePresenter), 
}
```

##### LiteralValuePresenter

LiteralValuePresenter也是自定义枚举类型，我们以使用的例子为准，继续深入了解。

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValuePresenter {
    BooleanField(BooleanFieldValue),
    CascaderField(CascaderFieldValue),
    CheckboxField(CheckboxFieldValue),
    DateTimeField(DateTimeFieldValue),
    FileField(FileFieldValue),
    MultipleLineField(MultipleLineFieldValue),
    NumericField(NumericFieldValue), // 我们的例子是加法，所以我们会用到这个
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
```

###### NumericFieldValue and Number

`NumericFieldValue` 是一个枚举，值为 `Value` 和 `Nil`

```rust
// 以我们用到的例子为例,继续深入
pub enum NumericFieldValue {
    Value(Number),
    Nil,
}
```

`Number` 定义，类型为一个枚举，值有两种

- `Integer` 类型为基础有符号整数类型，长度为64位
  
- `Float` 类型为基础浮点数类型，长度为64位
  

```rust
pub enum Number {
    Integer(i64),
    Float(f64),
}
```

#### extract_number 函数

extract_number传入我们上面解释过的 `ValuePresenter`，返回一个 `Number`

根据代码分析

函数目的是将 `ValuePresenter` 结构为 `Number` 这个由基础类型构成的枚举

```rust
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
```

#### add 函数

add函数接受我们 `extract_number` 函数返回的类型为 `Number` 的值当作入参参数

返回值同样也位 `Number`

```rust
fn add(a: Number, b: Number) -> Number {
    // match将 Number 类型解构为基础的数据类型
    match (a, b) {
        (Number::Integer(first), Number::Integer(second)) => Number::Integer(first + second),
        (Number::Integer(first), Number::Float(second)) => Number::Float(first as f64 + second),
        (Number::Float(first), Number::Integer(second)) => Number::Float(first + second as f64),
        (Number::Float(first), Number::Float(second)) => Number::Float(first + second),
    }
}
```

#### Entrypoint 返回值 Outputs

我们已经理清楚基础逻辑，现在就是最后一点返回值 `Outputs`

`Outputs `是结构体，`build` 是他实现的方法，返回值是

`Self` 也就是说返回值为 `Outputs` 结构体

但是传入的是 `vec` 数组，所以我们需要将他包装为 `vec` 类型，但是由于是 `ValuePresenter` 类型，所以我们需要将`Number` 重新封装为 `ValuePresenter`

```rust
 Outputs::build(vec![ValuePresenter::Literal(
     LiteralValuePresenter::NumericField(NumericFieldValue::Value(sum)),
 )]) // 利用Outputs build包装后返回一个Outpust类型
```

##### Outputs build

```rust
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

### vec![FieldType::NumericField, FieldType::NumericField]

参数就是我们上面分析过的 `NumericFieldValue` 这个枚举

```rust
pub enum NumericFieldValue {
    Value(Number),
    Nil,
}
```

### Program! 参数总结

```rust
program!(
    entrypoint,//传入的是一个函数，返回值是outputs
    vec![FieldType::NumericField, FieldType::NumericField]// 这个是不是很眼熟，
);
// 没错就是这个，证明我们传入的参数是什么类型的
pub enum NumericFieldValue {
    Value(Number),
    Nil,
}
```

## Program! 宏分析

整体架构就是一个宏列出了参数和函数

- entrypoint
  
- types
  
- run
  
- wrap_run
  

```rust
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

### wrap_run

有三个参数，第一个就是 `run` 函数输入的字符串引用，第二个是我们传入的函数，第三个就是我们传入的类型

```rust
#[doc(hidden)]
pub fn wrap_run<F>(inputs: &str, entrypoint: F, types: Vec<FieldType>)
where
    F: Fn(Vec<ValuePresenter>) -> Outputs, // 说明泛型F实现Fn(Vec<ValuePresenter>) -> Outputs这个特征
{
    let json: Value = match serde_json::from_str(inputs) { // 将输入的值类型转化为Value
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

#### from_str

```rust
let json: Value = match serde_json::from_str(inputs) {
        Ok(json) => json,
        Err(err) => panic!("Failed to parse inputs: {}", err),
 };
// 下面是serde_json::from_str库的代码
pub fn from_str<'a, T>(s: &'a str) -> Result<T> // 这里返回的是一个Result
where
    T: de::Deserialize<'a>,
{
    from_trait(read::StrRead::new(s))
} 
// 下面是库 Value 定义，我们删除掉部分注释，现在就是非常的清楚明了啦
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

#### parse

传入两个参数，一个是 `json` 他的数据类型是 `Value`，另一个是 `type` `FieldType::NumericField` 返回值是一个 `Result`

```rust
    let outputs: Outputs = match parse(&json, types) {
        Ok(inputs) => entrypoint(inputs),
        Err(err) => panic!("Failed to decode inputs: {:?}", err),
    };
```

```rust
pub fn parse(args: &Value, types: Vec<FieldType>) -> Result<Vec<ValuePresenter>, DecodeError> {
    match args {
        Value::Array(list) => { // 匹配出args的值 list是一个 vec数组 类型 &Vec<Value>
            let types_len = types.len(); // 获取types长度

            let pairs = types.into_iter().zip(list.iter());
            // 上面这句的意思就是将她一一对应，类似变成了元组结构体
            // 下面就是看看长度是不是相等
            if pairs.len() != types_len {
                panic!("Invalid number of inputs");
            }
            // 创建一个可变的 result vec 数组值的类型为 ValuePresenter
            let mut result: Vec<ValuePresenter> = Vec::new();

            for (field_type, value) in pairs { // 利用 for 循环拆开
                if value.is_object() {
                    match ValuePresenter::from_json(value) { // 这个是枚举的方法
                        Ok(vp) => {
                            if vp.get_field_type() == field_type {
                                result.push(vp); // 更新我们创建的 vec 数组将值添加进去
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

            Ok(result) // 所以这里本质返回的就是一个Vec<ValuePresenter>数组
        }
        value => Err(DecodeError::InvalidJsonObject(value)),
    }

}

```

上面的 `from_json` 的具体实现他是一个 `ValuePresenter` 枚举的实现方法,返回的是自己本身相当于做一个验证

```rust
pub fn from_json(json: &Value) -> Result<Self, DecodeError> {
        if !json.is_object() {
            return Err(DecodeError::InvalidJsonObject(json)); // 判断是不是一个对象
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

#### entrypoint(inputs)

```rust
let outputs: Outputs = match parse(&json, types) {
        Ok(inputs) => entrypoint(inputs),
        Err(err) => panic!("Failed to decode inputs: {:?}", err),
    };
```

这就是我们第一部分提到的会返回一个 `Outputs`

```rust
pub struct Outputs(pub Vec<ValuePresenter>);
```

#### 结尾

```rust
let str = outputs.to_json().to_string();
// 最后调用hostcall_set_outputs传入地址和长度
    unsafe {
        hostcall_set_outputs(str.as_ptr(), str.len());
    }
```

这个是 `to_json()` 方法，他是 `Outputs`结构体的方法，返回值是一个 `Value`，是我们上面提到的 `serde_json` 定义的 `Value`

Outpust 是一个 `vec` 数组

```rust
pub fn to_json(&self) -> Value {
        Value::Array(self.0.iter().map(|vp| vp.to_json()).collect())
    }
```

## Binding文件架构分析

我们现在大致的知道，`binding` 的库的作用。他会接受一个你编写完成的函数，这个函数的返回值一个个 `Outputs`，里面的类型 `ValuePresenter` 是属于自己自定义的类型，也就是说，我们在编写传入函数的时候所使用的类型的是 `binding` 封装的类型，并不是基础类型。同时我们也需要传入自己定义的参数在 `binding` 封装的是类型是什么，然后就会调用 `run` 函数输入参数，程序就是一个数据处理的过程。接下来就是 `binding` 利用传入的数据和你需要的类型，然后对其进行封装。传入函数处理数据。

然后我们看一下文件列表，进行分析

整个 `value_presenter` 都是对数据的定义和封装，包括实现的方法，我们在向上查询的时候起始已经对整个文件的进行了一个线性的查看。

`mod` 定义了总的数据结构 `ValuePresenter`，并编写了关键的方法，例如 `from_json`

继续深入就是 `literal_value_presenter.rs`. `LiteralValuePresenter` 定义了子面值的呈现并为其实现了相应的组件方法，`native` 和`list` 就是相当于最后一层封装，定义对应了方法

`value` 是每个类型的具体实现

```bash
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
