use typst::syntax::Source;
use typst_languagetool_preprocessor::preprocess;

#[test]
fn test() {
    let source = Source::detached(
        r#"#import "@preview/cetz:0.2.2"

#let gap = {
  linebreak()
  linebreak()
}

#set page(header: [
  #set text(8pt, style: "italic")
  SWEN304 Assignment 2
  #h(1fr) 2024
  #h(1fr) Trent Shailer - 300602354
], footer: [
  #set align(right)
  #set text(8pt, style: "italic")
  Page
  #counter(page).display("1 of 1", both: true)
])

#set raw(tab-size: 4)
#set heading(numbering: "1.a.1.")
#set par(justify: true)

= Relational Algebra
== Translate the following queries into Relational Algebra:
===
$pi_"Name" ("Suppliers"-(sigma_("Items.Category" != "'bread'") ("Suppliers" join "Supplied_By" join "Items")))$

===
$pi_"Description" (sigma_("count">=2) (pi_"count, Description" (""_"SId"gamma_("count(*)") ("Items" join "Supplied_By"))))$
// TODO double check grouping and count

===
$pi_"Name" ("Suppliers" - (sigma_("Supplied_By.Year"=2024) ("Suppliers" join "Supplied_By")))$

===
$pi_"Description" (sigma_("Location"="'Napier'") ("Items" join "Supplied_By" join ("Suppliers" - (sigma_("Price">100) ("Suppliers" join "Supplied_By")))))$

== Translate the following queries into plain English and into SQL
===
```sql
select Name, Phone
from Items
where Amount > 1000
natural join Suppliers
natural join Supplied_By;
```

Find the names and phone numbers of the suppliers who have supplied more than
1000 of any item.

===
```sql
select Description, Name
from Items
where price < 10
natural join Supplied_By
natural join Suppliers;
```

Find the description of the item and the name of the supplier who have supplied
the item for a price less than \$10.

===
```sql
select SId
from Supplied_By
where Amount > 1000
inner join (
  Select SId
  from Supplied_By
  where Description = 'Carrot Cake'
  natural join Items
) using (SId);
```

Find the supplier ID of the suppliers who have supplied more than 1000 carrot
cakes in a year.

= Heuristic and Cost-Based Query Optimization
== Heuristic query optimization
===
$pi_"StudentId, Name, Grade" (sigma_("CourName"="'Database Systems'" and "Term"=2024 " " and "Tutor"="'Tom'") ("Student" join "Enrolled" join "Course"))$

===
#cetz.canvas({
  import cetz.tree: tree
  import cetz.draw: set-style

  set-style(content: (padding: .2))

  tree(spread: 5, grow: 2, ($pi_("StudentId, Name, Tutor")$, (
    $join_("S.StudentId"="E.StudentId")$,
    ($join_("E.CourseId"="C.CourseId")$, (
      ($pi_("CourseId")$),
      ($sigma_("ClassRep" = "30001234")$, ($"Course"$)),
    ), (
      ($pi_("StudentId","CourseId")$),
      ($sigma_("Grade" = "'A'")$, ($"Enrolled"$)),
    ),),
    (
      ($pi_("StudentId","Name", "Tutor")$),
      ($sigma_("NoOfPts">160)$, ($"Student"$)),
    ),
  ),))
})

*Rules Applied:*
// Cartesian product followed by a select according to a join condition with a join operator (Rule 12)
// switching Course and Student  restrictive select operation could be applied as early as possible (Rule 9)

- Commuting #sym.sigma with #sym.join (Rule 6)
- Commuting #sym.pi with #sym.join (Rule 7)// TODO applying project (p) operations as early as possible (Rule 7)
- Associativity of #sym.join, #sym.times, #sym.union, and #sym.sect (Rule 9)
- Converting a (#sym.sigma, #sym.times) into #sym.join (Rule 12)

#set heading(numbering: "1.a.i.")
== Query cost calculation
===
#cetz.canvas({
  import cetz.tree: tree
  import cetz.draw: set-style

  set-style(content: (padding: .2))

  tree(spread: 4, grow: 2, ($pi_("StudentId, Name, Grade")$, (
    $sigma_("term"=2024" " and "CourseId"="'SWEN304'")$,
    ($times$, ($"Student"$), ($"Enrolled"$)),
  )))
})
// TODO calculate cost

===
#cetz.canvas(
  {
    import cetz.tree: tree
    import cetz.draw: set-style

    set-style(content: (padding: .2))

    tree(
      spread: 4,
      grow: 2,
      (
        $pi_("StudentId, Name, Grade")$,
        (
          $times$,
          ($"Student"$),
          ($sigma_("term"=2024" " and "CourseId"="'SWEN304'")$, ($"Enrolled"$),),
        ),
      ),
    )
  },
)// TODO calculate cost

===
// TODO Which of the above two trees has a smaller query cost and why?

#set heading(numbering: "1.a.1.")
= PostgreSQL and Query Optimization

```sql
select * from users;
```

`MOV`
"#,
    );
    // dbg!(&source.root());
    let _paragraphs = preprocess(&source);
    dbg!(&_paragraphs);
}
