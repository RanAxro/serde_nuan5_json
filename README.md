# serde_nuan5_json
### Serialization and deserialization of an Infinite Nikki type of JSON extended format

---

A new type is introduced on top of the JSON format, with the form  
` [: 10008: false, 20001: true] `  
It starts with `[:` and ends with `]`, containing several key-value pairs separated by commas, where the key is of **integer** type. This format is called **IdMap**, and this JSON extension format is called **nuan5json**.

for instance:
```json
{
  "EditPhotoHandler": {
    "editState": true,
    "hasSticker": false,
    "hasText": true
  },
  "interactive_photo": [:
    10008: false,
    20001: true
  ],
}
```

---

You can use `IdMap` just like `BTreeMap`.
```rust
let mut id_map = IdMap::new();
id_map.insert(10008, false);
```
It is defined as follows:
```rust
#[derive(Serialize, Deserialize)]
pub struct IdMap<T>(BTreeMap<i64, T>);

impl<T> IdMap<T>{
  pub fn new() -> Self{
    IdMap(BTreeMap::new())
  }
}

impl<T> Deref for IdMap<T>{
  type Target = BTreeMap<i64, T>;
  fn deref(&self) -> &Self::Target{
    &self.0
  }
}

impl<T> DerefMut for IdMap<T>{
  fn deref_mut(&mut self) -> &mut Self::Target{
    &mut self.0
  }
}
```

---

Additionally, I have defined the **AdaptiveArray** and **OptionMap** types, which have the following characteristics:

* **AdaptiveArray**:  
  For `Array<T>` with length `len`:
  - If `len = 0`, it is stored as `{}`.
  - If `len = 1` and the sole element is `data`:
    - If `T` is `Object`, it is stored as `[data]`.
    - Otherwise, it is stored as `data`.
  - If `len > 1`, it is stored directly as an `Array`.

* **OptionMap**:  
  When the content is not intended to be stored, an empty Object `{}` is stored.  

Defined as follows:
```rust
enum AdaptiveArray<T>{
  Array(Vec<T>),
  Item(T),
  Empty {},
}

enum OptionMap<T>{
  None {},
  Some(T),
}
```

---

Unlike `serde_json`, my library does not provide a `Value` type; therefore, you must define your data models before use.  
For example:
```rust

#[derive(Serialize, Deserialize)]
pub struct NikkiPhotoCustomData{
  #[serde(rename = "EditPhotoHandler")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub edit_photo_handler: Option<EditPhotoHandler>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub interactive_photo: Option<IdMap<bool>>,

  // ···
}
// ···
```
Of course, I have provided some models in `core/src/structs` for your convenience.

---

Several methods are provided for ease of use:

**Serialization:**
- `to_string`
- `to_string_pretty`

**Deserialization:**
- `from_str`