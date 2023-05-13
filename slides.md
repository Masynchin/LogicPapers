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

Сссылка на презентацию:

<img src="/qrcode.png" width="100" />

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

# На чём можно писать?

Prolog:

- [SWI Prolog](https://www.swi-prolog.org)

Rust:

- [Crepe](https://github.com/ekzhang/crepe)
- [Cozo](https://github.com/cozodb/cozo)

Clojure:

- [Datomic](https://github.com/Datomic)
- [DataScript](https://github.com/tonsky/datascript)

---

# Правила

```prolog
Утверждение :- Условия.
```

<div v-click>

Примеры:

</div>

<div v-click>

Конъюнкция (И):

```prolog
Натуральное(X) :- Целое(X), БольшеНуля(X).
```

</div>

<div v-click>

Дизъюнкция (ИЛИ):

```prolog
НеНоль(X) :- БольшеНуля(X) ; МеньшеНуля(X).
```

</div>

<div v-click>

```prolog
НеНоль(X) :- БольшеНуля(X).
НеНоль(X) :- МеньшеНуля(X).
```

</div>

---

Отрицание (НЕ):

```prolog
НеНоль(X) :- !Ноль(X).
```

---

# Граф

<img src="/graph.png" width="200" />

Входные данные:

```prolog
Edge(1, 2).
Edge(1, 3).
Edge(3, 4).
Single(5).
```

---

- Все вершины

```prolog
Node(X) :- Single(X).
Node(X) :- Edge(X, _).
Node(X) :- Edge(_, X).
```

- Правила

```prolog
% Рефлексивность
CanWalk(X, X) :- Node(X).

% Симметричность
CanWalk(X, Y) :- Edge(X, Y).
CanWalk(X, Y) :- Edge(Y, X).

% Транзитивность
CanWalk(X, Y) :- CanWalk(X, Z), CanWalk(Z, Y).
```

---

```prolog
?- CanWalk(1, 2).
true
```

<div v-click>

<br/>

```prolog
?- CanWalk(2, 4).
true
```

</div>

<div v-click>

<br/>

```prolog
?- CanWalk(3, 5).
false
```

</div>

<div v-click>

<br/>

```prolog
?- CanWalk(X, 3).
X = 1 ;
X = 2 ;
X = 3 ;
X = 4
```

</div>

---

# Задача "Морской бой"

На вход подаются координаты клеток кораблей, и координаты клеток выстрелов.

Вывести координаты следующих клеток:

- Клетки с раненными кораблями
- Клетки промахов
- Клетки с целыми кораблями
- Клетки с потопленными кораблями

---
hideInToc: true
---

# Входные данные

<img src="/battleship.png" width="300" />

---
hideInToc: true
---

## Раненные корабли

```prolog
Hit(x, y) <- Ship(x, y), Shot(x, y).
```

<img src="/battleship_hits.png" width="300" />

---
hideInToc: true
---

## Промахи

```prolog
Miss(x, y) <- Shot(x, y), !Ship(x, y).
```

<img src="/battleship_misses.png" width="300" />

---
hideInToc: true
---

## Целые корабли

```prolog
Undamaged(x, y) <- Ship(x, y), !Shot(x, y).
```

<img src="/battleship_undamaged.png" width="300" />

---
hideInToc: true
---

## Потопленные корабли

---
hideInToc: true
---

## Объединим корабли 1

<div v-click>

```prolog
Related(x, y, x + 1, y) <- Ship(x, y), Ship(x + 1, y).
Related(x, y, x, y + 1) <- Ship(x, y), Ship(x, y + 1).
Related(x1, y1, x2, y2) <- Related(x2, y2, x1, y1).
```

<img src="/battleship_graph.png" width="300" />

</div>

---
hideInToc: true
---

## Объединим корабли 2

```prolog
Related(x1, y1, x2, y2) <-
    Related(x1, y1, a, b), Related(a, b, x2, y2).
```

<img src="/battleship_components.png" width="300" />

---
hideInToc: true
---

## Целые соседи

```prolog
RelatedUndamaged(x, y) <-
    Related(x, y, u, v), Undamaged(u, v).
```

<img src="/battleship_related_undamaged.png" width="300" />

---
hideInToc: true
---

## Потопленные корабли

```prolog
Sink(x, y) <- Hit(x, y), !RelatedUndamaged(x, y).
```

<img src="/battleship_sink.png" width="300" />

---
hideInToc: true
---

## Вся программа

```prolog
% Раненные клетки, промахи, целые клетки
Hit(x, y) <- Ship(x, y), Shot(x, y).
Miss(x, y) <- Shot(x, y), !Ship(x, y).
Undamaged(x, y) <- Ship(x, y), !Shot(x, y).

% Потопленные
Related(x, y, x + 1, y) <- Ship(x, y), Ship(x + 1, y).
Related(x, y, x, y + 1) <- Ship(x, y), Ship(x, y + 1).
Related(x1, y1, x2, y2) <- Related(x2, y2, x1, y1).
Related(x1, y1, x2, y2) <-
    Related(x1, y1, a, b), Related(a, b, x2, y2).

RelatedUndamaged(x, y) <-
    Related(x, y, u, v), Undamaged(u, v).

Sink(x, y) <- Hit(x, y), !RelatedUndamaged(x, y).
```

---
hideInToc: true
layout: center
---

## Спасибо за внимание!

<br/>

<img src="/qrcode.png" width="100" />
