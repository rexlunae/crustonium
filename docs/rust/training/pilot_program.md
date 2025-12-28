# Pilot Program: Cargo Migration

**Phase 1.3: Documentation and Training**

Guide for selecting and migrating pilot components to Cargo.

[TOC]

## Overview

The pilot program validates the Cargo migration approach with 3-5 small components before broader rollout. This document guides component selection, migration, and lessons learned.

## Pilot Goals

### Primary Objectives

1. **Validate tooling**: Prove gn_to_cargo.py and hybrid build work
2. **Identify gaps**: Find missing features or documentation
3. **Measure impact**: Quantify build time and developer experience changes
4. **Build expertise**: Create team of migration champions
5. **Refine process**: Improve migration workflow based on real experience

### Success Criteria

- ✅ All pilot components build with Cargo
- ✅ All tests pass in both GN and Cargo builds
- ✅ Build times comparable or better
- ✅ No regressions in functionality
- ✅ Documentation updated based on findings
- ✅ Team confident in larger migrations

## Component Selection

### Ideal Pilot Characteristics

**Must Have**:
- Pure Rust or Rust-heavy component
- Well-defined boundaries
- Good test coverage
- Active development (to validate workflow)
- Small-medium size (< 10k LoC)

**Nice to Have**:
- Minimal C++ FFI (or good FFI examples)
- Few external dependencies
- Multiple contributors (tests workflow across team)
- Representative of future migrations

### Anti-Patterns (Avoid for Pilot)

❌ Massive components (> 50k LoC)  
❌ Complex C++ integration not yet prototyped  
❌ No tests  
❌ Deprecated code  
❌ Circular dependencies with many components  
❌ Critical path components (too risky for pilot)

### Recommended Pilot Components

Based on Chromium codebase analysis:

#### Tier 1: Ideal Pilots (Choose 2-3)

**1. QR Code Generator** (`components/qr_code_generator`)
- **Why**: Pure Rust, FFI well-defined, good tests
- **Size**: ~2k LoC
- **FFI**: Simple C interface
- **Dependencies**: Few (qr_code crate)
- **Risk**: Low

**2. Rust Test Infrastructure** (`testing/rust_gtest_interop`)
- **Why**: Testing-focused, self-contained
- **Size**: ~1k LoC
- **FFI**: Gtest integration
- **Dependencies**: Minimal
- **Risk**: Low

**3. Build Tools** (`tools/crates/gnrt`)
- **Why**: Developer tool, isolated from product
- **Size**: ~3k LoC
- **FFI**: None (pure Rust binary)
- **Dependencies**: Rust ecosystem only
- **Risk**: Very low

#### Tier 2: Good Pilots (Choose 1-2 if needed)

**4. Bluetooth Parser** (`device/bluetooth/bluez/ble_scan_parser`)
- **Why**: Well-defined parsing task
- **Size**: ~2k LoC
- **FFI**: Parsing interface
- **Dependencies**: Few
- **Risk**: Low-medium

**5. Payment Validation** (`components/facilitated_payments`)
- **Why**: Security-sensitive (validates migration benefits)
- **Size**: ~3k LoC  
- **FFI**: Validation interface
- **Dependencies**: Several
- **Risk**: Medium (security component)

## Migration Process

### Step 1: Preparation (Week 1)

**1.1 Document Current State**
```bash
# Build times
time ninja -C out/Default component_name

# Test execution
time out/Default/component_tests

# Dependencies
gn desc out/Default //path/to:target deps
```

**1.2 Review Code**
- Read source code
- Understand FFI boundaries
- Identify dependencies
- Check test coverage

**1.3 Set Up Tracking**
Create tracking doc:
- Component name
- Owner
- Start date
- Current metrics (build time, test count, LoC)
- Expected completion

### Step 2: Generate Cargo.toml (Week 1)

**2.1 Use gn_to_cargo.py**
```bash
python3 tools/cargo_migration/gn_to_cargo.py \
  path/to/BUILD.gn \
  --target component_name \
  --output path/to/Cargo.toml
```

**2.2 Review and Adjust**
```toml
# Check generated file
cat path/to/Cargo.toml

# Adjust as needed:
# - Verify dependencies
# - Add features if needed
# - Set correct crate type
# - Add metadata
```

