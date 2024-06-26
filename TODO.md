# TODO

## Layout node system

- [ ] removing ui elements + children by name or id
  - [ ] or 'handle'? enum
- [ ] use default padding + child gap everywhere
- [x] an 'update' callback callback for ui elements
- [ ] "layers" - i.e a stack of root nodes instead of just 1
  - [ ] higher layer = rendered on top & responds to click first
  - [ ] attempt this again with a different strategy - nodes have 'types': 'Default' or 'Layer'. Layer acts as a layer and has the same dimensions as its parent
- [ ] resizability should be configurable on a per-axis basis (right now it's for both)
- [x] interactive layout_nodes
  - [x] hover states
  - [x] click state
  - [x] button 'actions' described by enum - e.g "SelectRoomDefinition(id: string)", "RemoveUIElement(id: string)", "AddUIElement(some other enum)"
- [x] remove nodes
- [x] stretch to fill layout_nodes
  - [x] take primary/off axis into account (e.g off axis will not have siblings)
  - [x] support multiple stretch to fill siblings
  - [x] recurse down as well (or does it already do this?)
- [ ] Fix the ordering of the input/ui update/interactivity/prerendering, it is not obvious and hard to follow right now
- [ ] middleware-like prerender/precalculate "modifiers" to override prerendered values in a clean/abstracted way (like randomizing background/text colors)
- [ ] TreeNodeInput should accept a parent_id parameter too - it will just be ignored for the "children" TreeNodeInputs
- [ ] "validate layout" or something, that identifies/warns when children_size is greater than content_size etc
- [ ] rename 'prerender' to 'precalculate'
- [x] rename 'element_tree' to 'elements'
- [ ] update algorithms to calculate widths of all children at once
- [ ] abstract layout calculations from actual layout node system - it's just a bunch of math
  - [ ] basic tests
- [ ] adding/removing a layout node should trigger a rerender
  - make Elements.tree private? this would be the only way to really ensure this happens
- [ ] I could probably iterate through things once instead of twice, once for each axis
- [ ] text color config
- [ ] UI theme
- [ ] "space between" gap style
  - child_gap will need to be an enum, "Fixed" and "SpaceBetween"
  - actually this sounds a lot like expand_to_fill with all nodes having a weight of 1
- [ ] UI components
  - [ ] buttons
    - (This will probably be the done when I do the "interactive layout_nodes" thing ^^^)
  - [ ] text inputs
  - [ ] radios

## Cleanup

- [ ] move measurements mod into types mod

## Route finding

## Other

- translate pixel ratio better? (things look small on my monitor)
- keycode=>command mapping system
- room/cursor position is still wrong.
- 'active' state for buttons
- info text when you hover over room buttons
- flexible Dimensions for buttons
- UI themes
- different colors of sky for different times of day
- different color grading/shading for different times of day
- water pipes
- electricity
- bathrooms
- heatmaps
- car parking (above and below ground)
  - some office occupants need it and others dont
- monorail (2nd + floors)
- underground rail
- rename 'tower' to 'current_tower'
- status text goes away after timer
- status text should have a max width and wrap lines
- notification text when you get office income
- if no transportation is connected to room, complain via notification and tenants should not move in/existing tenants should move out
- Floor room type, that
  - is placeable like a normal room
  - connects rooms on the same floor when they arent placed next to each other
  - replaces a room when it is deleted
- destroyable rooms
- basic saving/loading UI
- tests that use fixture data
  - e.g for route-finding algorithm
  - derive Debug on Game & print out game -> copy + paste to create fixture data. Not json data
- 'resizable after placement' resizable rooms (elevators, but maybe lobby??)
- restaurants
- Different colors for whether a room is occupied or not
- Randomization within range for moving in/checking in/out, so it doesn't all happen at once
- colors for night/day
- weather
- Room evaluation
  - affected by proximity to elevators, noisy room types, etc
  - improvements over time
- fix this tower.tower pattern with TowerSlice
- line-height for text
- Query builder type for rooms in tower
- Room will probably need to be a trait to enable custom behavior/data for specific room types
- room selection Buttons
- Tools
  - build, destroy, inspect
- build tool panel
- room destroying
  - seperate build/destroy validation
- Room "layers" (just basic + transportation, for now)
- Tools
- Camera point offset to enable lerping between cells + smooth scanning
- Basic SFX
- Esc menu
- Store rooms by floor as well, for performance reasons
  - I might have to start using Rc+RefCell for this
  - Don't do this until it makes sense to start optimizing

# Done

- layout node system
  - get rid of Elements and make TreeNodes store a generic Data type
  - rename 'layout\*tree' to 'elements' or 'ui_elements'
  - create 'LayoutNode\_\_?' struct to encapsulate layout node list/tree stuff, but not coupled to slice
  - slice could just be a thin wrapper around this
  - account for 'flow direction' when calculating position on axis
  - 'calculate' layout node functions maybe shouldn't live on layout\_)node
    - create new calculated dimensions property: children_dimensions. Total children size is not going to always be the same as content_size
  - center content on non primary axis too
- add 'color' to calculated layout node properties for now to stop the crazy flashing
- button 'row's
- abstract button row's dimensions/positioning into a 1D fn.
- after I get layout node working, refactor Wrapper/LeafNode to just be the same type
- Clickable buttons (ui state)
- deprecate stuff called "\_\_Slice"
- seperate game into 'world' and 'ui'
  - world = time, towers, wallet
  - engine = "timers"
  - ui
- rooms should not get occupants unless they are connected to transportation
- timer ids should be enums not strings
- Timers + listeners should use enums instead of strings for ids
- Transportation, pathfinding
- json serialization/deserialization
- Room 'layers'
  - normal
  - transportation
- BUGFIX: room blueprint 'offset' doesn't get calculated when placing
- Occupants
  - arrive between 5-8 in the evening, leave 7-10 in the morning
- center blueprint room to cursor
- Flexible rooms price should be per cell
- Time
- Occupants
  - that move into condo
  - that move into office
- Office rent money
- Condo sale money
- Room colors
- Don't reset the selected cell to 0,0 every time
- Resizable rooms
- Make blueprint room centered on cursor, not below/to the right
- Orient 0,0 to the middle of the screen (not top left)
- Floor gfx
- Basic Camera
- base set of validators for all room definitions
