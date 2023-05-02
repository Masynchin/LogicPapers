---
# my settings
layout: center
hideInToc: true

aspectRatio: '4/3'
canvasWidth: 560

highlighter: prism
# show line numbers in code blocks
# lineNumbers: false
# page transition
transition: slide-left
---

# Немного о логическом программировании

Какой-нибудь бомбический подзаголовок

*Добавить Qr-Code со ссылкой на презентацию*

<!--
The last comment block of each slide will be treated as slide notes. It will be visible and editable in Presenter Mode along with the slide. [Read more in the docs](https://sli.dev/guide/syntax.html#notes)
-->

---
layout: center
hideInToc: true
---

# План

<Toc></Toc>

<!--
You can have `style` tag in markdown to override the style for the current page.
Learn more: https://sli.dev/guide/syntax#embedded-styles
-->

---

# О парадигмах

<!-- A programming paradigm is an approach to programming a computer based on a mathematical theory or a coherent set of principles. -->

Парадигма программирования это подход к программированию основанный на математической теории или другом согласованном наборе принципов.

<!-- Each paradigm supports a set of concepts that makes it the best for a certain kind of problem. -->

Каждая парадигма поддерживает набор концепций, позволяющие лучшим образом решать определённый вид задач.

---

Примеры парадигм программирования:

<div v-click>

- Процедурное программирование

<div class="flex gap-2">
  <svg width="40" height="40">
    <image href="https://cdn.svgporn.com/logos/c.svg" height="40" width="40" />
  </svg>
  <svg width="40" height="40">
    <image href="https://cdn.svgporn.com/logos/javascript.svg" height="40" width="40" />
  </svg>
  <svg width="32" height="40">
    <image href="https://cdn.svgporn.com/logos/gopher.svg" height="40" width="36" />
  </svg>
  <svg width="60" height="40">
    <image href="https://upload.wikimedia.org/wikipedia/commons/0/0f/Original_Ferris.svg" height="40" width="60" />
  </svg>
</div>

</div>

<br>

<div v-click>

```cpp
void main() {
    int res = 0;
    int n;
    for (i = 0; i < 4; i++) {
        cin >> n;
        res = res + n;
    }
    cout << res;
}
```

</div>

---

- Объектно-ориентированное программирование

<div class="flex gap-2">
  <svg width="40" height="40">
    <image href="https://cdn.svgporn.com/logos/java.svg" height="40" width="40" />
  </svg>
  <svg width="40" height="40">
    <image href="https://cdn.svgporn.com/logos/python.svg" height="40" width="40" />
  </svg>
  <svg width="40" height="40">
    <image href="https://cdn.svgporn.com/logos/scala.svg" height="40" width="40" />
  </svg>
  <svg width="40" height="40">
    <image href="https://cdn.svgporn.com/logos/ruby.svg" height="40" width="40" />
  </svg>
</div>

<br>

<div v-click>

```ruby
class Main
  puts Integers::from(STDIN).take(4).sum
end
```

</div>

---

- Функциональное программирование

<div class="flex gap-2">
  <svg width="40" height="40" xmlns="http://www.w3.org/2000/svg">
    <image href="https://cdn.svgporn.com/logos/haskell-icon.svg" height="40" width="40" />
  </svg>
  <svg width="40" height="40" xmlns="http://www.w3.org/2000/svg">
    <image href="https://cdn.svgporn.com/logos/scala.svg" height="40" width="40" />
  </svg>
  <svg width="40" height="40" xmlns="http://www.w3.org/2000/svg">
    <image href="https://cdn.svgporn.com/logos/clojure.svg" height="40" width="40" />
  </svg>
</div>

<div v-click>
  
<br>

```haskell
main =
  repeatM readline
  <#> mapMaybe toInt
  <#> take 4
  <#> sum
  >>= print
```

</div>

---

- Логическое программирование

<div class="flex gap-2">
  <svg width="40" height="40" xmlns="http://www.w3.org/2000/svg">
    <image href="https://api.iconify.design/material-symbols:question-mark.svg" height="40" width="40" />
  </svg>
  <svg width="40" height="40" xmlns="http://www.w3.org/2000/svg">
    <image href="https://api.iconify.design/material-symbols:question-mark.svg" height="40" width="40" />
  </svg>
  <svg width="40" height="40" xmlns="http://www.w3.org/2000/svg">
    <image href="https://api.iconify.design/material-symbols:question-mark.svg" height="40" width="40" />
  </svg>
</div>

---

# Prolog

<svg width="60" height="60">
  <image href="https://plugins.jetbrains.com/files/13954/290585/icon/pluginIcon.svg" height="60" width="60" />
</svg>

Один из первых (1972) языков логического программирования.

---

# Представление графа

<img src="/graph.png" width="200" />

*Проверить код*

*Разбить на два/три слайда*

*Переписать на crepe, если не работает*

```prolog {all|1-4|6-8|all}
edge(1, 2).
edge(1, 3).
edge(3, 4).
node(5).

node(X) :- edge(X, _).
node(X) :- edge(_, X).

% Рефлексивность
walk(X, X) :- node(X).

% Симметричность
walk(X, Y) :- edge(X, Y).
walk(X, Y) :- edge(Y, X).

% Транзитивность
walk(X, Y) :- walk(X, Z), walk(Z, Y).
% Изначально: walk(X, Y) :- edge(X, Z), walk(Z, Y).
```

```prolog
?- walk(1, 2).
true
```

Класс эквивалентности

```prolog
?- walk(5, X).
???

?- walk(1, X).
???

?- walk(2, X).
```

<div v-click>

```prolog
?- walk(2, 4).
true
```

</div>

<div v-click>

```prolog
?- walk(3, 5).
false
```

</div>

```prolog
?- walk(X, 4).
...
```

---

# Вариации

- [SWI Prolog](https://www.swi-prolog.org) - Prolog *вставить лого с сайта*
- [Datomic](https://github.com/Datomic) - Clojure (как реализация Datalog) *вставить лого из репоса*
- [Crepe](https://github.com/ekzhang/crepe) - Rust *вставить краба*

---

# Задача "Морской бой"

На вход подаётся множество координат клеток кораблей, и множество координат клеток выстрелов.

Вывести множества координат следующих клеток:

- Клетки с раненными кораблями
- Клетки промахов
- Клетки целыми кораблями
- Клетки с потопленными кораблями

---

Обозначим множество координат клеток кораблей так:

$\text{Ship} := \Set{ (x, y) \in \R^{2} }$

Аналогично обозначим множество координат клеток выстрелов:

$\text{Shot} := \Set{ (x, y) \in \R^{2} }$

---

## Раненные корабли

В клетке $(x, y)$ есть повреждённый корабль тогда и только тогда, когда в $(x, y)$ есть корабль и в $(x, y)$ был произведён выстрел.

Запишем это условие математически:

$\text{Hit} := \Set{ (x, y) | (x, y) \in \text{Ship}, (x, y) \in \text{Shot} }$

То есть, множество клеток повреждённых кораблей есть пересечение клеток кораблей и клеток, куда был произведён выстрел.

---

Наша математическая запись

$\text{Hit} := \Set{ (x, y) | (x, y) \in \text{Ship}, (x, y) \in \text{Shot} }$

На прологе будет выглядеть так:

```prolog
Hit(x, y) <- Ship(x, y), Shot(x, y);
```

Очень похоже, не так ли?

<!--
Presenter note with **bold**, *italic*, and ~~striked~~ text.

Also, HTML elements are valid:
<div class="flex w-full">
  <span style="flex-grow: 1;">Left content</span>
  <span>Right content</span>
</div>
-->

---

## Промахи

---

## Целые корабли

---

## Потопленные корабли

---

Всё!
