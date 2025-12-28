# Rust Adoption Plan for Chromium

[TOC]

## Executive Summary

This document outlines a comprehensive plan for adopting Rust within the Chromium codebase and establishing a path towards gradually replacing C++ code with Rust while maintaining full compatibility. The plan is designed to be incremental, risk-averse, and focused on delivering value at each stage.

**Current State**: Chromium already has Rust infrastructure in place with ~4,850 Rust files compared to ~125,000 C++ files, representing approximately 3.7% of the codebase by file count.

**Goal**: Establish a sustainable, incremental path to increase Rust adoption across the codebase, focusing on areas where Rust provides the most value (memory safety, security, and performance) while maintaining full backward compatibility with existing C++ code.

## Vision and Goals

### Primary Goals

1. **Improve Security**: Reduce memory safety vulnerabilities by leveraging Rust's memory safety guarantees
2. **Maintain Compatibility**: Ensure seamless interoperability between Rust and C++ during the entire transition
3. **Incremental Adoption**: Enable gradual migration without requiring big-bang rewrites
4. **Developer Productivity**: Provide clear guidelines, tools, and training to maximize developer effectiveness
5. **Performance**: Match or exceed C++ performance in critical paths

### Non-Goals

- Complete rewrite of the entire codebase (this would be impractical and unnecessary)
- Breaking existing APIs or functionality
- Forcing adoption where C++ is sufficient and well-maintained

## Current State Analysis

### Existing Rust Infrastructure

Chromium already has comprehensive Rust support:

- **Build System**: Full GN integration with templates:
  - `rust_static_library.gni`
  - `rust_executable.gni`
  - `rust_shared_library.gni`
  - `cargo_crate.gni`
  - `rust_bindgen.gni`
  - `rs_bindings_from_cc.gni`

- **FFI Support**: 
  - `cxx` crate for C++/Rust interoperability
  - `bindgen` for generating Rust bindings from C headers
  - Crubit infrastructure (in development)

- **Testing**: 
  - `rust_gtest_interop` for integrating Rust tests with gtest
  - `rust_unit_test.gni` for Rust-specific unit tests

- **Third-party Dependencies**:
  - `//third_party/rust/chromium_crates_io/` for crates.io dependencies
  - Process for importing and updating crates
  - Tools for dependency management (`gnrt`)

### Current Rust Usage

Rust is currently used in the following areas:

1. **Media Processing**: `media/filters/symphonia_glue.rs`
2. **QR Code Generation**: `components/qr_code_generator/qr_code_generator_ffi_glue.rs`
3. **Data Import**: `components/user_data_importer/utility/parsing_ffi/`
4. **Payment Validation**: `components/facilitated_payments/core/validation/`
5. **Bluetooth**: `device/bluetooth/bluez/ble_scan_parser/`
6. **Testing Infrastructure**: `testing/rust_gtest_interop/`
7. **Build Tools**: `tools/crates/gnrt/`

### Gaps and Opportunities

**Gaps:**
- Limited Rust usage in core browser components
- No comprehensive migration guidelines for large C++ modules
- Limited Rust expertise across the team
- Incomplete coverage of Chromium APIs in Rust

**Opportunities:**
- Parser implementations (high security value)
- Protocol handlers (network data processing)
- Cryptographic operations
- Data validation and sanitization
- New feature development
- Isolated utility libraries

## Adoption Strategy

### Phase 1: Foundation (Months 1-6)

**Goal**: Strengthen infrastructure and team readiness

#### 1.1 Documentation and Guidelines

- [ ] **Create comprehensive migration guides**
  - Module assessment checklist
  - Step-by-step migration process
  - FFI boundary design patterns
  - Performance benchmarking guidelines
  
- [ ] **Establish Rust coding standards**
  - Extend existing style guide with Chromium-specific patterns
  - Error handling conventions
  - Logging and debugging patterns
  - Testing requirements

- [ ] **Document architecture patterns**
  - Component boundary design
  - State management patterns
  - Async/await integration with Chromium's threading model
  - Memory management across FFI boundaries

#### 1.2 Team Enablement

- [ ] **Training Program**
  - Basic Rust training for all interested developers
  - Advanced Rust training for core team members
  - FFI and interop workshops
  - Code review training for Rust

