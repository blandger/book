## Синтаксис шаблонов

В этой секции мы подробно расскажем о видах шаблонов и месте и способах их
использования.

### Литералы

Как мы уже видели в главе 6, вы можете использовать литералы непосредственно к
коде:

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

Будет напечатано `one`, т.к. `x` равен 1.

### Именованные переменные

Именованные переменные - это всеохватывающие шаблоны, которые принимают все
допустимые значения.

В коде 18-10 мы декларируем переменную `x` со значением `Some(5)` и переменную
`y` со значением `10`:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

<span class="caption">код 18-10: Выражение `match`, которое демонстрируют особенности
работы с включенными внутрь `Some()` значения переменной `y`</span>

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

Рассмотрим подробнее, что же происходит при работе этого кода. Первый шаблон рукава
`match` имеет значение `Some(50)` и текущее значение `x` не соответствует этому
значению. Во втором рукаве шаблон соответствует всем значениям Some.
Обратите внимание что в этому случае создаётся новая область видимости, в которой
включается новая переменная с именем `y`, которой присваивается значения внешней
переменной.

Если же `x` равно `None`, сработает шаблон `_`. В этом случае, будет напечатано
`Default case, x = None`.

Последняя строчка кода сообщит о текущем состоянии переменных `x` и `y`.

### Группа шаблонов

В выражение `match` благодаря логическому оператору `|` можно реализовать группу
условий, одном из которых должно удовлетворять условие. В результате будет выполнен
одни блок кода. Эта опция делает код компактнее::

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

Будет напечатано `one or two`.

### Использование `...`

Вы также можете использовать `...` для поиска значения входящего в числовой отрезок:

```rust
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

Если `x` равно 1, 2, 3, 4 или 5, будет напечатано "one through five".

Также в `match` можно использовать отрезки символов:

```rust
let x = 'c';

match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

Будет напечатано `early ASCII letter`.

### Деструктуризация для получения значений

Шаблоны могут быть использованы для деструктуризации структур, перечислений,
кортежей и ссылок. Деструктуризация значит получение значений из компонентов
группировочной структуры. Код 18-11 демонстрирует объявление структуры `Point`.
Структура содержит два поля `x` и `y`. С помощью `let` мы инициируем несколько
переменных одновременно:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

<span class="caption">код 18-11: инициализация переменных</span>

В этом коде создаются переменные `x` и `y`, которые соответствуют переменным `x`
и `y` в `p` по всем параметрам (имени, типу). Очень важно, чтобы имена переменных
совпадали. Если же вы хотите использовать другие имена переменных, это тоже можно
сделать, добавив описание полей структуры:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

<span class="caption">код 18-12: деструктуризация полей структуры в переменные</span>

Также мы можем одновременно тестировать и использовать поля структуры для инициализации
переменных:

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

<span class="caption">код 18-13: деструктуризация и тестирование на соответствие
значению шаблона</span>

Будет напечатано `On the y axis at 7`, т.к. `x = 0`.

Мы уже приводили пример деструктуризации в главе 6 в коде 6-5, где мы деструктурировали
тип данных `Option<i32>` используя выражение `match`, добавляя единицу внутреннему
значению `Some`.

Если значение, которые мы ищем в шаблоне содержит ссылку, мы можем использовать
`&` в шаблоне для разделения ссылки и значения. Это удобно в замыканиях используемых
итераторами. Код 18-14 демонстрирует, как реализовать цепочку вызовов, для того, чтобы
выполнить необходимые расчёты и получить результат:

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
];
let sum_of_squares: i32 = points
    .iter()
    .map(|&Point {x, y}| x * x + y * y)
    .sum();
```

<span class="caption">код 18-14: описания ссылки на структуру для доступа к полям
структуры</span>

`iter` обрабатывает ссылки на элементы последовательности. Если в коде вы не укажите
в описании шаблона перед названием структуры символ `&` мы получим ошибку типов
входных данных:

```text
error[E0308]: mismatched types
  -->
   |
