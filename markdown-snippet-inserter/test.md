---

title: Details über Rust Collections
marp: true
theme: rhea
color: "dark-gray"

---

<!-- 
footer: "Details über Rust Collections"
 -->
<!--
paginate: true
 -->
<!-- 
_footer: ''
_paginate: false
 -->
<!-- _class: lead -->

# Details über Rust Collections

<br>

### It's Iterators all the way down!

---

### Wie sind Algorithmen und Datenstrukturen in der Rust Standardbibliothek organisiert?

### Wie unterscheiden sich die Rust Container von denen in der C++ STL?

### Welche "Rusty" APIs bieten die Standardcollections an?

---

<!-- _class: lead -->

# Iteratoren und `#include<algorithm>`

## Wie kann man generische Operationen mit Collections ausführen?

---

# Grundlage: Iteratoren in Rust

````rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
````

Ziemlich viele Rust Konzepte in diesem Snippet!

* Assoziierter Typ `Item`: erlaubt es, den Element-Typ festzulegen
* Methode `next`: Nächstes Element, oder `None`

---

# Standard Iterationskonstruktoren

Jede Collection bietet Iterationsfunktionen an:

* `iter()`: iteriere mit shared borrows
* `iter_mut()`: iteriere mit unique, mutable borrows
* `into_iter()`: iteriere "by Value" (konsumiert die Datenstruktur)

````rust

````

---

# Spezielle Iterationskonstruktoren

Viele Collections bieten verschiedene Schnittstellen zum iterieren über Aspekte ihres Inhaltes an.
Ein Beispiel mit einer HashMap:

````rust
fn sum_values<K>(map: &HashMap<K, u32>) -> u32 {
    let mut sum = 0;
    for value in map.values() {
        sum += value;
    }
    sum
}
````

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3Acollections%3A%3AHashMap%3B%0A%0Afn+sum_values%3CK%3E%28map%3A+%26HashMap%3CK%2C+u32%3E%29+-%3E+u32+%7B%0A++++let+mut+sum+%3D+0%3B%0A++++for+value+in+map.values%28%29+%7B%0A++++++++sum+%2B%3D+value%3B%0A++++%7D%0A++++sum%0A%7D%0A%0Afn+sum_values_shorter%3CK%3E%28map%3A+%26HashMap%3CK%2C+u32%3E%29+-%3E+u32+%7B%0A++++map.values%28%29.sum%28%29%0A%7D%0A)" -->

---

# Kombinatoren für Iteratoren: `sum`

Auf Iteratoren sind gemeinsame Kombinatoren definiert, welche generische Operationen auf Collections definieren.

Ein sehr einfaches Beispiel:

````rust
fn sum_values<K>(map: &HashMap<K, u32>) -> u32 {
    map.values().sum()
}
````

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3Acollections%3A%3AHashMap%3B%0A%0Afn+sum_values%3CK%3E%28map%3A+%26HashMap%3CK%2C+u32%3E%29+-%3E+u32+%7B%0A++++let+mut+sum+%3D+0%3B%0A++++for+value+in+map.values%28%29+%7B%0A++++++++sum+%2B%3D+value%3B%0A++++%7D%0A++++sum%0A%7D%0A%0Afn+sum_values_shorter%3CK%3E%28map%3A+%26HashMap%3CK%2C+u32%3E%29+-%3E+u32+%7B%0A++++map.values%28%29.sum%28%29%0A%7D%0A)" -->

---

# Kombinatoren für Iteratoren: `all`

`all` wertet ein Prädikat für jeden Wert aus und ist wahr, wenn das Prädikat immer gilt.

````rust
assert!(name.chars().all(|c| c.is_ascii_alphanumeric()));
````

Gegenspieler: `any`

<!-- _footer: "[Protohackers Budget Chat](https://github.com/barafael/protohackers/blob/f1fe6cf0d6864661efd7d0014edc327ed523114d/budget_chat/src/main.rs#L56)" -->

---

# Kombinatoren für Iteratoren: `any`

`any` wertet ein Prädikat für jeden Wert aus, bis es gilt, und ist genau dann wahr.
Im Beispiel dürfen Tickets nur ausgestellt werden, wenn an diesem Tag noch kein Ticket ausgestellt wurde.

````rust

````