- [ ] **Champion Network**
  - Identify 10-15 Rust champions across teams
  - Provide advanced training and support
  - Establish regular sync meetings

- [ ] **Mentorship Program**
  - Pair Rust champions with teams adopting Rust
  - Code review support for Rust CLs
  - Office hours for Rust questions

#### 1.3 Infrastructure Improvements

- [ ] **Enhanced Build System**
  - Improve build caching for Rust
  - Optimize incremental compilation
  - Better error messages for FFI issues
  - Cross-compilation support validation

- [ ] **Tooling Enhancements**
  - IDE integration improvements (VSCode, CLion)
  - Debugging experience improvements
  - Performance profiling tools
  - Code coverage integration

- [ ] **CI/CD Integration**
  - Rust-specific linters in presubmit
  - Security scanning for Rust code
  - Performance regression detection
  - Automated dependency updates

### Phase 2: Targeted Adoption (Months 7-18)

**Goal**: Prove value through strategic component migrations

#### 2.1 New Feature Development

**Policy**: All new features should evaluate Rust as the default choice, falling back to C++ only when necessary.

**Decision Criteria**:
- Does the feature handle untrusted data? → Prefer Rust
- Is it a new, isolated component? → Prefer Rust
- Does it require complex memory management? → Prefer Rust
- Does it heavily interact with existing C++ code? → Consider C++
- Are there time constraints that would be impacted by learning curve? → Consider team readiness

#### 2.2 High-Value Migration Targets

Prioritize components based on:
1. **Security Impact**: Handles untrusted data, has history of vulnerabilities
2. **Isolation**: Can be migrated independently
3. **Team Readiness**: Team has Rust expertise or champions

**Tier 1 Candidates** (Months 7-12):
- [ ] **URL Parsing and Validation**
  - High security value
  - Well-defined boundaries
  - Existing Rust crates available

- [ ] **Media Codecs and Parsers**
  - Extend existing `symphonia_glue.rs` work
  - High vulnerability surface
  - Performance critical

- [ ] **Protocol Parsers** (HTTP headers, WebSocket, etc.)
  - Security-sensitive
  - Clear interfaces
  - Good test coverage available

- [ ] **Cryptographic Utilities**
  - Security-critical
  - Can leverage vetted Rust crates
  - Well-defined APIs

**Tier 2 Candidates** (Months 13-18):
- [ ] **JSON and XML Parsers**
  - Frequently handle untrusted input
  - Good Rust ecosystem support

- [ ] **Regular Expression Engine**
  - Security and performance benefits
  - Self-contained

- [ ] **Data Validation Libraries**
  - Input sanitization
  - Form validation
  - Content security

#### 2.3 Migration Process

For each migration candidate:

1. **Assessment Phase** (1-2 weeks)
   - Document current functionality and APIs
   - Identify dependencies and consumers
   - Define success criteria
   - Estimate effort and risk

2. **Design Phase** (2-3 weeks)
   - Design Rust architecture
   - Define FFI boundaries
   - Plan for compatibility layer
   - Review with stakeholders

3. **Implementation Phase** (varies by component)
   - Implement Rust core functionality
   - Create C++ compatibility wrapper
   - Implement comprehensive tests
   - Performance benchmarking

4. **Transition Phase** (2-4 weeks)
   - Gradual rollout with feature flags
   - Monitor performance and stability
   - Address issues and feedback
   - Full cutover when stable

5. **Cleanup Phase** (1-2 weeks)
   - Remove old C++ implementation
   - Update documentation
   - Knowledge transfer
   - Retrospective

### Phase 3: Scaled Adoption (Months 19-36)

**Goal**: Expand Rust adoption broadly across the codebase

#### 3.1 Component Categories

- [ ] **Networking Stack Components**
  - Protocol implementations
  - Request/response handling
  - Header parsing

- [ ] **Rendering Engine Utilities**
  - CSS parsing (evaluate)
  - Image decoding (expand existing work)
  - Font handling

- [ ] **Storage and Persistence**
  - Data serialization
  - Cache management
  - IndexedDB backend components

- [ ] **Platform Abstraction Layers**
  - File I/O utilities
  - Process management
  - IPC serialization