**2.3 Add to Workspace**
```toml
# Edit root Cargo.toml
[workspace]
members = [
    "path/to/component",  # Add this
]
```

### Step 3: Test Build (Week 1-2)

**3.1 Initial Build**
```bash
# Try building
cargo build -p chromium-component-name

# Fix any errors
# Common issues:
# - Missing dependencies
# - Wrong paths
# - FFI configuration
```

**3.2 Validate Tests**
```bash
# Run Cargo tests
cargo test -p chromium-component-name

# Compare with GN tests
out/Default/component_tests
```

**3.3 Compare Outputs**
```bash
# Build with both systems
./tools/cargo_migration/hybrid_build.sh --system gn
./tools/cargo_migration/hybrid_build.sh --system cargo

# Verify identical functionality
# (not necessarily identical binaries)
```

### Step 4: Integration (Week 2)

**4.1 Hybrid Build Support**
```python
# Update BUILD.gn to support Cargo
rust_static_library("component_name") {
  enable_cargo_build = true
  cargo_toml = "Cargo.toml"
  
  # Keep existing GN build as fallback
  crate_root = "src/lib.rs"
  sources = [ "src/lib.rs" ]
}
```

**4.2 CI Integration**
```yaml
# Verify CI passes
# - Cargo builds
# - Tests pass
# - Security scans run
```

**4.3 Documentation**
```markdown
# Update component README
## Building

With Cargo:
    cargo build -p chromium-component-name

With GN (legacy):
    ninja -C out/Default component_name
```

### Step 5: Validation (Week 3)

**5.1 Performance Testing**
```bash
# Measure build times
time cargo build -p component  # Clean
time cargo build -p component  # Incremental

# Compare with GN
time ninja -C out/Default component
```

**5.2 Functionality Testing**
```bash
# Run full test suite
cargo test -p component --all-features

# Run integration tests
./run_integration_tests.sh
```

**5.3 Team Validation**
- Other developers try building
- Collect feedback
- Address concerns

### Step 6: Documentation (Week 4)

**6.1 Write Migration Report**
Template in next section.

**6.2 Update Documentation**
- Component README
- Build instructions
- Known issues
- Lessons learned

**6.3 Present to Team**
- Demo migration
- Share metrics
- Discuss challenges
- Answer questions

## Migration Report Template

```markdown
# Component Migration Report: [Component Name]

## Overview
- **Component**: components/component_name
- **Owner**: @username
- **Duration**: X days
- **Status**: ✅ Complete / ⚠️ Blocked / ❌ Failed

## Metrics

### Build Performance
| Metric | GN/Ninja | Cargo | Change |
|--------|----------|-------|--------|
| Clean build | 45s | 38s | -15% ✅ |
| Incremental | 5s | 4s | -20% ✅ |
| Binary size | 2.1 MB | 2.0 MB | -5% ✅ |

### Test Performance
| Metric | GN/Ninja | Cargo | Change |
|--------|----------|-------|--------|
| Test count | 127 | 127 | 0 |
| Test time | 12s | 11s | -8% ✅ |
| Coverage | 85% | 85% | 0 |

## Challenges Encountered

### 1. [Challenge Name]
**Issue**: Description of problem  
**Solution**: How we solved it  
**Time Lost**: X hours  
**Prevention**: How to avoid in future

### 2. [Challenge Name]
...

## Lessons Learned

### What Went Well
- ✅ gn_to_cargo.py generated 90% correct Cargo.toml
- ✅ Tests migrated easily
- ✅ Build times improved

### What Could Be Improved
- ⚠️ FFI configuration required manual adjustment
- ⚠️ Documentation was sparse
- ⚠️ Initial learning curve for team

### Recommendations for Future Migrations
1. Start with simpler components for team training
2. Budget extra time for FFI configuration
3. Test incrementally, don't wait until end

## Tooling Feedback

### gn_to_cargo.py
- **Pros**: Fast, mostly accurate
- **Cons**: Missed some edge cases
- **Suggestions**: Add support for [specific feature]

### hybrid_build.sh
- **Pros**: Easy to use, good error messages
- **Cons**: [any issues found]
- **Suggestions**: [improvements]

## Documentation Updates

- [ ] Updated component README
- [ ] Added Cargo build instructions
- [ ] Documented known issues
- [ ] Updated team wiki

## Next Steps

- [ ] Monitor for regressions
- [ ] Migrate next component: [name]
- [ ] Share learnings with team
- [ ] Update migration playbook

## Team Feedback

**Developer A**: "Build times are noticeably faster"  
**Developer B**: "Initial setup was confusing, but doc helped"  
**Developer C**: "Tests run faster with cargo nextest"

## Conclusion

Overall success. Migration took slightly longer than estimated but results are positive. Recommend continuing with next pilot.

---
**Report Date**: YYYY-MM-DD  
**Next Review**: YYYY-MM-DD
```

