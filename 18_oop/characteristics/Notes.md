## Chapter 18 - Object-Oriented Programming Features: Characteristics of Object-Oriented Languages

### Overview

There is **no consensus** in the programming community about what features a language must have to be considered *object oriented*. 

Arguably, **OOP** languages share certain common characteristics—namely, objects, encapsulation, and inheritance.

Let’s look at what each of those characteristics means and whether Rust supports it.

### Objects Contain Data and Behavior

*Design Patterns: Elements of Reusable Object-Oriented Software* aka *Gang of Four book*, defines **OOP** in the following way:

```
Object-oriented programs are made up of objects. 

An object packages both data and the procedures that operate on that data. 

The procedures are typically called methods or operations.
```

Using this definition, Rust is *object oriented*: **Structs** and **enums** have data, and **impl blocks** provide methods on structs and enums.

### Encapsulation That Hides Implementation Details

Another aspect commonly associated with OOP is the idea of **encapsulation**

**encapsulation**: the implementation details of an object aren’t accessible to code using that object. 

Therefore, the only way to interact with an object is through its **public API**.

Code using the object *shouldn’t* be able to reach into the object’s internals and change data or behavior directly. 

This enables the programmer to change and refactor an object’s internals *without* needing to change the code that uses the object.

In rust this is done via the keyword `pub` as we can then choose which modules, types, etc are exposed as public.

For example, we can define a struct `AveragedCollection` that has a field containing a `vector` of `i32` values. 

The struct can also have a field that contains the *average* of the values in the `vector`, meaning the average *doesn’t* have to be computed on demand whenever anyone needs it.

We can define such a struct like this:

```rs
pub struct AveragedCollection {
   list: Vec<i32>,
   average: f64,
}
```

The struct is marked `pub` so that other code can use it, but the fields within the struct *remain private*.

This is important in this case because we want to *ensure* that whenever a value is added or removed from the list, the average is also *updated*.

We do this by implementing `add`, `remove`, and `average` methods on the struct:

```rs
impl AveragedCollection {
   pub fn add(&mut self, value: i32) {
      self.list.push(value);
      self.update_average();
   }

   pub fn remove(&mut self) -> Option<i32> {
      let result = self.list.pop();
      match result {
         Some(value) => {
               self.update_average();
               Some(value)
         }
         None => None,
      }
   }

   pub fn average(&self) -> f64 {
      self.average
   }

   fn update_average(&mut self) {
      let total: i32 = self.list.iter().sum();
      self.average = total as f64 / self.list.len() as f64;
   }
}
```

The public methods `add`, `remove`, and `average` are the *only* ways to access or modify data in an instance of `AveragedCollection`.

The `average` method returns the value in the average field, allowing external code to *read* the average but *not modify it*.

Because we’ve *encapsulated* the implementation details of the struct `AveragedCollection`, we can easily change aspects, such as the data structure, in the future.

If we made `list` public instead, this wouldn’t necessarily be the case: `HashSet<i32>` and `Vec<i32>` have different methods for adding and removing items.

As such: the external code would likely have to change if it were modifying list directly.

If **encapsulation** is a required aspect for a language to be considered object oriented, then **Rust meets that requirement**.


### 