#### 3.2 Parallel C++ Improvement

As Rust adoption scales, continue investing in C++ safety:
- Expand use of safe C++ idioms
- Leverage Clang sanitizers
- Use modern C++ features (smart pointers, span, etc.)
- This ensures non-migrated code remains secure

### Phase 4: Maturity and Optimization (Months 37+)

**Goal**: Rust as a primary language alongside C++

#### 4.1 Advanced Integration

- [ ] **Shared Ownership Models**
  - Refine patterns for shared state between Rust and C++
  - Optimize memory management across boundaries

- [ ] **Performance Optimization**
  - Profile and optimize hot paths
  - Reduce FFI overhead
  - Leverage Rust's zero-cost abstractions

- [ ] **Enhanced Tooling**
  - Automated migration assistance tools
  - Better debugging across language boundaries
  - Integrated profiling

#### 4.2 Ongoing Activities

- Regular assessment of migration progress
- Continuous training and skill development
- Ecosystem monitoring (new crates, Rust language features)
- Documentation updates and refinement

## Technical Guidelines

### FFI Boundaries

#### Design Principles

1. **Minimize FFI Crossings**: Design coarse-grained APIs to reduce overhead
2. **Data Ownership Clarity**: Clearly document who owns memory at boundaries
3. **Error Handling**: Use consistent error propagation patterns
4. **Type Safety**: Leverage `cxx` crate's type-safe bindings

#### Recommended Patterns

**Using cxx crate** (Preferred):
```rust
#[cxx::bridge(namespace = "chromium::component")]
mod ffi {
    extern "C++" {
        // C++ types and functions used from Rust
    }
    
    extern "Rust" {
        // Rust functions exposed to C++
        fn process_data(input: &[u8]) -> Result<Vec<u8>>;
    }
}
```

**Using bindgen** (For C APIs):
```rust
// Generated bindings for existing C APIs
// Use when interfacing with legacy code or C libraries
```

### Memory Management

1. **Rust Owns Data**: Prefer Rust ownership, expose through FFI
2. **Refcounted Shared State**: Use `Arc` in Rust, `scoped_refptr` in C++
3. **Callbacks**: Use `Box<dyn Fn>` for Rust callbacks, function pointers for C++
4. **Buffers**: Use slices (`&[u8]`) for immutable data, vectors for owned data

### Error Handling

1. **Rust Side**: Use `Result<T, E>` for all fallible operations
2. **FFI Boundary**: Convert to status codes or error enums
3. **C++ Side**: Translate to appropriate C++ error handling (status, exceptions where allowed)

### Threading Model

1. **Task Posting**: Integrate with Chromium's task scheduler
2. **Thread Safety**: Use Rust's `Send` and `Sync` traits
3. **Blocking Operations**: Clearly document and handle appropriately
4. **Async/Await**: Consider integration with Chromium's asynchronous patterns

### Testing Strategy

1. **Unit Tests**: Rust-native tests using `#[test]` and `rust_unit_test.gni`
2. **Integration Tests**: Use `rust_gtest_interop` for testing across FFI
3. **Performance Tests**: Benchmark critical paths, compare with C++ baseline
4. **Fuzzing**: Integrate with ClusterFuzz and libFuzzer

### Security Considerations

1. **Unsafe Code**: Minimize use, require explicit approval
2. **Dependency Vetting**: Follow existing third-party review process
3. **Vulnerability Handling**: Monitor Rust Security Advisory Database
4. **Sandboxing**: Follow Rule of 2 guidelines
5. **Memory Safety**: Leverage Rust's guarantees, but verify FFI boundaries

## Success Metrics

### Quantitative Metrics

1. **Adoption Metrics**
   - Percentage of codebase in Rust (by lines, files, components)
   - Number of components migrated to Rust
   - Number of new features implemented in Rust

2. **Security Metrics**
   - Reduction in memory safety vulnerabilities
   - Number of security bugs in Rust vs. C++ code
   - Time to fix security issues

3. **Performance Metrics**
   - Build time impact
   - Runtime performance (compare Rust vs. C++ implementations)
   - Binary size impact

4. **Developer Productivity**
   - Time to implement features in Rust vs. C++
   - Code review turnaround time
   - Bug fix time