## Tracking Dashboard

### Pilot Components Status

| Component | Owner | Status | Start | End | Build Δ | Issues |
|-----------|-------|--------|-------|-----|---------|--------|
| QR Code | @dev1 | ✅ Done | 01-15 | 01-29 | +15% ✅ | 0 |
| Test Infra | @dev2 | 🚧 Active | 01-22 | - | - | 2 |
| Build Tools | @dev3 | 📋 Planned | - | - | - | 0 |
| BT Parser | - | ⏸️ Queued | - | - | - | 0 |
| Payment | - | ⏸️ Queued | - | - | - | 0 |

Legend:
- ✅ Done: Fully migrated and validated
- 🚧 Active: Migration in progress
- 📋 Planned: Scheduled, not started
- ⏸️ Queued: Pending capacity
- ❌ Blocked: Waiting on dependency

### Key Metrics

**Aggregate Results**:
- Components migrated: X / 5
- Avg build time change: -12%
- Avg test time change: -8%
- Total time invested: Y hours
- Issues found: Z
- Documentation pages created: N

## Champion Network

### Roles

**Migration Champions**: Developers who've completed a pilot migration
- Help others with migrations
- Provide code reviews
- Share best practices

**Tool Maintainers**: Keep migration tools updated
- Fix bugs in gn_to_cargo.py
- Improve hybrid_build.sh
- Update CI workflows

**Documentation Writers**: Maintain migration docs
- Update based on learnings
- Create new guides as needed
- Answer questions

### Building the Network

**After each pilot**:
1. Migrating developer becomes champion
2. Hosts knowledge share session
3. Adds learnings to documentation
4. Mentors next migrator

**Recognition**:
- List champions in README
- Shout-outs in team meetings
- Contribution to career development

## Post-Pilot Review

### Review Meeting Agenda

1. **Metrics Review** (15 min)
   - Build performance
   - Test performance
   - Developer satisfaction

2. **Challenges Discussion** (20 min)
   - Technical issues
   - Process issues
   - Tooling gaps

3. **Lessons Learned** (15 min)
   - What worked well
   - What needs improvement
   - Unexpected findings

4. **Go/No-Go Decision** (10 min)
   - Continue to Phase 2?
   - What needs fixing first?
   - Resource allocation

### Decision Criteria

**Proceed to Phase 2 if**:
- ✅ All pilots completed successfully
- ✅ Build times competitive or better
- ✅ No major functionality regressions
- ✅ Team feedback positive
- ✅ Tooling gaps identified and addressable

**Pause if**:
- ⚠️ Major blockers found
- ⚠️ Build times significantly worse
- ⚠️ Team strongly negative
- ⚠️ Tooling needs major overhaul

## Resources

- [Cargo Basics](cargo_basics.md)
- [Workspace Best Practices](workspace_best_practices.md)
- [Troubleshooting Guide](troubleshooting.md)
- [Cargo Adoption Plan](../../cargo_adoption_plan.md)

## Support

**Questions?**
- Slack: `#cargo-migration`
- Office Hours: Weekly (check calendar)
- Email: `build-dev@chromium.org`

**Tracking**:
- Pilot Dashboard: [link]
- Issue Tracker: [link]
- Meeting Notes: [link]

---

**Status**: Pilot program in Phase 1.3  
**Next Update**: [date]
