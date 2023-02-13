# rip

**rip** is aspiring to implement a subset of Python's `pip`


TODOs:
- [x] Resolve this: `lib.rs(65, 5): rustdoc does not generate documentation for macro invocations`
- [x] Implement version comparisons
    - [x] Add more tests for each case
- [ ] Refactor (the code is really ugly)
    - [ ] Replace `.ok_or` and `.unwrap` bullshit with pattern-matching
    - [ ] Find a way to do away with method chains
- [ ] Parse version constraints
- [ ] Create better handling for unconformant versions
- [ ] Develop strategy to parse multi-fields
- [ ] Parse remaining metadata
- [ ] ...