#!/usr/bin/env python3
"""
GN to Cargo.toml Translator

Phase 1.2: Tooling Development
This tool parses BUILD.gn files and generates Cargo.toml manifests.

Usage:
    python3 gn_to_cargo.py <path/to/BUILD.gn> [--output <path/to/Cargo.toml>]
"""

import argparse
import json
import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Optional, Set


class GNParser:
    """Parser for BUILD.gn files."""
    
    def __init__(self, gn_file: Path):
        self.gn_file = gn_file
        self.content = gn_file.read_text()
        
    def parse_rust_static_library(self, target_name: str) -> Optional[Dict]:
        """Parse a rust_static_library target."""
        # Simple regex-based parsing (production version would use proper GN parser)
        pattern = rf'rust_static_library\("{target_name}"\)\s*\{{([^}}]+)\}}'
        match = re.search(pattern, self.content, re.DOTALL)
        
        if not match:
            return None
            
        target_content = match.group(1)
        
        return {
            'name': target_name,
            'type': 'rust_static_library',
            'crate_root': self._extract_field(target_content, 'crate_root'),
            'sources': self._extract_list(target_content, 'sources'),
            'deps': self._extract_list(target_content, 'deps'),
            'features': self._extract_list(target_content, 'features'),
            'allow_unsafe': self._extract_bool(target_content, 'allow_unsafe'),
        }
    
    def parse_cargo_crate(self, target_name: str) -> Optional[Dict]:
        """Parse a cargo_crate target."""
        pattern = rf'cargo_crate\("{target_name}"\)\s*\{{([^}}]+)\}}'
        match = re.search(pattern, self.content, re.DOTALL)
        
        if not match:
            return None
            
        target_content = match.group(1)
        
        return {
            'name': target_name,
            'type': 'cargo_crate',
            'crate_name': self._extract_field(target_content, 'crate_name'),
            'version': self._extract_field(target_content, 'version'),
            'features': self._extract_list(target_content, 'features'),
        }
    
    def find_all_targets(self) -> List[str]:
        """Find all Rust-related targets in the BUILD.gn file."""
        targets = []
        
        # Find rust_static_library targets
        for match in re.finditer(r'rust_static_library\("([^"]+)"\)', self.content):
            targets.append(match.group(1))
            
        # Find cargo_crate targets
        for match in re.finditer(r'cargo_crate\("([^"]+)"\)', self.content):
            targets.append(match.group(1))
            
        return targets
    
    def _extract_field(self, content: str, field: str) -> Optional[str]:
        """Extract a single field value."""
        pattern = rf'{field}\s*=\s*"([^"]*)"'
        match = re.search(pattern, content)
        return match.group(1) if match else None
    
    def _extract_list(self, content: str, field: str) -> List[str]:
        """Extract a list field value."""
        pattern = rf'{field}\s*=\s*\[([^\]]*)\]'
        match = re.search(pattern, content, re.DOTALL)
        
        if not match:
            return []
            
        list_content = match.group(1)
        # Extract quoted strings
        items = re.findall(r'"([^"]*)"', list_content)
        return items
    
    def _extract_bool(self, content: str, field: str) -> bool:
        """Extract a boolean field value."""
        pattern = rf'{field}\s*=\s*(true|false)'
        match = re.search(pattern, content)
        return match.group(1) == 'true' if match else False