### Qualitative Metrics

1. **Developer Satisfaction**
   - Survey feedback on Rust development experience
   - Perceived code quality and maintainability

2. **Code Quality**
   - Maintainability assessments
   - Technical debt reduction

3. **Community Engagement**
   - Participation in Rust discussions
   - Contributions to Rust tooling and documentation

### Milestones

- **M1 (Month 6)**: Infrastructure complete, 5+ developers trained, first migration planned
- **M2 (Month 12)**: 2-3 components migrated, 10+ developers proficient, positive security impact
- **M3 (Month 18)**: 5-8 components migrated, Rust default for new security-sensitive features
- **M4 (Month 24)**: 10-15 components migrated, 25+ developers proficient, measurable security improvement
- **M5 (Month 36)**: 20+ components migrated, Rust as co-primary language with C++

## Risk Management

### Technical Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| FFI overhead degrades performance | High | Benchmark early, design coarse-grained APIs, optimize hot paths |
| Build time increases significantly | Medium | Invest in caching, incremental compilation, parallel builds |
| Binary size bloat | Medium | Monitor size, use link-time optimization, selective feature inclusion |
| Integration complexity with existing C++ | High | Strong FFI guidelines, `cxx` crate usage, thorough testing |
| Rust toolchain instability | Low | Use stable Rust, limit unstable features, monitor releases |

### Organizational Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Insufficient Rust expertise | High | Comprehensive training program, external experts, champion network |
| Resistance to adoption | Medium | Clear value demonstration, voluntary early adoption, success stories |
| Split focus between two languages | Medium | Clear migration strategy, tooling parity, unified code review |
| Knowledge silos | Medium | Documentation, mentorship, code review guidelines |
| Inconsistent adoption across teams | Medium | Clear guidelines, incentives, success metrics |

### Migration Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Breaking changes during migration | High | Feature flags, gradual rollout, comprehensive testing |
| Regression in functionality | High | Extensive test coverage, parallel running, careful validation |
| Schedule delays | Medium | Conservative estimates, phased approach, early prototyping |
| Incomplete migrations | Medium | Clear ownership, milestone tracking, management support |

## Resource Requirements

### Team Composition

**Phase 1** (Foundation):
- 1-2 Full-time Rust infrastructure engineers
- 3-5 Part-time champions (20% time)
- External training resources

**Phase 2** (Targeted Adoption):
- 2-3 Full-time migration engineers
- 5-10 Part-time contributors (20-50% time)
- 1 Technical writer (documentation)

**Phase 3** (Scaled Adoption):
- 5-8 Full-time engineers working on Rust components
- 15-20 Part-time contributors
- Ongoing training and support resources

### Infrastructure

- CI/CD capacity for Rust builds
- Enhanced tooling (IDE, debugging, profiling)
- Documentation platform updates
- Training materials and courses

## Governance

### Decision Making

1. **Language Choice for New Features**
   - Team decision with Rust as default for security-sensitive code
   - Must document rationale if choosing C++

2. **Migration Prioritization**
   - Quarterly planning by Rust adoption working group
   - Input from security team, component owners, and stakeholders

3. **Standards and Guidelines**
   - Rust style guide maintained by Rust working group
   - FFI patterns reviewed and approved by architecture team

### Working Group

Establish a **Rust Adoption Working Group**:
- Representatives from major components
- Build and infrastructure team members
- Security team representative
- Developer experience representative

**Responsibilities**:
- Maintain adoption roadmap
- Review migration proposals
- Update guidelines and best practices
- Track metrics and report progress
- Provide support to teams

### Review Process

1. **New Rust Code**: Follow existing OWNERS review process
2. **Migration CLs**: Require review from both component OWNERS and Rust expert
3. **FFI Changes**: Require review from Rust working group member
4. **Third-party Crates**: Follow existing third-party review process with Rust-specific checklist

## Communication Plan

### Regular Updates

- **Monthly**: Rust adoption metrics dashboard
- **Quarterly**: Working group report to leadership
- **Bi-annual**: All-hands presentation on progress and wins

### Documentation

- Maintain up-to-date documentation in `//docs/rust/`
- Migration playbooks and case studies
- FAQ and troubleshooting guides
- Regular blog posts on successful migrations