14 |         .map(|Point {x, y}| x * x + y * y)
   |               ^^^^^^^^^^^^ expected &Point, found struct `Point`
   |
   = note: expected type `&Point`
              found type `Point`
```

Вы также можете смешивать, искать по шаблону и деструктировать ещё более сложным
образом. Например, так:

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

Эта конструкция разделяет комплексный тип на компоненты.

### Игнорирование значений в шаблоне

В грамматике шаблонов есть несколько элементов, которые могут быть использованы
для игнорирования значений или группы значений: `_`, имя с `_` и `..`. Рассмотрим
каждый такой шаблон подробнее.

#### Игнорирование всего значения с помощью шаблона `_`

Мы уже познакомились с использование `_` операторе `match`. Его также можно использовать,
как обозначение как способ игнорирования аргумента:

```rust
fn foo(_: i32) {
    // code goes here
}
```

<span class="caption">код 18-15: использование `_` в заголовке функции</span>

#### Игнорирование части значения `_`

Вы также можете использовать `_` для игнорирования части значения. Код 18-16,
в операторе `match` находит значение `Some` и игнорирует его содержимое внутри:

```rust
let x = Some(5);

match x {
    Some(_) => println!("got a Some and I don't care what's inside"),
    None => (),
}
```

<span class="caption">код 18-16: игнорирование содержания `Some`</span>

Это весьма удобно, когда просто нужно проигнорировать конкретное значение `Some`.

Мы также можем использовать `_` для игнорирования определённых значений в шаблоне:

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

<span class="caption">код 18-17: игнорирование элементы шаблона</span>

Будет напечатано `Some numbers: 2, 8, 32`.

#### Игнорирование неиспользуемых переменные, у которых в начале имени стоит символ `_`

Если необходим оставить в коде неиспользуемую переменную это можно сделать с помощью
символа `_`:

```rust
fn main() {
    let _x = 5;
    let y = 10;
    println!("{}", y);
}
```

<span class="caption">код 18-18: демонстрация игнорировании компилятором неиспользуемой
функции</span>

Обратите внимание на смысловой разнице использования символа `_`, как части имени
переменной и `_` в отдельности.

В коде 18_19 демонстрируется пример использования переменной с именем `s`.
В имени используется `_` для того, чтобы предотвратить многократное использование
переменной `s`:

```rust,ignore
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

<span class="caption">код 18-19: неиспользуемая переменная связывается  с переменой
`s` и получает владение</span>

Если используется `_`, код будет работать:

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

<span class="caption">код 18-20: использование `_`</span>

#### Игнорирование оставшихся частей значения с помощью `..`

С помощью значений вы можете с помощью шаблонов получить только небольшую часть
данных. Используя `..` вы можете получить отрезок данных. В примере 18-21 у вас
есть структура содержащая координаты (x,y,z). В `match` мы хотим получить только
данные `x`, а остальные данные проигнорировать:
```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

<span class="caption">код 18-21: игнорирование полей структуры `Point` кроме  `x`
с помощью `..`</span>

`..` это сокращённая запись `y: _` and `z: _`. Такая опция удобна при работе со структурой
 с большим количеством полей.

`..` также может быть использована для получения только нужных значений в кортеже:

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}
```

<span class="caption">код 18-22: выбор только первого и последнего значения кортежа</span>


В продемонстрированном примере мы получили значения первого и последнего элемента
кортежа.

Использование `..` должно быть однозначно и очевидно. Если компилятор не сможет
понять вашего замысла - будет ошибка:

```rust,ignore
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```

<span class="caption">код 18-23: попытка использовать `..` дважды</span>

Описание ошибки:

```text
error: `..` can only be used once per tuple or tuple struct pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |                      ^^
```

Т.к. невозможно определить сколько значений необходимо проигнорировать. Имя переменной
не несёт для компилятора какой-либо информации для выборки. Поэтому код не будет
скомпилирован.

