#!/usr/bin/env python3
"""
JSON Schema Validation Script

This script validates all JSON schema files to ensure they conform to
JSON Schema Draft 2020-12 specification and are properly formatted.

Usage: python3 validate_schemas.py
"""

import json
import os
import sys
from pathlib import Path

# ANSI color codes
class Colors:
    RESET = '\033[0m'
    GREEN = '\033[32m'
    RED = '\033[31m'
    YELLOW = '\033[33m'
    BLUE = '\033[34m'
    CYAN = '\033[36m'

def log(message, color=Colors.RESET):
    print(f"{color}{message}{Colors.RESET}")

def validate_json_file(file_path):
    """Validate a single JSON schema file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            schema = json.load(f)
        
        # Check required JSON Schema fields
        required_fields = ['$schema', '$id', 'title', 'description']
        missing = [field for field in required_fields if field not in schema]
        
        if missing:
            return {
                'valid': False,
                'errors': [f"Missing required fields: {', '.join(missing)}"]
            }
        
        # Validate $schema version
        if '2020-12' not in schema['$schema']:
            return {
                'valid': False,
                'errors': ['Schema must use JSON Schema Draft 2020-12']
            }
        
        # Check $id format
        if not schema['$id'].startswith('https://neuronexus.app/schemas/'):
            return {
                'valid': False,
                'errors': ['Schema $id must start with https://neuronexus.app/schemas/']
            }
        
        return {'valid': True, 'errors': []}
        
    except json.JSONDecodeError as e:
        return {
            'valid': False,
            'errors': [f"JSON parse error: {str(e)}"]
        }
    except Exception as e:
        return {
            'valid': False,
            'errors': [f"Error: {str(e)}"]
        }

def find_schema_files(directory):
    """Find all .schema.json files recursively."""
    return list(Path(directory).rglob('*.schema.json'))

def main():
    log('\n╔════════════════════════════════════════════════════════════╗', Colors.CYAN)
    log('║         NeuroNexus JSON Schema Validator                  ║', Colors.CYAN)
    log('║         JSON Schema Draft 2020-12                          ║', Colors.CYAN)
    log('╚════════════════════════════════════════════════════════════╝\n', Colors.CYAN)
    
    script_dir = Path(__file__).parent
    schemas_dir = script_dir / 'schemas'
    
    if not schemas_dir.exists():
        log('❌ Schemas directory not found!', Colors.RED)
        sys.exit(1)
    
    schema_files = find_schema_files(schemas_dir)
    
    if not schema_files:
        log('❌ No schema files found!', Colors.RED)
        sys.exit(1)
    
    log(f"Found {len(schema_files)} schema file(s) to validate\n", Colors.BLUE)
    
    valid_count = 0
    invalid_count = 0
    errors = []
    
    for file_path in sorted(schema_files):
        relative_path = file_path.relative_to(script_dir)
        print(f"Validating {relative_path}... ", end='')
        
        result = validate_json_file(file_path)
        
        if result['valid']:
            log('✓ PASS', Colors.GREEN)
            valid_count += 1
        else:
            log('✗ FAIL', Colors.RED)
            invalid_count += 1
            errors.append({'file': str(relative_path), 'errors': result['errors']})
    
    # Summary
    log('\n' + '─' * 60, Colors.CYAN)
    log('VALIDATION SUMMARY', Colors.CYAN)
    log('─' * 60, Colors.CYAN)
    log(f"Total schemas: {len(schema_files)}", Colors.BLUE)
    log(f"✓ Valid: {valid_count}", Colors.GREEN)
    
    if invalid_count > 0:
        log(f"✗ Invalid: {invalid_count}", Colors.RED)
        
        log('\nERRORS:', Colors.RED)
        for error_info in errors:
            log(f"\n{error_info['file']}:", Colors.YELLOW)
            for error in error_info['errors']:
                log(f"  - {error}", Colors.RED)
        
        log('\n❌ Validation failed!\n', Colors.RED)
        sys.exit(1)
    else:
        log('\n✅ All schemas are valid!\n', Colors.GREEN)
        sys.exit(0)

if __name__ == '__main__':
    main()