### Community Engagement

- Internal Slack channel: `#rust`
- Mailing list: `rust-dev@chromium.org`
- Office hours for Q&A
- Rust brown bag sessions
- Code review guidelines and examples

## Appendices

### A. Migration Checklist Template

When migrating a component to Rust, use this checklist:

**Assessment**
- [ ] Document current functionality and APIs
- [ ] Identify all dependencies
- [ ] Identify all consumers
- [ ] List existing tests
- [ ] Assess security impact
- [ ] Estimate LOC and complexity

**Planning**
- [ ] Design Rust architecture
- [ ] Define FFI boundaries
- [ ] Plan compatibility layer
- [ ] Design test strategy
- [ ] Define success criteria
- [ ] Get stakeholder approval

**Implementation**
- [ ] Implement Rust core
- [ ] Create C++ wrapper
- [ ] Port all tests
- [ ] Add integration tests
- [ ] Document new APIs
- [ ] Benchmark performance

**Validation**
- [ ] Code review
- [ ] Security review
- [ ] Performance validation
- [ ] Integration testing
- [ ] Gradual rollout plan

**Transition**
- [ ] Feature flag implementation
- [ ] Staged rollout
- [ ] Monitor metrics
- [ ] Address issues
- [ ] Full cutover

**Cleanup**
- [ ] Remove old C++ code
- [ ] Update documentation
- [ ] Knowledge transfer
- [ ] Retrospective

### B. Recommended Crates

For common use cases, prefer these well-vetted crates:

**Core Utilities**
- `serde` - Serialization/deserialization
- `log` - Logging (integrated with Chromium)
- `thiserror` - Error handling
- `anyhow` - Error context

**Data Structures**
- `smallvec` - Stack-allocated vectors
- `bytes` - Efficient byte buffers
- `indexmap` - Ordered maps

**Parsing**
- `nom` - Parser combinators
- `serde_json` - JSON parsing
- `quick-xml` - XML parsing
- `url` - URL parsing

**Crypto**
- `ring` - Cryptographic operations
- `rustls` - TLS implementation
- `sha2`, `blake3` - Hashing

**Async**
- Consider integration strategy before adopting async runtime

### C. Reference Materials

**Internal Documentation**
- [Rust in Chromium](../rust.md)
- [Rust FFI Guide](../rust/ffi.md)
- [Rust Style Guide](../styleguide/rust/rust.md)
- [Adding Third-party Crates](../third_party/rust/README-importing-new-crates.md)

**External Resources**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Comprehensive Rust (Chromium Day)](https://google.github.io/comprehensive-rust/chromium/)
- [cxx.rs Documentation](https://cxx.rs/)

### D. FAQ

**Q: Do we need to rewrite everything in Rust?**
A: No. We focus on high-value targets where Rust provides security or performance benefits. Many C++ components will remain.

**Q: What about performance?**
A: Rust performance is generally comparable to C++. We benchmark all migrations and optimize as needed.

**Q: How long will complete adoption take?**
A: This is an ongoing process without a fixed end date. We aim for Rust as a co-primary language within 3-5 years.

**Q: What if I don't know Rust?**
A: Training will be provided. Rust is not required for all developers, but we encourage learning.

**Q: Will Rust increase build times?**
A: Initially yes, but we're investing in caching and incremental compilation to minimize impact.

**Q: Can I use unstable Rust features?**
A: Only with approval from the Rust toolchain team. See [unstable feature usage guidelines](../tools/rust/unstable_rust_feature_usage.md).

---

## Conclusion

This Rust adoption plan provides a structured, incremental approach to introducing Rust into the Chromium codebase while maintaining full compatibility with existing C++ code. By focusing on high-value targets, investing in infrastructure and training, and maintaining a pragmatic approach, we can realize the benefits of Rust's memory safety and performance while managing risk and ensuring a smooth transition.

The success of this plan depends on:
- Strong leadership support
- Investment in training and tooling
- Clear communication and documentation
- Patience and pragmatism in execution
- Continuous learning and adaptation

With these elements in place, we can build a more secure, maintainable, and performant codebase that leverages the best of both Rust and C++.