class CargoGenerator:
    """Generator for Cargo.toml files."""
    
    def __init__(self, workspace_root: Path):
        self.workspace_root = workspace_root
        
    def generate_from_gn(self, gn_data: Dict, output_path: Optional[Path] = None) -> str:
        """Generate Cargo.toml from parsed GN data."""
        if gn_data['type'] == 'rust_static_library':
            return self._generate_static_library(gn_data, output_path)
        elif gn_data['type'] == 'cargo_crate':
            return self._generate_cargo_crate(gn_data, output_path)
        else:
            raise ValueError(f"Unknown target type: {gn_data['type']}")
    
    def _generate_static_library(self, data: Dict, output_path: Optional[Path]) -> str:
        """Generate Cargo.toml for a rust_static_library target."""
        # Convert GN target name to Cargo package name
        package_name = self._gn_to_cargo_name(data['name'])
        
        # Determine crate type
        crate_types = ['staticlib', 'rlib']
        
        # Convert GN deps to Cargo dependencies
        dependencies = self._convert_dependencies(data.get('deps', []))
        
        cargo_toml = f"""# Generated from BUILD.gn by gn_to_cargo.py
# Phase 1.2: Tooling Development

[package]
name = "{package_name}"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
"""
        
        # Add dependencies
        for dep_name, dep_spec in dependencies.items():
            if isinstance(dep_spec, str):
                cargo_toml += f'{dep_name} = "{dep_spec}"\n'
            else:
                cargo_toml += f'{dep_name} = {{ workspace = true }}\n'
        
        # Add lib section
        cargo_toml += f"""
[lib]
name = "{data['name'].replace('-', '_')}"
"""
        
        if data.get('crate_root'):
            cargo_toml += f'path = "{data["crate_root"]}"\n'
        
        cargo_toml += f'crate-type = {json.dumps(crate_types)}\n'
        
        # Add build dependencies if needed (for cxx integration)
        if self._needs_cxx(data):
            cargo_toml += """
[build-dependencies]
cxx-build = { workspace = true }
"""
        
        return cargo_toml
    
    def _generate_cargo_crate(self, data: Dict, output_path: Optional[Path]) -> str:
        """Generate Cargo.toml for a cargo_crate target (third-party)."""
        # Third-party crates are typically handled differently
        # This is mainly for documentation purposes
        return f"""# Third-party crate: {data['crate_name']}
# Managed via third_party/rust/chromium_crates_io/Cargo.toml
# See docs/cargo_adoption_plan.md for details
"""
    
    def _gn_to_cargo_name(self, gn_name: str) -> str:
        """Convert GN target name to Cargo package name."""
        # Replace : with - and ensure valid Cargo name
        name = gn_name.replace(':', '-').replace('/', '-')
        # Add chromium prefix to avoid conflicts
        if not name.startswith('chromium-'):
            name = f'chromium-{name}'
        return name
    
    def _convert_dependencies(self, gn_deps: List[str]) -> Dict[str, any]:
        """Convert GN dependencies to Cargo dependencies."""
        cargo_deps = {}
        
        for dep in gn_deps:
            # Parse GN dependency path
            if dep.startswith('//third_party/rust/'):
                # Third-party Rust crate
                crate_name = self._extract_crate_name(dep)
                cargo_deps[crate_name] = {'workspace': True}
            elif dep.startswith('//'):
                # First-party dependency
                dep_name = self._gn_to_cargo_name(dep.split(':')[-1])
                cargo_deps[dep_name] = {'path': self._compute_relative_path(dep)}
            
        return cargo_deps
    
    def _extract_crate_name(self, gn_path: str) -> str:
        """Extract crate name from GN third_party path."""
        # //third_party/rust/serde/v1:lib -> serde
        parts = gn_path.split('/')
        if 'rust' in parts:
            rust_idx = parts.index('rust')
            if rust_idx + 1 < len(parts):
                return parts[rust_idx + 1]
        return 'unknown'
    
    def _compute_relative_path(self, gn_path: str) -> str:
        """Compute relative path for local dependency."""
        # Simplified - would need proper path resolution
        return gn_path.replace('//', '').replace(':', '/')
    
    def _needs_cxx(self, data: Dict) -> bool:
        """Check if the crate needs cxx build dependencies."""
        # Check sources for .rs files that might use cxx
        sources = data.get('sources', [])
        return any('ffi' in src or 'bridge' in src for src in sources)


def main():
    parser = argparse.ArgumentParser(
        description='Convert BUILD.gn to Cargo.toml',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    parser.add_argument('gn_file', type=Path, help='Path to BUILD.gn file')
    parser.add_argument('--output', '-o', type=Path, help='Output Cargo.toml path')
    parser.add_argument('--target', '-t', help='Specific target to convert')
    parser.add_argument('--list', '-l', action='store_true', help='List all targets')
    parser.add_argument('--workspace', '-w', type=Path, default=Path.cwd(),
                       help='Workspace root directory')
    
    args = parser.parse_args()
    
    if not args.gn_file.exists():
        print(f"Error: {args.gn_file} does not exist", file=sys.stderr)
        return 1
    
    gn_parser = GNParser(args.gn_file)
    
    # List targets if requested
    if args.list:
        targets = gn_parser.find_all_targets()
        print(f"Found {len(targets)} Rust targets in {args.gn_file}:")
        for target in targets:
            print(f"  - {target}")
        return 0
    
    # Parse specific target or first found
    target_name = args.target
    if not target_name:
        targets = gn_parser.find_all_targets()
        if not targets:
            print(f"Error: No Rust targets found in {args.gn_file}", file=sys.stderr)
            return 1
        target_name = targets[0]
        print(f"Using first target: {target_name}")
    
    # Try parsing as rust_static_library first
    gn_data = gn_parser.parse_rust_static_library(target_name)
    if not gn_data:
        # Try as cargo_crate
        gn_data = gn_parser.parse_cargo_crate(target_name)
    
    if not gn_data:
        print(f"Error: Could not parse target '{target_name}'", file=sys.stderr)
        return 1
    
    # Generate Cargo.toml
    generator = CargoGenerator(args.workspace)
    cargo_toml = generator.generate_from_gn(gn_data, args.output)
    
    # Write or print
    if args.output:
        args.output.write_text(cargo_toml)
        print(f"Generated {args.output}")
    else:
        print(cargo_toml)
    
    return 0


if __name__ == '__main__':
    sys.exit(main())