<!-- _footer: "[Protohackers Speedd](https://github.com/barafael/protohackers/blob/f1fe6cf0d6864661efd7d0014edc327ed523114d/speedd/src/collector.rs#L75)" -->

---

# Kombinatoren für Iteratoren: `map`

`map` ordnet die Elemente eines Iterators bijektiv auf einem neuen Iterator zu. Die übergebene Funktion bestimmt den Typ.

````rust

````

Hinweis: `parse_hex_digit` gibt ein `Result<T, E>` zurück. Also ist der Iterator danach ein `Iterator<Item = Result<u8, Error>>`.

<!-- _footer: "[Protohackers Netcrab](https://github.com/barafael/protohackers/blob/f1fe6cf0d6864661efd7d0014edc327ed523114d/netcrab/src/main.rs#L32)" -->

---

# Kombinatoren für Iteratoren: `filter`

Filter nimmt ein Prädikat und behält nur die Elemente des Iterators, für welche das Prädikat wahr ist.

````rust

````

einfacher:

````rust

````

````rust

````

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+core%3A%3Aiter%3A%3AFilter%3B%0Ause+std%3A%3Aslice%3A%3AIter%3B%0A%0Afn+main%28%29+%7B%0A++++let+a+%3D+%5B0i32%2C+-1%2C+2%2C+-3%2C+4%5D%3B%0A++++let+mut+iter%3A+Filter%3CIter%3C%27_%2C+i32%3E%2C+_%3E+%3D+a.iter%28%29.filter%28%7Cx%7C+x.is_positive%28%29%29%3B%0A++++%0A++++while+let+Some%28n%29+%3D+iter.next%28%29+%7B%0A++++++++dbg%21%28n%29%3B%0A++++%7D%0A%7D%0A)" -->

---

# Kombinatoren für Iteratoren: `collect`

`collect` erstellt eine Collection aus einem Iterator.
Hier ist häufig ein Turbofish oder eine Annotation nötig.

````rust

````

Die Typinferenz setzt hier den Freiheitsgrad von [`collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) fest.
Oft kann man so mit einem [`let`](https://doc.rust-lang.org/std/keyword.let.html) binding den Turbofish vermeiden.

---

# Kombinatoren für Iteratoren: `collect`

Manchmal braucht es einen Turbofish!

````rust

````

Hier setzt der Turbofish den Freiheitsgrad fest.

Das Ergebnis ist ein `Result<Vec<u8>, Error>`, weil man auch ein `Result<T, E>` als Collection sehen kann.

<!-- _footer: "[Protohackers Netcrab](https://github.com/barafael/protohackers/blob/f1fe6cf0d6864661efd7d0014edc327ed523114d/netcrab/src/main.rs#L32)" -->

---

# Collecting und [`Result<Collection, E>`](https://doc.rust-lang.org/std/result/)

Gegeben ein [`Result<T, E>`](https://doc.rust-lang.org/std/result/), wie kann man [`?`](https://doc.rust-lang.org/std/result/index.html#the-question-mark-operator-) zur Fehlerbehandlung einsetzen?

<p style = "text-align: center;">
<code>Iterator<Item = Result<T, E>> -> Result<Vec<T>, E></code>
</p>


````rust

````

---

# Collecting und [`Result<Collection, E>`](https://doc.rust-lang.org/std/result/)

Der [`FromIterator<A>`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) Trait wird genutzt um [`collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) zu implementieren.

````rust

````

