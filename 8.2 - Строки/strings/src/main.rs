#![allow(unused_variables)]
fn main() {
    // Когда в Rust говорят о строках, то имеются в виду два типа строк:
    //
    //     - определённый в стандартной библиотеке тип String, который может расти в размере и т.д.
    //     - строковый срез типа &str, который встроен в сам язык.
    //
    // Оба эти типа хранят строки в формате UTF-8. Здесь мы в основном будем обсуждать строки типа
    // String. Внутренне String хранится в виде байт и дополнительно нескольких полезных методов
    // для работы с этими байтами, если они представляют из себя строку.

    // Создать пустую строку можно так:
    let s = String::new();

    // Создать строку, в которой уже что-то есть можно несколькими способами.
    // Такой мы уже видели:
    let s = String::from("Hello");

    // Если тип реализовывает типаж Display, то он поддерживает метод to_string. Строковые литералы
    // реализовывают типаж Display, поэтому создать инициализированную чем-либо строку можно и так:
    let data = "initial contents";
    let s = data.to_string(); // s имеет тип String.

    // Или же можно to_string вызвать сразу для литерала:
    let s = "initial contents".to_string(); // s имеет тип String.

    // Т.к. строки поддерживают UTF-8, то они могут хранить любые символы:
    let s = String::from("Привет");
    println!("{}", s);

    // Добавить строку к строке можно с помощью метода push_str, который принимате на вход только
    // строковый литерал(т.е. тип &str):
    let mut s = String::from("foo");
    s.push_str("bar");
    let excl_mark = "!";
    s.push_str(excl_mark);
    println!("s: {}", s);

    // Добавить один символ в конец строки можно так:
    let mut s = String::from("lo");
    s.push('l');

    // Объединить две строки можно с помощью '+':
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;

    // Нужно пользоваться именно 's1 + &s2', т.к. сложение строк на самом деле приводит к вызову
    // метода add для s1, а этом метод имеет следующую сигнатуру(если подставить конкретный тип
    // String в дженерик):
    //
    //     fn add(self, s: &str) -> String {

    // После сложения переменная s1 дальше уже не может быть использована, т.к. её значение было
    // перемещено в add.
    // Также из сигнатуры add видно, что она принимает на вход только &str, но мы ей передали
    // ссылку на String и всё равно код компилируется. Дело в том, что Rust может привести(coerce) тип
    // &String к типу &str. Для этого он использует так называемый deref coercion, который более
    // подробно будет рассмотрен в 15-й главе.

    // Также строки можно объединять с помощью макроса format!, который возвращает String:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    // Во многих языка программирования к отдельному символу строки можно обращаться по индексу в
    // квадратных скобках. В Rust такое нельзя делать:
    let s = String::from("hello");
    // Следующий код вызовет ошибку.
    // let h = s[0];

    // Эта ошибка связана с тем, как хранится String в памяти. Структура String является
    // надстройкой над Vec<u8> и хранит строку в закодированном согласно UTF-8 виде. В этой
    // кодировке символ необъязательно занимает один байт. Поэтому чтобы не возвращать части
    // символов Rust выдаёт ошибку компиляции.
    // Если мы точно знаем границы символов, то можно использовать срезы строк, чтобы получить
    // отдельный символ или группу символов:
    let s = "Кошка";
    let ka = &s[0..2];
    let ko = &s[0..4];
    println!("ka: {}, ko: {}", ka, ko);
    // Этот код работает, т.к. мы точно знаем, что русские буквы в UTF-8 занимают по 2 байта. Если
    // бы мы попытались получить срез вида &s[0..1], то во время выполнения была бы паника.

    // Итерировать по строкам можно по-разному. По каждому символу(Unicode scalar value) можно
    // пройтись c помощью такого цикла:
    for c in "мука".chars() {
        println!("{}", c);
    }
    // С помощью chars нельзя пройтись по так называемым кластерам графемов(grapheme clusters), из
    // которых тоже может состоять UTF-8 закодированная строка. Такой возможности нет в стандартной
    // библиотеке, но можно поискать крейты на "crates.io".

    // Каждый байт можно получить так:
    for c in "AB".bytes() {
        println!("{}", c);
    }
}
