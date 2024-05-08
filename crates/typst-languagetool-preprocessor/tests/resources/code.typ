#import "@preview/cetz:0.2.2"

#let gap = {
  linebreak()
  linebreak()
}

#cetz.canvas({
  import cetz.tree: tree
  import cetz.draw: set-style
  
  set-style(content: (padding: .2))
  
  tree(spread: 5, grow: 2, ([A], (
    [B],
    ([C], (([D]), ([E], ([F])),), (([G]), ([H], ([I])),),),
    (([J]), ([K], ([L])),),
  ),))
})