Ganz schön kompliziert. Daumenregel: man kann [`collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) auf Iteratoren aufrufen, auch auf Iteratoren von `Result`.
Und man sollte den gewünschten Ausgabetyp festlegen.

---

# Collecting und [`try_join_all`](https://docs.rs/futures/latest/futures/future/fn.try_join_all.html)

Realistisches Beispiel mit Futures und Future-Kombinatoren:

````rust

````

Das ist ein sehr beliebtes Pattern.

<!-- _footer: "
[Achat: chat with cancellation](https://github.com/barafael/achat/blob/c8fa30d90b703b41993e04f53fe474070b0ab199/bin/chat_with_cancel.rs#L51)
" -->

---

# Weitere Kombinatoren

`map`, `filter`, `collect` sind nur der Anfang!

[Provided Methods vom Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html#provided-methods)

[Provided Methods von String Slices (`&str`)](https://doc.rust-lang.org/std/primitive.str.html)

Besondere/beliebte Kombinatoren: `chain`, `zip`, `cycle`, `take`, `windows`, `fold`, ...

Viele weitere in [itertools](https://github.com/rust-itertools/itertools): `windows`, `interleave`, `collect_vec`, `join`, `partition`, `peek_nth`

---

# Einschub: Parallele Iteratoren

Die Iteratoren sind eh schon Threadsafe! Also: work stealing.

````rust

````

<!-- _footer: "[Julia Set Renderer mit Rayon](https://github.com/cocomundo/julia-set-renderer/blob/b88241ba482c0af1269a990ad3184d47179e7144/src/lib.rs#L42)" -->

---

# Iteratoren jenseits von Collections

Auch viele andere Funktionen geben einen Iterator zurück, selbst wenn es keine Collection dahinter gibt:

* `std::env::args()`: Argumente des Programmes
* `std::str::matches()`: Matches eines Patterns in einem String

Man kann die Schnittstelle auch sonst vielfältig nutzen, zum Beispiel um [Fibonacci-Sequenzen](https://doc.rust-lang.org/rust-by-example/trait/iter.html) zu erzeugen.

---

# Die HashMap Entry API

Die Entry API ist der idiomatische und ergonomische Weg, HashMap-Einträge zu bearbeiten.
Dabei werden die Einträge der Map in-place editiert!
Ein Entry:

````rust
pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}
````

Mehr Details auf "[A Rust Gem - The Rust Map API](https://www.thecodedmessage.com/posts/rust-map-entry/)"

---

# Beispiel: `or_insert`

Oft will man auf existierenden Elementen operieren, oder falls es keine gibt, einen Anfangswert einfügen:

````rust

````

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3Acollections%3A%3AHashMap%3B%0A%0Afn+main%28%29+%7B%0A++++let+mut+counts%3A+HashMap%3C%26str%2C+usize%3E+%3D+HashMap%3A%3Anew%28%29%3B%0A++++for+name+in+%5B%22a%22%2C+%22b%22%2C+%22c%22%2C+%22a%22%2C+%22a%22%5D+%7B%0A++++++++*counts.entry%28name%29.or_insert%280%29+%2B%3D+1%3B%0A++++%7D%0A++++dbg%21%28counts%29%3B%0A%7D%0A)" -->

---

# Beispiel: `or_default`

````rust

````

`or_default` gibt ein `&mut V`, also einen mutable borrow auf den Wert in der HashMap.

Wo kommt der default-Wert her? Natürlich vom `Default` Trait!

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3Acollections%3A%3AHashMap%3B%0A%0Afn+main%28%29+%7B%0A++++let+mut+counts%3A+HashMap%3C%26str%2C+usize%3E+%3D+HashMap%3A%3Anew%28%29%3B%0A++++for+name+in+%5B%22a%22%2C+%22b%22%2C+%22c%22%2C+%22a%22%2C+%22a%22%5D+%7B%0A++++++++*counts.entry%28name%29.or_default%28%29+%2B%3D+1%3B%0A++++%7D%0A++++dbg%21%28counts%29%3B%0A%7D%0A)" -->

---

# Beispiel: `and_modify`

Viele der [Usages](https://github.com/search?q=%22and_modify%22+language%3ARust&type=code&ref=advsearch) lassen sich auch als Kombination von `or_insert(.)` oder `or_default`.

````rust

````

Äquivalent:

````rust

````

<!-- _footer: "[facebook/hhvm](https://github.com/facebook/hhvm/blob/c01bc30d5883ffdf08329111fa709ed9da815ad5/hphp/hack/src/hackc/ir/conversions/ir_to_bc/emitter.rs#L171)" -->

---

<!-- _class: lead -->

# Static Table Patterns

---

# Perfect Hash Function

Die [phf](https://github.com/rust-phf/rust-phf) Crate erstellt statische Tabellen zur Compilezeit und ermöglicht so das Lookup mithilfe einer "Perfekten Hashfunktion".

````rust
static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "loop" => Keyword::Loop,
    "continue" => Keyword::Continue,
    "break" => Keyword::Break,
    "fn" => Keyword::Fn,
    "extern" => Keyword::Extern,
};
````

<!-- _footer: "[Offizielles Beispiel](https://docs.rs/phf/latest/phf/index.html#example-with-the-macros-feature-enabled)" -->

---

# Vor- und Nachteile von PHF

* unschlagbar schnell
* unterstützt auch ordered und unordered sets

Aber:

* Die Keys müssen Literale sein (integer, characters, oder strings)
* Die Tabelle und ihre Inhalte sind vollkommen statisch
* Sowohl die Makro-API als auch der Codegenerator im `build.rs` sind nicht besonders idiomatisch

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+phf%3A%3Aphf_map%3B%0A%0A%23%5Bderive%28Clone%29%5D%0Apub+enum+Keyword+%7B%0A++++Loop%2C%0A++++Continue%2C%0A++++Break%2C%0A++++Fn%2C%0A++++Extern%2C%0A%7D%0A%0Astatic+KEYWORDS%3A+phf%3A%3AMap%3C%26%27static+str%2C+Keyword%3E+%3D+phf_map%21+%7B%0A++++%22loop%22+%3D%3E+Keyword%3A%3ALoop%2C%0A++++%22continue%22+%3D%3E+Keyword%3A%3AContinue%2C%0A++++%22break%22+%3D%3E+Keyword%3A%3ABhttps://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3A%7Bsync%3A%3AMutex%2C+collections%3A%3AHashMap%7D%3B%0Ause+once_cell%3A%3Async%3A%3ALazy%3B%0A%0Astatic+GLOBAL_DATA%3A+Lazy%3CMutex%3CHashMap%3Ci32%2C+String%3E%3E%3E+%3D+Lazy%3A%3Anew%28%7C%7C+%7B%0A++++let+mut+m+%3D+HashMap%3A%3Anew%28%29%3B%0A++++m.insert%2813%2C+%22Spica%22.to_string%28%29%29%3B%0A++++m.insert%2874%2C+%22Hoyten%22.to_string%28%29%29%3B%0A++++Mutex%3A%3Anew%28m%29%0A%7D%29%3B%0A%0Afn+main%28%29+%7B%0A++++println%21%28%22%7B%3A%3F%7D%22%2C+GLOBAL_DATA.lock%28%29.unwrap%28%29%29%3B%0A%7Dreak%2C%0A++++%22fn%22+%3D%3E+Keyword%3A%3AFn%2C%0A++++%22extern%22+%3D%3E+Keyword%3A%3AExtern%2C%0A%7D%3B%0A%0Apub+fn+parse_keyword%28keyword%3A+%26str%29+-%3E+Option%3CKeyword%3E+%7B%0A++++KEYWORDS.get%28keyword%29.cloned%28%29%0A%7D%0A)" -->

---

# Lazy Static Mutable Map

Ein Mutex schützt eine globale HashMap, welche beim ersten Abruf (lazy) populiert wird:

````rust
static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});
````

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3A%7Bsync%3A%3AMutex%2C+collections%3A%3AHashMap%7D%3B%0Ause+once_cell%3A%3Async%3A%3ALazy%3B%0A%0Astatic+GLOBAL_DATA%3A+Lazy%3CMutex%3CHashMap%3Ci32%2C+String%3E%3E%3E+%3D+Lazy%3A%3Anew%28%7C%7C+%7B%0A++++let+mut+m+%3D+HashMap%3A%3Anew%28%29%3B%0A++++m.insert%2813%2C+%22Spica%22.to_string%28%29%29%3B%0A++++m.insert%2874%2C+%22Hoyten%22.to_string%28%29%29%3B%0A++++Mutex%3A%3Anew%28m%29%0A%7D%29%3B%0A%0Afn+main%28%29+%7B%0A++++println%21%28%22%7B%3A%3F%7D%22%2C+GLOBAL_DATA.lock%28%29.unwrap%28%29%29%3B%0A%7D)" -->

---

# Vor- und Nachteile LSMM

* Konzeptionell einfach zu verstehen
* Thread Safe trotz Global Mutable State
* HashMap ist relativ flexibel und hat eine riesige gute API

Aber:

* Mutex
* Initiale Konstruktion der HashMap muss ohne Argumente auskommen

---

# Rückschau

* Details über Iteratoren
* Kombinatoren für Iteratoren
* Die HashMap Entry API
* Static Table Patterns