### `ref` и `ref mut` для изменений ссылок в шаблонах

Обычно, когда значение совпадает с шаблоном последовательность работы программы
перемещается внутрь блока `match`, т.е. применяются правила владения. Код 18-24
демонстрирует пример:

```rust,ignore
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">код 18-24: создание переменной и попытка получить владение
ей</span>

Пример не скомпилируется, т.к. значение внутри `Some` будет перемещено `match`.

Воспользуемся символом `ref` в шаблоне для того сделать ссылку на полученное значение :

```rust
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">код 18-25: создание такой ссылки, что он не будет иметь
владение своим значением</span>

Этот код скомпилируется, т.к. мы не перемещаем значение внутрь блока.

Для создания изменяемой ссылки используйте `ref mut`:

```rust
let mut robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">код 18-26: создание изменяемой ссылки на значение</span>

Этот пример будет скомпилирован и в результате будет напечатано
`robot_name is: Some("Another name")`. Т.к. `name` является изменяемой ссылкой,
мы получим доступ к данным с помощью оператора `*`.

### Дополнительные условия с помощью защиты оператора `match`(Match Guards)

Вы можете представить т.н. дополнительную проверку (защиту) (*match guards*) рукава
оператора `match` с помощью введения дополнительного условия внутри оператором `if`.
Например, так:

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

<span class="caption">код 18-27: введение внутреннего дополнительного условия</span>


Будет напечатано `less than five: 4`. Если же `num` был бы увеличен `Some(7)`,
этот пример напечатал бы `7`. Дополнительная защита позволяет усложнить проверку,
добавив гибкости (при необходимости).

К примере кода 18-10, мы видели, что если шаблон скрывал переменные, мы не могли
создать шаблон для отслеживания случая, при котором это значение было равно какому-либо
значение извне `match`. Код 18-28 показывает реализовать эту задачу с помощью защиты
`match`:

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

<span class="caption">код 18-28: использование защиты match для дополнительного тестирования
значения</span>

Будет напечатано `Default case, x = Some(5)`. Т.к. второе условие не соблюдается
`x` != `y`, выполняется последние условие в `match`.

Если мы используем защиту match в нескольких шаблонах при помощи `|`, то условие
применяется ко всем шаблонам. Код 18-29 показывает это на примере:

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

<span class="caption">код 18-29: комбинация нескольких шаблонов с защитой match</span>

Будет напечатано `no`, т.к. условие в `if` применяется ко всему шаблону `4 | 5 |
6`, not only to the last value `6`. Другими словами, работает следующее условие:

```text
(4 | 5 | 6) if y => ...
```

а не это:

```text
4 | 5 | (6 if y) => ...
```

### Связывание `@`

Для того, чтобы проверить соответствие шаблону и создать переменную связанную с этим
значением мы можем использовать `@`. Код 18-30 демонстрирует пример, в котором
мы хотим проверить наличие поля  `Message::Hello` `id` в отрезке значений `3...7`,
а также связать найденное значение с переменной:

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id @ 3...7 } => {
        println!("Found an id in range: {}", id)
    },
    Message::Hello { id: 10...12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

<span class="caption">код 18-30: использование `@` для связывания значения в шаблоне
во время тестирования</span>

Будет напечатано `Found an id in range: 5`. Написав `id @` перед отрезком значений
мы проверяет входит ли значение в этот отрезок. Во втором рукаве проверяется условие.
Найденное значение не сохраняется. В последнем условии `match` мы только определили
переменную (она не будет учувствовать ни в каком сравнении). Использование `@`
добавляет гибкости и лаконичности в код ваших программ.

## Итоги

Шаблоны весьма полезны. При использовании оператора `match` компилятор должен
убедиться, что были проверены все возможные варианты совпадений. Оператор `let`
ми другие опции могут сделать этот `match` ещё более удобным в применении.

Настало время рассмотреть расширенные возможности Rust. В следующей главе будет
рассмотрены такие опции языка. Надеюсь, что будет интересно.